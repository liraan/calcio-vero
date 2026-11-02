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
    DuelContext, DuelType, InfluenceAgent, InfluenceContext, InfluenceWeights, InjuryContext,
    PassContext, PressureContext, SaveContext, SetPieceContext, ShotContext, SkillKind,
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

impl GroundTruth {
    /// Ranks the agents by influence on a point (the formula is in
    /// `specs/L0-match/03-entities-minimal.md` §2.9).
    ///
    /// The output is an order of `PlayerId`, most influential first. The
    /// continuous value never leaves this crate. Determinism: the agents are
    /// sorted by (distance from the point, `PlayerId`) before any latent is
    /// generated, so the RNG consumption order does not depend on the Vec
    /// order given by the caller.
    pub fn resolve_influence(
        &mut self,
        ctx: &InfluenceContext,
        weights: &InfluenceWeights,
    ) -> Vec<PlayerId> {
        let mut agents: Vec<InfluenceAgent> = ctx.agents.clone();
        agents.sort_by(|a, b| {
            let da = distance_squared(ctx.point, a.position);
            let db = distance_squared(ctx.point, b.position);
            da.cmp(&db).then(a.id.slot().cmp(&b.id.slot()))
        });
        let skills: Vec<i32> = agents
            .iter()
            .map(|a| self.skill_milli(a.id, ctx.skill))
            .collect();
        let raw: Vec<i32> = agents
            .iter()
            .zip(skills.iter())
            .map(|(a, s)| raw_influence(ctx.point, a, *s, weights))
            .collect();
        let n = agents.len();
        let mut net: Vec<(PlayerId, i32)> = Vec::with_capacity(n);
        for i in 0..n {
            let mut redundancy: i64 = 0;
            for j in 0..n {
                if j == i {
                    continue;
                }
                if distance_squared(ctx.point, agents[j].position)
                    < distance_squared(ctx.point, agents[i].position)
                {
                    redundancy += i64::from(raw[j].max(0));
                }
            }
            let red_term = i32::try_from(redundancy / 100).unwrap_or(i32::MAX);
            let score = raw[i] - (weights.w_red * red_term) / 1000;
            net.push((agents[i].id, score));
        }
        net.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.slot().cmp(&b.0.slot())));
        net.into_iter().map(|(id, _)| id).collect()
    }
}

/// Squared distance in integer centimetres.
fn distance_squared(a: (i32, i32), b: (i32, i32)) -> i64 {
    let dx = i64::from(a.0 - b.0);
    let dy = i64::from(a.1 - b.1);
    dx * dx + dy * dy
}

/// The raw (pre-redundancy) influence of one agent on a point, fixed-point.
fn raw_influence(
    point: (i32, i32),
    agent: &InfluenceAgent,
    skill_milli: i32,
    w: &InfluenceWeights,
) -> i32 {
    let d2 = distance_squared(point, agent.position);
    let t2 = i64::from(w.space_threshold_cm) * i64::from(w.space_threshold_cm);
    let space_milli = if d2 >= t2 { 0 } else { ((t2 - d2) * 1000) / t2 };
    let space_milli = i32::try_from(space_milli).unwrap_or(1000);
    let fatigue_milli = agent.fatigue_milli.clamp(0, 1000);
    (w.w_space * space_milli + w.w_skill * skill_milli - w.w_cost * fatigue_milli) / 1000
}

impl GroundTruth {
    /// The latent for `skill` of `player`, fixed-point x1000.
    fn skill_milli(&mut self, player: PlayerId, skill: SkillKind) -> i32 {
        let l = self.latents_for(player);
        let v = match skill {
            SkillKind::Tackling => l.tackling,
            SkillKind::Passing => l.passing,
            SkillKind::Finishing => l.finishing,
            SkillKind::Aerial => l.aerial,
            SkillKind::Goalkeeping => l.goalkeeping,
        };
        #[allow(clippy::cast_possible_truncation)]
        let milli = (v * 1000.0) as i32;
        milli.clamp(0, 1000)
    }
}

#[cfg(test)]
mod influence_tests {
    use super::*;
    use pcfm_core::ids::PlayerId;

    fn w() -> InfluenceWeights {
        InfluenceWeights {
            w_space: 100,
            w_skill: 40,
            w_cost: 20,
            w_red: 50,
            space_threshold_cm: 2000,
        }
    }

    fn agent(slot: u32, x: i32, y: i32, fatigue_milli: i32) -> InfluenceAgent {
        InfluenceAgent {
            id: PlayerId::new(slot, 0),
            position: (x, y),
            fatigue_milli,
        }
    }

    #[test]
    fn closer_agent_ranks_higher() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let c = InfluenceContext {
            point: (0, 0),
            agents: vec![agent(1, 100, 0, 0), agent(2, 1500, 0, 0)],
            skill: SkillKind::Tackling,
        };
        let order = gt.resolve_influence(&c, &w());
        assert_eq!(order[0], PlayerId::new(1, 0));
    }

    #[test]
    fn fatigue_lowers_influence() {
        let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
        let c = InfluenceContext {
            point: (0, 0),
            agents: vec![agent(1, 100, 0, 1000), agent(2, 200, 0, 0)],
            skill: SkillKind::Tackling,
        };
        let order = gt.resolve_influence(&c, &w());
        assert_eq!(order[0], PlayerId::new(2, 0));
    }
}

#[cfg(test)]
mod influence_determinism_tests {
    use super::*;
    use pcfm_core::ids::PlayerId;

    fn w() -> InfluenceWeights {
        InfluenceWeights {
            w_space: 100,
            w_skill: 40,
            w_cost: 20,
            w_red: 50,
            space_threshold_cm: 2000,
        }
    }

    fn agent(slot: u32, x: i32, y: i32) -> InfluenceAgent {
        InfluenceAgent {
            id: PlayerId::new(slot, 0),
            position: (x, y),
            fatigue_milli: 0,
        }
    }

    #[test]
    fn same_input_same_order() {
        let mut a = GroundTruth::new(42, MatchId::new(1, 0));
        let mut b = GroundTruth::new(42, MatchId::new(1, 0));
        let c = InfluenceContext {
            point: (0, 0),
            agents: vec![agent(1, 100, 0), agent(2, 100, 0), agent(3, 900, 0)],
            skill: SkillKind::Tackling,
        };
        assert_eq!(a.resolve_influence(&c, &w()), b.resolve_influence(&c, &w()));
    }

    #[test]
    fn vec_order_does_not_matter() {
        let mut a = GroundTruth::new(42, MatchId::new(1, 0));
        let mut b = GroundTruth::new(42, MatchId::new(1, 0));
        let c1 = InfluenceContext {
            point: (0, 0),
            agents: vec![agent(1, 100, 0), agent(2, 900, 0)],
            skill: SkillKind::Tackling,
        };
        let c2 = InfluenceContext {
            point: (0, 0),
            agents: vec![agent(2, 900, 0), agent(1, 100, 0)],
            skill: SkillKind::Tackling,
        };
        assert_eq!(
            a.resolve_influence(&c1, &w()),
            b.resolve_influence(&c2, &w())
        );
    }
}
