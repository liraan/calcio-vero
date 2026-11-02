//! The in-progress state of a match.
//!
//! `MatchState` is mutated by the tick loop. At the end of the match, it
//! produces a `MatchResult` via `finish`.

use pcfm_core::ids::{ActorId, EventId, PlayerId};
use pcfm_core::rng::StreamName;
use pcfm_core::time::GameInstant;
use pcfm_model::Squad;
use pcfm_stats::{EventChain, EventType, Half, MatchEvent};

use crate::context::MatchContext;
use crate::perception::distance_squared;
use crate::result::{MatchResult, MatchStatistics, Score};
use crate::spatial::SpatialState;

/// Which team has the ball.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Possession {
    Home,
    Away,
}

/// The in-progress state of a match.
pub struct MatchState {
    score: Score,
    possession: Possession,
    home_possession_ticks: u64,
    away_possession_ticks: u64,
    shots: (u16, u16),
    next_event_slot: u32,
    /// Which half is currently being played (ADR-026).
    half: Half,
    /// The absolute tick at which the current half started.
    half_start_tick: u64,
    /// The spatial state: positions, ball, carrier (slice B2).
    spatial: SpatialState,
    // Stream names are built once and reused. They are deterministic by
    // construction (literal non-empty strings) and do not change during a
    // match.
    actions_stream: StreamName,
    action_kind_stream: StreamName,
}

impl MatchState {
    /// Creates the initial state: kickoff, home possession.
    ///
    /// # Panics
    ///
    /// Panics if the literal stream names used internally are empty. They
    /// are compile-time constants (`"match:actions"`, `"match:action_kind"`,
    /// `"match:player_pick"`), so this panic is unreachable in practice.
    #[must_use]
    pub fn new(match_: &pcfm_model::Match) -> Self {
        Self {
            score: Score { home: 0, away: 0 },
            possession: Possession::Home,
            home_possession_ticks: 0,
            away_possession_ticks: 0,
            shots: (0, 0),
            next_event_slot: 0,
            half: Half::First,
            half_start_tick: 0,
            spatial: SpatialState::from_squads(&match_.pitch, &match_.home, &match_.away),
            actions_stream: StreamName::new(format!("match:{}:actions", match_.id.slot()))
                .expect("stream name is non-empty"),
            action_kind_stream: StreamName::new(format!("match:{}:action_kind", match_.id.slot()))
                .expect("stream name is non-empty"),
        }
    }

    /// Switches the current half and records the absolute tick at which it
    /// started (ADR-026).
    ///
    /// The timestamp of an event is relative to its half. The tick loop is
    /// continuous (the absolute tick keeps growing); this method is the
    /// only place the two are reconciled.
    pub const fn begin_half(&mut self, half: Half, start_tick: u64) {
        self.half = half;
        self.half_start_tick = start_tick;
    }
    /// Advances the state by one tick.
    ///
    /// # The prototype model
    ///
    /// Every tick, with probability `action_probability` (default `0.0005`),
    /// one of four actions happens. The split is a **placeholder**, not a
    /// design decision:
    ///
    /// | Roll range | Action | Why this split |
    /// |------------|--------|----------------|
    /// | `[0.00, 0.60)` | Pass | The most common action in football |
    /// | `[0.60, 0.85)` | Shot | Less common, higher impact |
    /// | `[0.85, 0.97)` | Duel | Medium frequency |
    /// | `[0.97, 1.00)` | Foul | Rare |
    ///
    /// All players are "midfielder" in the demo, and the formation is
    /// always `4-4-2`. A realistic engine would model role, formation,
    /// position on the pitch, and per-player action probabilities.
    ///
    /// **Who acts vs what they do (2026-11-02).** With the influence model,
    /// *who* acts is deterministic (max influence on the ball, §2.9). *What*
    /// they do is still this roll: an open design question (does each agent
    /// have an intent?). The per-agent slice must not assume the roll is the
    /// design; it is the current placeholder for intention.
    ///
    /// The `noise` term in `resolve_*` (`pcfm-hidden`) is also a
    /// placeholder: it adds variance to the outcome, but it is not
    /// calibrated to real football data. See `docs/CODE_TOUR.md` "A note
    /// on the current simulation".
    #[allow(clippy::too_many_lines)]
    pub fn tick(&mut self, tick: u64, ctx: &mut MatchContext<'_>) {
        match self.possession {
            Possession::Home => self.home_possession_ticks += 1,
            Possession::Away => self.away_possession_ticks += 1,
        }

        let action_roll = pcfm_core::rng::next_unit(ctx.rng.stream(&self.actions_stream));
        if action_roll >= ctx.config.action_probability {
            return;
        }

        let (attacker, defender) = self.pick_from_space(ctx);

        let action_kind_roll = pcfm_core::rng::next_unit(ctx.rng.stream(&self.action_kind_stream));

        // The action belongs to the nearest agent; the ball follows.
        self.spatial.set_carrier(Some(attacker));

        // Pressure beside the action (decision 2026-11-02, step toward the
        // outcome): the most influential opponent on the ball's point presses,
        // if it is within the space threshold. The rank is the order from
        // resolve_influence; no scalar enters the event.
        {
            let weights = crate::balance::InfluenceWeightsJson::load().to_weights();
            let infl_ctx = self.build_influence_ctx(
                ctx,
                self.spatial.ball(),
                pcfm_hidden::SkillKind::Tackling,
            );
            let order = ctx.hidden.resolve_influence(&infl_ctx, &weights);
            if let Some(&presser) = order.first() {
                if let Some(a) = self.spatial.agent(presser) {
                    if distance_squared(a.position, self.spatial.ball())
                        <= i64::from(weights.space_threshold_cm)
                            * i64::from(weights.space_threshold_cm)
                    {
                        self.emit_event(
                            ctx,
                            tick,
                            Some(ActorId::Player(presser)),
                            EventType::Pressure,
                            serde_json::json!({
                                "from": presser.slot(),
                                "to": attacker.slot(),
                            }),
                        );
                    }
                }
            }
        }

        if action_kind_roll < 0.6 {
            let ctx_pass = pcfm_hidden::PassContext { passer: attacker };
            let outcome = ctx.hidden.resolve_pass(&ctx_pass);
            if !outcome.complete {
                self.flip_possession();
                self.spatial.set_carrier(None);
            }
        } else if action_kind_roll < 0.85 {
            let ctx_shot = pcfm_hidden::ShotContext { shooter: attacker };
            let outcome = ctx.hidden.resolve_shot(&ctx_shot);
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(attacker)),
                EventType::Shot,
                serde_json::json!({ "goal": outcome.goal }),
            );
            match self.possession {
                Possession::Home => {
                    self.shots.0 += 1;
                }
                Possession::Away => {
                    self.shots.1 += 1;
                }
            }
            if outcome.goal {
                match self.possession {
                    Possession::Home => self.score.home += 1,
                    Possession::Away => self.score.away += 1,
                }
                self.emit_event(
                    ctx,
                    tick,
                    Some(ActorId::Player(attacker)),
                    EventType::Goal,
                    serde_json::json!({}),
                );
            }
            self.flip_possession();
        } else if action_kind_roll < 0.97 {
            let ctx_duel = pcfm_hidden::DuelContext {
                attacker,
                defender,
                duel_type: pcfm_hidden::DuelType::Ground,
            };
            let outcome = ctx.hidden.resolve_duel(&ctx_duel);
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(attacker)),
                EventType::Duel,
                serde_json::json!({ "winner_slot": outcome.winner.slot() }),
            );
            if outcome.winner == defender {
                self.flip_possession();
                self.spatial.set_carrier(Some(defender));
                self.emit_event(
                    ctx,
                    tick,
                    Some(ActorId::Player(defender)),
                    EventType::Tackle,
                    serde_json::json!({ "from": attacker.slot(), "to": defender.slot() }),
                );
            }
        } else {
            let ctx_duel = pcfm_hidden::DuelContext {
                attacker,
                defender,
                duel_type: pcfm_hidden::DuelType::Ground,
            };
            let _outcome = ctx.hidden.resolve_duel(&ctx_duel);
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(defender)),
                EventType::Foul,
                serde_json::json!({}),
            );
            self.flip_possession();
            self.spatial.set_carrier(None);
        }
    }

    /// Emits a match-boundary event (`KickOff`, `HalfTime`, `SecondHalf`,
    /// `FullTime`). Boundary events have `actor = None` (ADR-026 §4).
    pub fn emit_boundary(&mut self, ctx: &mut MatchContext<'_>, tick: u64, event_type: EventType) {
        self.emit_event(ctx, tick, None, event_type, serde_json::json!({}));
    }
    /// Produces the final result.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn finish(&self, chain: EventChain) -> MatchResult {
        let total = self.home_possession_ticks + self.away_possession_ticks;
        let possession = if total == 0 {
            (0.5, 0.5)
        } else {
            let home = (self.home_possession_ticks as f32) / (total as f32);
            (home, 1.0 - home)
        };

        let state_hash = chain.chain_hash();

        MatchResult {
            score: self.score,
            events: chain,
            stats: MatchStatistics {
                possession,
                shots: self.shots,
                goals: (self.score.home, self.score.away),
            },
            state_hash,
            narrative: Vec::new(),
        }
    }

    const fn flip_possession(&mut self) {
        self.possession = match self.possession {
            Possession::Home => Possession::Away,
            Possession::Away => Possession::Home,
        };
    }

    /// The attacker and defender of the next action, from space (slice B2).
    ///
    /// The attacker is the agent of the possessing team nearest to the
    /// ball; the defender is the nearest opponent to the attacker. Ties
    /// are broken by `PlayerId` (deterministic).
    ///
    /// Special case of influence (`03-entities-minimal` §2.9): the general
    /// case reads the latents inside `pcfm-hidden`; here only the space term
    /// is used. The per-agent slice replaces this with the full formula.
    fn pick_from_space(&self, ctx: &MatchContext<'_>) -> (PlayerId, PlayerId) {
        let (attacker_squad, defender_squad) = self.squads(ctx);
        let ball = self.spatial.ball();
        let attacker = nearest_in_squad(&self.spatial, attacker_squad, ball);
        let attacker_pos = self.spatial.agent(attacker).map_or(ball, |a| a.position);
        let defender = nearest_in_squad(&self.spatial, defender_squad, attacker_pos);
        (attacker, defender)
    }

    const fn squads<'a>(&self, ctx: &'a MatchContext<'_>) -> (&'a Squad, &'a Squad) {
        match self.possession {
            Possession::Home => (&ctx.match_.home, &ctx.match_.away),
            Possession::Away => (&ctx.match_.away, &ctx.match_.home),
        }
    }

    /// Builds the influence context for the defenders of the current
    /// possession (the opponents of the carrier), on `point`.
    ///
    /// `skill` is the latent used as modulator. The agents are the
    /// defenders' squad: pressure is a contest of the opponents on the
    /// carrier (03-entities-minimal §2.9, use 2).
    fn build_influence_ctx(
        &self,
        ctx: &MatchContext<'_>,
        point: (i32, i32),
        skill: pcfm_hidden::SkillKind,
    ) -> pcfm_hidden::InfluenceContext {
        let (_, defenders) = self.squads(ctx);
        let agents = defenders
            .players
            .iter()
            .filter_map(|p| {
                self.spatial
                    .agent(p.id)
                    .map(|a| pcfm_hidden::InfluenceAgent {
                        id: p.id,
                        position: a.position,
                        #[allow(clippy::cast_possible_truncation)]
                        fatigue_milli: (p.physical.fatigue * 1000.0) as i32,
                    })
            })
            .collect();
        pcfm_hidden::InfluenceContext {
            point,
            agents,
            skill,
        }
    }

    fn emit_event(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        actor: Option<ActorId>,
        event_type: EventType,
        fields: serde_json::Value,
    ) {
        let ticks_per_sec = u64::from(ctx.config.ticks_per_second).max(1);
        let total_secs = tick / ticks_per_sec;
        let half_secs = (tick.saturating_sub(self.half_start_tick)) / ticks_per_sec;
        let minute = clamp_u8(half_secs / 60, 120);
        let second = clamp_u8(half_secs % 60, 60);
        let minutes_played = clamp_u16(total_secs / 60);

        let event = MatchEvent {
            event_id: EventId::new(self.next_event_slot, 0),
            match_id: ctx.match_.id,
            actor,
            half: self.half,
            timestamp_minute: minute,
            timestamp_second: second,
            minutes_played_cumulative: minutes_played,
            event_type,
            fields,
            created_at: GameInstant::from_raw(tick),
        };
        self.next_event_slot += 1;
        let result = ctx.chain.try_append(event);
        debug_assert!(
            result.is_ok(),
            "engine produced an invalid event: {result:?}"
        );
    }
}

/// The player of `squad` nearest to `point`, tie-broken by `PlayerId`.
///
/// # Panics
///
/// Panics if the squad has no agent in `spatial`. `Squad` has 11 players
/// and `SpatialState` places all of them, so this is unreachable.
fn nearest_in_squad(spatial: &SpatialState, squad: &Squad, point: (i32, i32)) -> PlayerId {
    squad
        .players
        .iter()
        .filter_map(|p| {
            spatial
                .agent(p.id)
                .map(|a| (distance_squared(point, a.position), p.id))
        })
        .min_by(|x, y| x.0.cmp(&y.0).then(x.1.slot().cmp(&y.1.slot())))
        .map(|(_, id)| id)
        .expect("spatial state contains every starter")
}

/// Clamps a `u64` to a `u8`, with a hard upper bound of `max`.
fn clamp_u8(value: u64, max: u8) -> u8 {
    u8::try_from(value.min(u64::from(max))).unwrap_or(max)
}

/// Clamps a `u64` to a `u16`.
fn clamp_u16(value: u64) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}
