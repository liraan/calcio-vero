//! Inputs to the `resolve_*` functions (Plane B).
//!
//! These structs are constructed by the engine and passed to
//! `pcfm-hidden`. They carry only Plane B data (positions, ids); latents
//! are read inside the crate.

use pcfm_core::ids::PlayerId;

/// The type of a duel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DuelType {
    /// Ground duel (tackle, dribble).
    Ground,
    /// Aerial duel (header).
    Aerial,
    /// Contest with the goalkeeper (cross, shot under pressure).
    WithGoalkeeper,
}

/// The context of a duel resolution.
#[derive(Debug, Clone, Copy)]
pub struct DuelContext {
    /// The attacker.
    pub attacker: PlayerId,
    /// The defender.
    pub defender: PlayerId,
    /// The type of duel.
    pub duel_type: DuelType,
}

/// The context of a shot resolution.
#[derive(Debug, Clone, Copy)]
pub struct ShotContext {
    /// The shooter.
    pub shooter: PlayerId,
    /// Where the shot starts (the ball), in centimetres, 3D.
    pub from: (i32, i32, i32),
    /// The goal centre, in centimetres, 3D.
    pub to: (i32, i32, i32),
    /// A `0..=100` value the caller computes from the situation
    /// (opponents near the shooter, distance to goal). Declared model
    /// choice: the die widens its deviation under pressure. Not a latent.
    pub difficulty: i32,
}

/// The context of a pass resolution.
#[derive(Debug, Clone, Copy)]
pub struct PassContext {
    /// The passer.
    pub passer: PlayerId,
    /// Where the pass starts (the ball), in centimetres, 3D.
    pub from: (i32, i32, i32),
    /// The receiver position, in centimetres, 3D.
    pub to: (i32, i32, i32),
}

/// The context of a pressure resolution.
#[derive(Debug, Clone, Copy)]
pub struct PressureContext {
    /// The player under pressure.
    pub player: PlayerId,
}

/// The context of an injury risk resolution.
#[derive(Debug, Clone, Copy)]
pub struct InjuryContext {
    /// The player at risk.
    pub player: PlayerId,
}

/// The context of a save resolution.
#[derive(Debug, Clone, Copy)]
pub struct SaveContext {
    /// The goalkeeper.
    pub goalkeeper: PlayerId,
    /// The point where the ball crossed the goal plane, in centimetres, 3D.
    /// The engine computes it from the trajectory; it is the geometric
    /// input the goalkeeper's reaction depends on.
    pub point: (i32, i32, i32),
}

/// The context of a set piece resolution.
#[derive(Debug, Clone, Copy)]
pub struct SetPieceContext {
    /// The taker.
    pub taker: PlayerId,
}

/// The latent attribute used as modulator by an influence query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillKind {
    /// Tackling.
    Tackling,
    /// Passing.
    Passing,
    /// Finishing.
    Finishing,
    /// Aerial.
    Aerial,
    /// Goalkeeping.
    Goalkeeping,
}

/// One agent as seen by an influence query.
///
/// Position in centimetres, fatigue in 0..=1000 (fixed-point, x1000).
/// No latent: the latent is read inside the crate.
#[derive(Debug, Clone, Copy)]
pub struct InfluenceAgent {
    /// The player.
    pub id: PlayerId,
    /// Position in centimetres.
    pub position: (i32, i32),
    /// Fatigue, fixed-point x1000, 0..=1000.
    pub fatigue_milli: i32,
}

/// The context of an influence query.
///
/// Plane B data only: positions and fatigue are observed. The skill term is
/// read inside the crate from the latents; the engine never sees it.
#[derive(Debug, Clone)]
pub struct InfluenceContext {
    /// The point being queried.
    pub point: (i32, i32),
    /// The agents to rank.
    pub agents: Vec<InfluenceAgent>,
    /// Which latent modulates the skill term.
    pub skill: SkillKind,
}

/// Weights and thresholds for the influence formula.
///
/// Prototype values, not calibrated. The form is doctrine; the numbers are
/// calibration (see `balance/influence.json`).
#[derive(Debug, Clone, Copy)]
pub struct InfluenceWeights {
    /// Weight of the space term.
    pub w_space: i32,
    /// Weight of the skill term.
    pub w_skill: i32,
    /// Weight of the fatigue term.
    pub w_cost: i32,
    /// Weight of the redundancy term.
    pub w_red: i32,
    /// Distance (cm) at which the space term reaches zero.
    pub space_threshold_cm: i32,
}
