//! Outcome types for `resolve_*` calls.
//!
//! Outcomes are Plane B data: observed results that the engine interprets.
//! They carry no latent value and no cause trace: since ADR-023
//! (accepted 2026-10-31) the engine builds the cause from the observed fields.

use pcfm_core::ids::PlayerId;

/// The outcome of a duel.
#[derive(Debug, Clone)]
pub struct DuelOutcome {
    /// The winner.
    pub winner: PlayerId,
    /// Whether the attacker retained the ball.
    pub ball_retained: bool,
    /// Whether a foul was committed, and by whom.
    pub foul_committed: Option<PlayerId>,
}

/// The outcome of a shot.
#[derive(Debug, Clone)]
pub struct ShotOutcome {
    /// Whether the shot resulted in a goal.
    pub goal: bool,
    /// Whether the shot was on target.
    pub on_target: bool,
}

/// The outcome of a pass.
#[derive(Debug, Clone)]
pub struct PassOutcome {
    /// Whether the pass was completed.
    pub complete: bool,
}

/// The outcome of a pressure event.
#[derive(Debug, Clone)]
pub struct PressureOutcome {
    /// Whether the pressure succeeded.
    pub success: bool,
}

/// The outcome of an injury risk resolution.
#[derive(Debug, Clone)]
pub struct InjuryOutcome {
    /// Whether the player was injured.
    pub injured: bool,
}

/// The outcome of a save.
#[derive(Debug, Clone)]
pub struct SaveOutcome {
    /// Whether the save succeeded.
    pub saved: bool,
}

/// The outcome of a set piece.
#[derive(Debug, Clone)]
pub struct SetPieceOutcome {
    /// Whether the set piece succeeded.
    pub success: bool,
}
