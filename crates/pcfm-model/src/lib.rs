//! `pcfm-model` — minimal entities for PCFM.
//!
//! See `specs/L0-match/03-entities-minimal.md` for the full specification.
//!
//! # Design
//!
//! This crate contains only the entities required to play a single match.
//! It does **not** contain Club, Competition, Contract, Staff, or any other
//! L1/L2 entity. If the match engine does not need it, it is not here.
//!
//! # Plane separation
//!
//! Every entity in this crate is **Plane B/C data**: it can be serialised,
//! logged, and displayed. Latent attributes (Plane A) do **not** appear in
//! any struct here. They live in `pcfm-hidden`, indexed by `PlayerId`, and
//! are accessed only via `GroundTruth::resolve_*` (see ADR-003 and
//! `specs/L0-match/10-ground-truth.md`).
//!
//! # Future-facing spatial foundation
//!
//! The crate also exposes a future-proof spatial foundation in [`spatial`].
//! This is not part of the current L0 match contract, but it provides the
//! geometry and movement primitives required by FM-style positional play in
//! later layers. It remains deliberately separated from deterministic engine
//! state and the hidden state boundary.

#![deny(missing_docs)]

pub mod error;
pub mod match_;
pub mod pitch;
pub mod player;
pub mod referee;
pub mod spatial;
pub mod squad;

pub use error::ModelError;
pub use match_::{Match, MatchRules, MatchSeed};
pub use pitch::{Pitch, Weather};
pub use player::{Foot, PhysicalStateMinimal, Player, PsychologicalStateMinimal, RoleId};
pub use referee::Referee;
pub use spatial::{
    BallState, FieldSide, FieldZone, Heading, PitchBounds, PlayerPose, Position2D, Velocity2D,
};
pub use squad::{FormationId, Squad};

/// Returns the version of `pcfm-model`, as declared in `Cargo.toml`.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert_ne!(version(), "");
    }
}
