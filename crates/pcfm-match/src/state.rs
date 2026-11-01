//! The in-progress state of a match.
//!
//! `MatchState` is mutated by the tick loop. At the end of the match, it
//! produces a `MatchResult` via `finish`.

use pcfm_core::ids::{ActorId, EventId, PlayerId};
use pcfm_core::rng::StreamName;
use pcfm_core::time::GameInstant;
use pcfm_stats::{EventChain, EventType, MatchEvent};

use crate::context::MatchContext;
use crate::result::{MatchResult, MatchStatistics, Score};

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
    // Stream names are built once and reused. They are deterministic by
    // construction (literal non-empty strings) and do not change during a
    // match.
    actions_stream: StreamName,
    action_kind_stream: StreamName,
    player_pick_stream: StreamName,
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
    pub fn new() -> Self {
        Self {
            score: Score { home: 0, away: 0 },
            possession: Possession::Home,
            home_possession_ticks: 0,
            away_possession_ticks: 0,
            shots: (0, 0),
            next_event_slot: 0,
            actions_stream: StreamName::new("match:actions")
                .expect("literal stream name is non-empty"),
            action_kind_stream: StreamName::new("match:action_kind")
                .expect("literal stream name is non-empty"),
            player_pick_stream: StreamName::new("match:player_pick")
                .expect("literal stream name is non-empty"),
        }
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

        let (attacker, defender) = self.pick_players(ctx);

        let action_kind_roll = pcfm_core::rng::next_unit(ctx.rng.stream(&self.action_kind_stream));

        if action_kind_roll < 0.6 {
            let ctx_pass = pcfm_hidden::PassContext { passer: attacker };
            let outcome = ctx.hidden.resolve_pass(&ctx_pass);
            if !outcome.complete {
                self.flip_possession();
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
        }
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

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    fn pick_players(&self, ctx: &mut MatchContext<'_>) -> (PlayerId, PlayerId) {
        let (attacker_squad, defender_squad) = match self.possession {
            Possession::Home => (&ctx.match_.home, &ctx.match_.away),
            Possession::Away => (&ctx.match_.away, &ctx.match_.home),
        };
        let roll = pcfm_core::rng::next_unit(ctx.rng.stream(&self.player_pick_stream));
        let idx_attacker = ((roll * attacker_squad.players.len() as f32) as usize)
            .min(attacker_squad.players.len() - 1);
        let idx_defender = ((roll * defender_squad.players.len() as f32) as usize)
            .min(defender_squad.players.len() - 1);
        (
            attacker_squad.players[idx_attacker].id,
            defender_squad.players[idx_defender].id,
        )
    }

    const fn flip_possession(&mut self) {
        self.possession = match self.possession {
            Possession::Home => Possession::Away,
            Possession::Away => Possession::Home,
        };
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
        let minute = clamp_u8(total_secs / 60, 120);
        let second = clamp_u8(total_secs % 60, 60);
        let minutes_played = clamp_u16(total_secs / 60);

        let event = MatchEvent {
            event_id: EventId::new(self.next_event_slot, 0),
            match_id: ctx.match_.id,
            actor,
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

/// Clamps a `u64` to a `u8`, with a hard upper bound of `max`.
fn clamp_u8(value: u64, max: u8) -> u8 {
    u8::try_from(value.min(u64::from(max))).unwrap_or(max)
}

/// Clamps a `u64` to a `u16`.
fn clamp_u16(value: u64) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}

impl Default for MatchState {
    fn default() -> Self {
        Self::new()
    }
}
