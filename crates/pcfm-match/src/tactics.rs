//! Tactical and phase-state model.
//!
//! This module formalises the separation between simulation state and
//! rendering. The engine owns the match state, while renderers project it for
//! text, HTML, 2D, or 3D views. The tactical model is part of the match
//! state itself and is therefore authoritative: renderers must not invent
//! missing tactical semantics.

use serde::{Deserialize, Serialize};

/// The side of the pitch / team being discussed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamSide {
    /// Home team.
    Home,
    /// Away team.
    Away,
}

/// The high-level phase of play.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseKind {
    /// Team in possession is controlling the ball.
    Possession,
    /// Team out of possession is reorganising / defending.
    NonPossession,
    /// The team is changing state between possession regimes.
    Transition,
    /// A set piece is being executed.
    SetPiece,
    /// A dead-ball phase such as throw-in, goal kick, or stoppage.
    DeadBall,
    /// A restarted phase after a stoppage.
    Restart,
    /// A break into an immediate counter-attack.
    CounterAttack,
    /// Controlled progression from the back.
    ControlledBuildUp,
    /// A pressing event keyed to current pressure intensity.
    PressingPhase,
}

/// Overall pressure intensity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureLevel {
    Low,
    Normal,
    High,
    Aggressive,
}

/// Relative line width preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidthMode {
    Narrow,
    Balanced,
    Wide,
}

/// Relative line-height preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineHeight {
    Deep,
    Mid,
    High,
}

/// Core tactical intent for a team.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamIntent {
    BuildFromBack,
    PossessionCircles,
    DirectAttack,
    WingOverload,
    CentralControl,
    HighPress,
    CompactBlock,
    CounterPress,
    ProtectMiddle,
    InvertLines,
    PushWide,
    DropDeep,
}

/// The current phase of play and its tactical context.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhaseState {
    /// Current phase kind.
    pub kind: PhaseKind,
    /// Which team is currently dominant in the phase.
    pub team_in_possession: Option<TeamSide>,
    /// Primary intent of the phase.
    pub primary_intent: TeamIntent,
    /// Pressure level applied in this phase.
    pub pressure_level: PressureLevel,
    /// Relative width used during this phase.
    pub width_mode: WidthMode,
    /// Relative line height used during this phase.
    pub line_height: LineHeight,
    /// Current transition bias.
    pub transition_bias: PressureLevel,
    /// Start tick for this phase.
    pub start_tick: u64,
    /// Optional end tick for this phase.
    pub end_tick: Option<u64>,
}

/// A role-specific override for a specific player.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleOverride {
    /// The player being overridden.
    pub player_id: u32,
    /// Intended role for that player.
    pub role: &'static str,
    /// Priority of this override.
    pub priority: u8,
    /// Preferred zone for the player.
    pub target_zone: Option<Zone>,
    /// Whether the player is allowed to push forward.
    pub allowed_to_push: bool,
    /// Whether the player is allowed to press.
    pub allowed_to_press: bool,
    /// Relative spacing modifier in the current phase.
    pub spacing_modifier: f32,
    /// Relative aggression modifier in the current phase.
    pub aggression_modifier: f32,
}

/// Spatial zones used for tactical targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Zone {
    LeftHalf,
    RightHalf,
    CentralChannel,
    LeftWing,
    RightWing,
    HalfSpaceLeft,
    HalfSpaceRight,
    Box,
    FrontThird,
    MiddleThird,
    BackThird,
}

/// Intended spatial behaviour for a player in a phase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialIntent {
    /// The player involved.
    pub player_id: u32,
    /// High-level target zone.
    pub target_zone: Zone,
    /// Preferred distance from the preferred path.
    pub preferred_distance: f32,
    /// Minimum width spacing.
    pub min_spacing: f32,
    /// Maximum width spacing.
    pub max_spacing: f32,
    /// Pressure radius around the player.
    pub pressure_radius: f32,
    /// Direction in which pressure or push should be applied.
    pub push_direction: Option<Direction>,
}

/// Directional hints for spatial behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

/// Tactical state for one side.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamTacticState {
    /// Team side.
    pub team: TeamSide,
    /// Current pressing intensity.
    pub pressing_intensity: PressureLevel,
    /// Width preference.
    pub width: f32,
    /// Line height preference.
    pub line_height: f32,
    /// Compactness preference.
    pub compactness: f32,
    /// Aggressiveness preference.
    pub aggressiveness: f32,
    /// Wing bias in the current structure.
    pub wing_bias: f32,
    /// Central density preference.
    pub central_density: f32,
    /// Defensive line depth preference.
    pub defensive_line_depth: f32,
    /// Whether the team is currently configured to counter-press.
    pub counterpress_enabled: bool,
    /// Whether the team has space to press high.
    pub high_press_allowed: bool,
    /// Whether wide channels are actively controlled.
    pub wide_channel_control: f32,
    /// Whether the line inversion is active.
    pub fifths_inversion_enabled: bool,
    /// Role-specific behavioural overrides.
    pub role_overrides: Vec<RoleOverride>,
}

/// A tactical change requested by a coach, AI, or rule engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TacticalCommand {
    /// The team affected.
    pub team: TeamSide,
    /// Simulation tick at which the command was issued.
    pub tick: u64,
    /// The kind of change.
    pub kind: TacticalCommandKind,
    /// Optional target of the command.
    pub target: Option<TacticTarget>,
    /// Duration in ticks, if this command is temporary.
    pub duration: Option<u64>,
    /// Priority of the tactical command.
    pub priority: u8,
    /// The reason behind the command.
    pub reason: Option<TacticalReason>,
    /// Source of the command.
    pub source: TacticalSource,
    /// Earliest tick when the command becomes active.
    pub valid_from_tick: u64,
    /// Optional tick after which the command expires.
    pub valid_until_tick: Option<u64>,
}

/// Specific tactical operations that can be applied live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalCommandKind {
    SwitchPhase,
    SetPressingIntensity,
    InvertFifths,
    PushWings,
    DropLine,
    AggressivePress,
    CompactMidBlock,
    CounterPressEnable,
    BuildFromBack,
    DirectPlay,
    SetFormationBias,
}

/// Optional target used by a tactical command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticTarget {
    Team,
    LeftWing,
    RightWing,
    CentralChannel,
    PressingZone,
    DefensiveLine,
}

/// The reason why a tactical command was issued.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalReason {
    Leading,
    Trailing,
    AfterRecovery,
    AfterConceding,
    UnderPressure,
    MatchState,
    SetPiece,
}

/// Source of the tactical command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticalSource {
    HumanCoach,
    AIManager,
    Rule,
    SetPiece,
}

impl Default for PhaseState {
    fn default() -> Self {
        Self {
            kind: PhaseKind::Possession,
            team_in_possession: None,
            primary_intent: TeamIntent::PossessionCircles,
            pressure_level: PressureLevel::Normal,
            width_mode: WidthMode::Balanced,
            line_height: LineHeight::Mid,
            transition_bias: PressureLevel::Normal,
            start_tick: 0,
            end_tick: None,
        }
    }
}

impl Default for TeamTacticState {
    fn default() -> Self {
        Self {
            team: TeamSide::Home,
            pressing_intensity: PressureLevel::Normal,
            width: 1.0,
            line_height: 1.0,
            compactness: 1.0,
            aggressiveness: 0.5,
            wing_bias: 0.0,
            central_density: 0.5,
            defensive_line_depth: 0.5,
            counterpress_enabled: false,
            high_press_allowed: true,
            wide_channel_control: 0.5,
            fifths_inversion_enabled: false,
            role_overrides: Vec::new(),
        }
    }
}
