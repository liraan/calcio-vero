use crate::spatial::{SpatialState, TeamShapeState, PressureMap, PressurePoint, Vec2};
use crate::tactics::{
    LineHeight, PhaseKind, PhaseState, PressureLevel, TeamIntent, TeamSide, TeamTacticState,
    WidthMode,
};

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
    phase_state: PhaseState,
    home_tactic: TeamTacticState,
    away_tactic: TeamTacticState,
    spatial: SpatialState,
    pending_commands: Vec<TacticalCommand>,
    snapshots: Vec<TickSnapshot>,
    actions_stream: StreamName,
    action_kind_stream: StreamName,
    player_pick_stream: StreamName,
}

impl MatchState {
    /// Creates the initial state: kickoff, home possession.
    #[must_use]
    pub fn new() -> Self {
        let home_tactic = TeamTacticState::default();
        let away_tactic = TeamTacticState::default();

        Self {
            score: Score { home: 0, away: 0 },
            possession: Possession::Home,
            home_possession_ticks: 0,
            away_possession_ticks: 0,
            shots: (0, 0),
            next_event_slot: 0,
            phase_state: PhaseState {
                kind: PhaseKind::Possession,
                team_in_possession: Some(TeamSide::Home),
                primary_intent: TeamIntent::PossessionCircles,
                pressure_level: PressureLevel::Normal,
                width_mode: WidthMode::Balanced,
                line_height: LineHeight::Mid,
                transition_bias: PressureLevel::Normal,
                start_tick: 0,
                end_tick: None,
            },
            home_tactic,
            away_tactic,
            spatial: SpatialState::default(),
            pending_commands: Vec::new(),
            snapshots: Vec::new(),
            actions_stream: StreamName::new("match:actions").expect("literal stream name is non-empty"),
            action_kind_stream: StreamName::new("match:action_kind").expect("literal stream name is non-empty"),
            player_pick_stream: StreamName::new("match:player_pick").expect("literal stream name is non-empty"),
        }
    }

    /// Queues a live tactical command to be applied to the match state.
    pub fn queue_tactical_command(&mut self, command: TacticalCommand) {
        self.pending_commands.push(command);
    }

    /// Applies a queued tactical command to the authoritative match state.
    pub fn apply_tactical_command(&mut self, command: TacticalCommand) {
        let target = match command.team {
            TeamSide::Home => &mut self.home_tactic,
            TeamSide::Away => &mut self.away_tactic,
        };
        match command.kind {
            crate::tactics::TacticalCommandKind::SetPressingIntensity => {
                target.pressing_intensity = match target.pressing_intensity {
                    PressureLevel::Low => PressureLevel::Normal,
                    PressureLevel::Normal => PressureLevel::High,
                    PressureLevel::High => PressureLevel::Aggressive,
                    PressureLevel::Aggressive => PressureLevel::Aggressive,
                };
            }
            crate::tactics::TacticalCommandKind::InvertFifths => {
                target.fifths_inversion_enabled = !target.fifths_inversion_enabled;
            }
            crate::tactics::TacticalCommandKind::PushWings => {
                target.wing_bias = 1.0;
                target.wide_channel_control = 1.0;
            }
            crate::tactics::TacticalCommandKind::DropLine => {
                target.defensive_line_depth = 1.0;
                target.line_height = 0.5;
            }
            crate::tactics::TacticalCommandKind::AggressivePress => {
                target.pressing_intensity = PressureLevel::Aggressive;
                target.high_press_allowed = true;
            }
            crate::tactics::TacticalCommandKind::CompactMidBlock => {
                target.compactness = 1.2;
            }
            crate::tactics::TacticalCommandKind::CounterPressEnable => {
                target.counterpress_enabled = true;
            }
            crate::tactics::TacticalCommandKind::BuildFromBack => {
                target.line_height = 0.5;
                target.wide_channel_control = 0.8;
            }
            crate::tactics::TacticalCommandKind::DirectPlay => {
                target.wing_bias = 0.8;
                target.aggressiveness = 1.0;
            }
            crate::tactics::TacticalCommandKind::SetFormationBias => {
                target.wing_bias = 0.6;
            }
            crate::tactics::TacticalCommandKind::SwitchPhase => {
                self.phase_state.kind = PhaseKind::Transition;
            }
        }
        self.update_phase_state();
    }

    fn apply_pending_commands(&mut self) {
        let pending = std::mem::take(&mut self.pending_commands);
        for command in pending {
            self.apply_tactical_command(command);
        }
    }

    fn update_phase_state(&mut self) {
        let team = match self.possession {
            Possession::Home => TeamSide::Home,
            Possession::Away => TeamSide::Away,
        };
        let tactic = match team {
            TeamSide::Home => &self.home_tactic,
            TeamSide::Away => &self.away_tactic,
        };

        self.phase_state.team_in_possession = Some(team);
        self.phase_state.pressure_level = tactic.pressing_intensity;
        self.phase_state.width_mode = if tactic.width < 0.75 {
            WidthMode::Narrow
        } else if tactic.width > 1.25 {
            WidthMode::Wide
        } else {
            WidthMode::Balanced
        };
        self.phase_state.line_height = if tactic.line_height < 0.6 {
            LineHeight::Deep
        } else if tactic.line_height > 1.2 {
            LineHeight::High
        } else {
            LineHeight::Mid
        };

        self.phase_state.kind = match tactic.pressing_intensity {
            PressureLevel::Aggressive if tactic.counterpress_enabled => PhaseKind::Transition,
            PressureLevel::Aggressive => PhaseKind::PressingPhase,
            PressureLevel::High => PhaseKind::Transition,
            _ => {
                if self.possession == Possession::Home {
                    PhaseKind::Possession
                } else {
                    PhaseKind::NonPossession
                }
            }
        };
        self.phase_state.primary_intent = if tactic.counterpress_enabled {
            TeamIntent::CounterPress
        } else if tactic.high_press_allowed {
            TeamIntent::HighPress
        } else if tactic.width > 1.1 {
            TeamIntent::PushWide
        } else {
            TeamIntent::PossessionCircles
        };
    }

    /// Derives the world-space spatial state from the authoritative match state.
    ///
    /// This is intentionally a lightweight rendering-prep step, not a new rules
    /// engine. It keeps the match engine and the renderer separate while still
    /// exposing a consistent field view for later TUI/2D projections.
    fn update_spatial_state(&mut self) {
        let ball_x = match self.possession {
            Possession::Home => 0.15,
            Possession::Away => -0.15,
        };
        let ball_y = match self.phase_state.line_height {
            LineHeight::Deep => -0.1,
            LineHeight::Mid => 0.0,
            LineHeight::High => 0.1,
        };

        self.spatial = SpatialState {
            ball: crate::spatial::BallState {
                position: Vec2 { x: ball_x, y: ball_y },
                velocity: Vec2 { x: 0.0, y: 0.0 },
                owner: None,
                last_touch: None,
            },
            players: vec![
                crate::spatial::PlayerSpatialState {
                    player_id: 1,
                    position: Vec2 { x: -0.35, y: 0.15 },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    heading: 0.0,
                    role: "midfielder",
                    zone: "middle",
                    spacing_bias: self.home_tactic.width,
                },
                crate::spatial::PlayerSpatialState {
                    player_id: 2,
                    position: Vec2 { x: 0.35, y: -0.15 },
                    velocity: Vec2 { x: 0.0, y: 0.0 },
                    heading: 0.0,
                    role: "defender",
                    zone: "left",
                    spacing_bias: self.away_tactic.width,
                },
            ],
            home_shape: TeamShapeState {
                width: self.home_tactic.width,
                depth: self.home_tactic.line_height,
                compactness: self.home_tactic.compactness,
                line_height: self.home_tactic.line_height,
                left_anchor: -0.5,
                right_anchor: 0.5,
                press_line: self.home_tactic.defensive_line_depth,
            },
            away_shape: TeamShapeState {
                width: self.away_tactic.width,
                depth: self.away_tactic.line_height,
                compactness: self.away_tactic.compactness,
                line_height: self.away_tactic.line_height,
                left_anchor: -0.5,
                right_anchor: 0.5,
                press_line: self.away_tactic.defensive_line_depth,
            },
            pressure: PressureMap {
                home: vec![PressurePoint {
                    position: Vec2 { x: 0.0, y: 0.0 },
                    intensity: self.home_tactic.pressing_intensity as i32 as f32 / 3.0,
                }],
                away: vec![PressurePoint {
                    position: Vec2 { x: 0.0, y: 0.0 },
                    intensity: self.away_tactic.pressing_intensity as i32 as f32 / 3.0,
                }],
            },
        };
    }

    /// Produces a renderable field projection from the authoritative match state.
    #[must_use]
    pub fn render_field(&self) -> crate::projection::FieldProjection {
        use crate::projection::FieldRenderer;
        let renderer = FieldRenderer::new();
        renderer.project(
            &self.spatial,
            Some(self.phase_state),
            Some(match self.possession {
                Possession::Home => TeamSide::Home,
                Possession::Away => TeamSide::Away,
            }),
        )
    }

    /// Advances the state by one tick.
    #[allow(clippy::too_many_lines)]
    pub fn tick(&mut self, tick: u64, ctx: &mut MatchContext<'_>) {
        self.apply_pending_commands();
        self.update_phase_state();

        match self.possession {
            Possession::Home => self.home_possession_ticks += 1,
            Possession::Away => self.away_possession_ticks += 1,
        }

        let action_roll = pcfm_core::rng::next_unit(ctx.rng.stream(&self.actions_stream));
        if action_roll >= ctx.config.action_probability {
            self.record_snapshot(tick, ctx);
            self.update_spatial_state();
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
                attacker,
                pcfm_stats::EventType::Shot,
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
                self.emit_event(ctx, tick, attacker, pcfm_stats::EventType::Goal, serde_json::json!({}));
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
                attacker,
                pcfm_stats::EventType::Duel,
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
            self.emit_event(ctx, tick, defender, pcfm_stats::EventType::Foul, serde_json::json!({}));
            self.flip_possession();
        }

        self.update_phase_state();
        self.update_spatial_state();
        self.record_snapshot(tick, ctx);
    }

    /// Produces the final result.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn finish(&self, chain: pcfm_stats::EventChain) -> crate::result::MatchResult {
        let total = self.home_possession_ticks + self.away_possession_ticks;
        let possession = if total == 0 {
            (0.5, 0.5)
        } else {
            let home = (self.home_possession_ticks as f32) / (total as f32);
            (home, 1.0 - home)
        };

        let state_hash = chain.chain_hash();

        crate::result::MatchResult {
            score: self.score,
            events: chain,
            stats: crate::result::MatchStatistics {
                possession,
                shots: self.shots,
                goals: (self.score.home, self.score.away),
            },
            state_hash,
            narrative: Vec::new(),
            snapshots: self.snapshots.clone(),
        }
    }

    fn record_snapshot(&mut self, tick: u64, ctx: &MatchContext<'_>) {
        self.snapshots.push(crate::result::TickSnapshot {
            tick,
            score: self.score,
            home_possession_ticks: self.home_possession_ticks,
            away_possession_ticks: self.away_possession_ticks,
            events_in_chain: ctx.chain.len(),
            state_hash: ctx.chain.chain_hash(),
        });
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    fn pick_players(&self, ctx: &mut MatchContext<'_>) -> (pcfm_core::ids::PlayerId, pcfm_core::ids::PlayerId) {
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
        player_id: pcfm_core::ids::PlayerId,
        event_type: pcfm_stats::EventType,
        fields: serde_json::Value,
    ) {
        let ticks_per_sec = u64::from(ctx.config.ticks_per_second).max(1);
        let total_secs = tick / ticks_per_sec;
        let minute = clamp_u8(total_secs / 60, 120);
        let second = clamp_u8(total_secs % 60, 60);
        let minutes_played = clamp_u16(total_secs / 60);

        let event = pcfm_stats::MatchEvent {
            event_id: pcfm_core::ids::EventId::new(self.next_event_slot, 0),
            match_id: ctx.match_.id,
            player_id,
            timestamp_minute: minute,
            timestamp_second: second,
            minutes_played_cumulative: minutes_played,
            event_type,
            fields,
            created_at: pcfm_core::time::GameInstant::from_raw(tick),
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

use pcfm_core::ids::{EventId, PlayerId};
use pcfm_core::rng::StreamName;
use pcfm_core::time::GameInstant;
use pcfm_stats::{EventChain, EventType, MatchEvent};

use crate::context::MatchContext;
use crate::result::{MatchResult, MatchStatistics, Score, TickSnapshot};
use crate::tactics::{
    LineHeight, PhaseKind, PhaseState, PressureLevel, TacticalCommand, TeamIntent, TeamSide,
    TeamTacticState, WidthMode,
};

// Re-export the type alias used in the file for convenience.
pub(crate) use crate::tactics::TacticalCommandKind;
