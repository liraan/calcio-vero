//! Internal facts emitted by the engine, classified into `MatchEvent`s by
//! the narrator (T1, T2 in `SESSION-STATE.md`).
//!
//! A fact is Plane A -> Plane B: it carries the cause (who, where, under
//! what condition), not the narrative. It is internal to `pcfm-match`:
//! not part of `MatchResult`, not serialised, not in `pcfm-stats`.
//! A new fact species is a `SIM_VERSION` bump if it changes which events
//! are emitted (CHANGELOG-SIM.md, "When to bump", point 2).

use pcfm_core::ids::{ActorId, PlayerId};
use pcfm_stats::EventType;

/// A fact the engine produces in a tick. The narrator classifies it into
/// zero or more `MatchEvent`s.
#[derive(Debug, Clone)]
pub enum Fact {
    /// Two agents contest the ball. The narrator emits `Duel` (always) and
    /// `Tackle` (when the defender wins).
    Contact {
        /// The agent who had the ball.
        attacker: PlayerId,
        /// The agent contesting it.
        defender: PlayerId,
        /// The agent who won.
        winner: PlayerId,
    },
    /// An action sent the ball along a trajectory. The narrator emits
    /// `Shot` (always) and `Goal` (when `goal` is true).
    Trajectory {
        /// The agent who produced it.
        actor: PlayerId,
        /// Whether a goal resulted.
        goal: bool,
    },
}

/// One classified event: a `MatchEvent` in everything but the header
/// (id, match, half, tick) the emitter adds. The classification is pure:
/// it depends only on the fact, so it is testable and deterministic.
#[derive(Debug, Clone)]
pub struct Classified {
    /// The actor of the event.
    pub actor: Option<ActorId>,
    /// The event type.
    pub event_type: EventType,
    /// The event fields.
    pub fields: serde_json::Value,
}

/// Classifies a fact into events, in emission order. The order is part of
/// the contract: `Duel` before `Tackle`, `Shot` before `Goal`.
pub fn classify(fact: &Fact) -> Vec<Classified> {
    match fact {
        Fact::Contact {
            attacker,
            defender,
            winner,
        } => {
            let mut out = Vec::with_capacity(2);
            out.push(Classified {
                actor: Some(ActorId::Player(*attacker)),
                event_type: EventType::Duel,
                fields: serde_json::json!({ "winner_slot": winner.slot() }),
            });
            if winner != attacker {
                out.push(Classified {
                    actor: Some(ActorId::Player(*defender)),
                    event_type: EventType::Tackle,
                    fields: serde_json::json!({
                        "from": attacker.slot(),
                        "to": defender.slot(),
                    }),
                });
            }
            out
        }
        Fact::Trajectory { actor, goal } => {
            let mut out = Vec::with_capacity(2);
            out.push(Classified {
                actor: Some(ActorId::Player(*actor)),
                event_type: EventType::Shot,
                fields: serde_json::json!({ "goal": goal }),
            });
            if *goal {
                out.push(Classified {
                    actor: Some(ActorId::Player(*actor)),
                    event_type: EventType::Goal,
                    fields: serde_json::json!({}),
                });
            }
            out
        }
    }
}
