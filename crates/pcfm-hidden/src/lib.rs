//! `pcfm-hidden` — Plane A (Ground Truth) for PCFM.
//!
//! See `specs/L0-match/10-ground-truth.md` and
//! `docs/adr/003-plane-a-enforcement.md` for the full specification.
//!
//! # What this crate is
//!
//! The only crate that contains latent attributes (Plane A). It exposes
//! **only** computations (`resolve_*`) and **only** Plane B observations
//! (`drain_observations`). It never exposes a latent value.
//!
//! # What this crate is not
//!
//! - It is **not** `Serialize`. There is no public way to serialise a
//!   `GroundTruth`.
//! - Its `Debug` impl prints only `GroundTruth { .. }`.
//! - It has **no** public fields.

#![deny(missing_docs)]

pub mod context;
pub mod observation;
pub mod outcome;

mod latents;

pub use context::{
    DuelContext, DuelType, InjuryContext, PassContext, PressureContext, SaveContext,
    SetPieceContext, ShotContext,
};
pub use observation::ObservationEvent;
pub use outcome::{
    DuelOutcome, InjuryOutcome, PassOutcome, PressureOutcome, SaveOutcome, SetPieceOutcome,
    ShotOutcome,
};

use std::collections::BTreeMap;
use std::fmt;

use pcfm_core::ids::{MatchId, PlayerId};
use pcfm_core::rng::{RngRegistry, StreamName};
use rand_chacha::ChaCha20Rng;

use crate::latents::PlayerLatents;
use pcfm_core::rng::next_unit;

/// Plane A. The actual state of the world, never exposed.
pub struct GroundTruth {
    world_seed: u64,
    match_id: MatchId,
    rng: ChaCha20Rng,
    latents: BTreeMap<PlayerId, PlayerLatents>,
    observations: Vec<ObservationEvent>,
}

impl GroundTruth {
    /// Creates a new `GroundTruth` from a world seed and a match id.
    ///
    /// # Panics
    ///
    /// Panics if the derived stream name is empty. The name is a formatted
    /// string that always contains at least the literal `"hidden:"`, so this
    /// panic is unreachable in practice. It is documented for completeness.
    #[must_use]
    pub fn new(world_seed: u64, match_id: MatchId) -> Self {
        let mut registry = RngRegistry::new(world_seed);
        let name = StreamName::new(format!("hidden:{}:resolve", match_id.slot()))
            .expect("stream name is always non-empty");
        let rng = registry.stream(&name).clone();
        Self {
            world_seed,
            match_id,
            rng,
            latents: BTreeMap::new(),
            observations: Vec::new(),
        }
    }

    /// Returns the world seed.
    #[must_use]
    pub const fn world_seed(&self) -> u64 {
        self.world_seed
    }

    /// Returns the match id.
    #[must_use]
    pub const fn match_id(&self) -> MatchId {
        self.match_id
    }

    /// Resolves a duel.
    ///
    /// # The prototype formula
    ///
    /// `attacker_wins = noise * 0.2 + attacker_score > defender_score`
    ///
    /// - `noise` is uniform in `[0, 1)`, drawn from the `hidden` RNG stream.
    /// - `0.2` is the **variance weight**: a higher value means the outcome
    ///   is less predictable from the latents. It is a placeholder, not a
    ///   calibrated value. A realistic model would derive it from the
    ///   distribution of duels in real matches (e.g. what fraction of
    ///   aerial duels the stronger player wins).
    /// - `attacker_score` and `defender_score` are **latent** values
    ///   (Plane A). They are read here, inside `pcfm-hidden`, and never
    ///   leave. The outcome is a `DuelOutcome` (Plane B) with only
    ///   observed fields; the engine builds the cause trace.
    ///
    /// See `docs/CODE_TOUR.md` "A note on the current simulation".
    pub fn resolve_duel(&mut self, ctx: &DuelContext) -> DuelOutcome {
        let attacker = self.latents_for(ctx.attacker);
        let defender = self.latents_for(ctx.defender);
        let noise = next_unit(&mut self.rng);

        let attacker_score = match ctx.duel_type {
            DuelType::Aerial => attacker.aerial,
            DuelType::Ground => attacker.tackling.mul_add(-1.0, 1.0),
            DuelType::WithGoalkeeper => attacker.finishing,
        };
        let defender_score = match ctx.duel_type {
            DuelType::Aerial => defender.aerial,
            DuelType::Ground => defender.tackling,
            DuelType::WithGoalkeeper => defender.goalkeeping,
        };

        let attacker_wins = noise.mul_add(0.2, attacker_score) > defender_score;
        let winner = if attacker_wins {
            ctx.attacker
        } else {
            ctx.defender
        };

        let outcome = DuelOutcome {
            winner,
            ball_retained: attacker_wins,
            foul_committed: None,
        };

        self.observations.push(ObservationEvent::Duel {
            attacker: ctx.attacker,
            defender: ctx.defender,
            duel_type: ctx.duel_type,
            winner,
        });

        outcome
    }

    /// Resolves a shot.
    pub fn resolve_shot(&mut self, ctx: &ShotContext) -> ShotOutcome {
        let shooter = self.latents_for(ctx.shooter);
        let noise = next_unit(&mut self.rng);
        let goal_probability = noise.mul_add(0.3, shooter.finishing).clamp(0.0, 1.0);
        let goal = goal_probability > 0.6;

        let outcome = ShotOutcome {
            goal,
            on_target: goal || noise > 0.3,
        };

        self.observations.push(ObservationEvent::Shot {
            shooter: ctx.shooter,
            goal,
        });

        outcome
    }

    /// Resolves a pass.
    pub fn resolve_pass(&mut self, ctx: &PassContext) -> PassOutcome {
        let passer = self.latents_for(ctx.passer);
        let noise = next_unit(&mut self.rng);
        let complete = noise.mul_add(0.3, passer.passing) > 0.5;

        let outcome = PassOutcome { complete };

        self.observations.push(ObservationEvent::Pass {
            passer: ctx.passer,
            complete,
        });

        outcome
    }

    /// Resolves a pressure event.
    pub fn resolve_pressure(&mut self, ctx: &PressureContext) -> PressureOutcome {
        let _ = self.latents_for(ctx.player);
        let noise = next_unit(&mut self.rng);
        let success = noise > 0.5;

        let outcome = PressureOutcome { success };

        self.observations.push(ObservationEvent::Pressure {
            player: ctx.player,
            success,
        });

        outcome
    }

    /// Resolves an injury risk.
    pub fn resolve_injury_risk(&mut self, ctx: &InjuryContext) -> InjuryOutcome {
        let player = self.latents_for(ctx.player);
        let noise = next_unit(&mut self.rng);
        let injured = noise.mul_add(0.3, player.injury_proneness) > 0.8;

        let outcome = InjuryOutcome { injured };

        self.observations.push(ObservationEvent::Injury {
            player: ctx.player,
            injured,
        });

        outcome
    }

    /// Resolves a save.
    pub fn resolve_save(&mut self, ctx: &SaveContext) -> SaveOutcome {
        let keeper = self.latents_for(ctx.goalkeeper);
        let noise = next_unit(&mut self.rng);
        let saved = noise.mul_add(0.3, keeper.goalkeeping) > 0.5;

        let outcome = SaveOutcome { saved };

        self.observations.push(ObservationEvent::Save {
            goalkeeper: ctx.goalkeeper,
            saved,
        });

        outcome
    }

    /// Resolves a set piece.
    pub fn resolve_set_piece(&mut self, ctx: &SetPieceContext) -> SetPieceOutcome {
        let taker = self.latents_for(ctx.taker);
        let noise = next_unit(&mut self.rng);
        let success = noise.mul_add(0.3, taker.set_pieces) > 0.5;

        let outcome = SetPieceOutcome { success };

        self.observations.push(ObservationEvent::SetPiece {
            taker: ctx.taker,
            success,
        });

        outcome
    }

    /// Drains the accumulated observations.
    ///
    /// The method is idempotent: the second call returns an empty `Vec`.
    pub fn drain_observations(&mut self) -> Vec<ObservationEvent> {
        std::mem::take(&mut self.observations)
    }

    fn latents_for(&mut self, player: PlayerId) -> PlayerLatents {
        if let Some(l) = self.latents.get(&player) {
            return *l;
        }
        let new = PlayerLatents::from_rng(&mut self.rng);
        self.latents.insert(player, new);
        new
    }
}

impl fmt::Debug for GroundTruth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroundTruth")
            .field("world_seed", &self.world_seed)
            .field("match_id", &self.match_id)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcfm_core::ids::{MatchId, PlayerId};

    fn ctx_duel() -> DuelContext {
        DuelContext {
            attacker: PlayerId::new(1, 0),
            defender: PlayerId::new(2, 0),
            duel_type: DuelType::Ground,
        }
    }

    #[test]
    fn debug_does_not_leak_latents() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let _ = gt.resolve_duel(&ctx_duel());
        let debug = format!("{gt:?}");
        assert!(!debug.contains("finishing"));
        assert!(!debug.contains("passing"));
        assert!(!debug.contains("tackling"));
        assert!(!debug.contains("injury_proneness"));
        assert!(debug.contains("GroundTruth"));
    }

    #[test]
    fn drain_is_idempotent() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let _ = gt.resolve_duel(&ctx_duel());
        let first = gt.drain_observations();
        let second = gt.drain_observations();
        assert!(!first.is_empty());
        assert!(second.is_empty());
    }

    #[test]
    fn two_ground_truths_with_same_seed_produce_same_outcomes() {
        let mut a = GroundTruth::new(42, MatchId::new(1, 0));
        let mut b = GroundTruth::new(42, MatchId::new(1, 0));
        let ctx = ctx_duel();
        let oa = a.resolve_duel(&ctx);
        let ob = b.resolve_duel(&ctx);
        assert_eq!(oa.winner, ob.winner);
    }

    #[test]
    fn observations_accumulate() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let _ = gt.resolve_duel(&ctx_duel());
        let _ = gt.resolve_duel(&ctx_duel());
        let _ = gt.resolve_duel(&ctx_duel());
        let drained = gt.drain_observations();
        assert_eq!(drained.len(), 3);
    }
}
