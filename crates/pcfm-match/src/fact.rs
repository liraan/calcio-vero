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
    /// Two agents contest the ball. Emits `Duel` (always) and `Tackle`
    /// (when the defender wins).
    Contact {
        attacker: PlayerId,
        defender: PlayerId,
        winner: PlayerId,
    },
    /// An action sent the ball along a trajectory.
    Trajectory {
        actor: PlayerId,
        kind: TrajectoryKind,
        /// `Pass`: whether the pass was completed.
        /// `Shot`: whether the shot was a goal (unused; `Goal` comes from `LineCrossing`).
        success: bool,
        /// `Shot` only: whether the shot was on target. Ignored for `Pass`.
        on_target: bool,
    },
    /// The ball crossed the goal line.
    GoalLineCrossing {
        actor: Option<ActorId>,
        point: (i32, i32, i32),
    },
}

/// The kind of trajectory a `Fact::Trajectory` carries.
#[derive(Debug, Clone, Copy)]
pub enum TrajectoryKind {
    Pass,
    Shot,
}

/// One classified event.
#[derive(Debug, Clone)]
pub struct Classified {
    pub actor: Option<ActorId>,
    pub event_type: EventType,
    pub fields: serde_json::Value,
}

/// Classifies a fact into events, in emission order.
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
        Fact::Trajectory {
            actor,
            kind,
            success,
            on_target,
        } => match kind {
            TrajectoryKind::Pass => vec![Classified {
                actor: Some(ActorId::Player(*actor)),
                event_type: EventType::Pass,
                fields: serde_json::json!({ "complete": success }),
            }],
            TrajectoryKind::Shot => vec![Classified {
                actor: Some(ActorId::Player(*actor)),
                event_type: EventType::Shot,
                fields: serde_json::json!({ "goal": success, "on_target": on_target }),
            }],
        },
        Fact::GoalLineCrossing { actor, point } => vec![Classified {
            actor: *actor,
            event_type: EventType::Goal,
            fields: serde_json::json!({
                "x": point.0,
                "y": point.1,
                "z": point.2,
            }),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcfm_core::ids::PlayerId;

    fn p(slot: u32) -> PlayerId {
        PlayerId::new(slot, 0)
    }

    #[test]
    fn contact_winner_is_attacker_emits_only_duel() {
        let c = classify(&Fact::Contact {
            attacker: p(1),
            defender: p(2),
            winner: p(1),
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Duel);
    }

    #[test]
    fn contact_winner_is_defender_emits_duel_then_tackle() {
        let c = classify(&Fact::Contact {
            attacker: p(1),
            defender: p(2),
            winner: p(2),
        });
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].event_type, EventType::Duel);
        assert_eq!(c[1].event_type, EventType::Tackle);
    }

    #[test]
    fn pass_trajectory_emits_pass_with_complete_field() {
        let c = classify(&Fact::Trajectory {
            actor: p(1),
            kind: TrajectoryKind::Pass,
            success: true,
            on_target: false,
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Pass);
        assert_eq!(c[0].fields["complete"], true);
    }

    #[test]
    fn shot_trajectory_emits_shot_with_goal_and_on_target() {
        let c = classify(&Fact::Trajectory {
            actor: p(1),
            kind: TrajectoryKind::Shot,
            success: false,
            on_target: true,
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Shot);
        assert_eq!(c[0].fields["goal"], false);
        assert_eq!(c[0].fields["on_target"], true);
    }

    #[test]
    fn goal_line_crossing_emits_goal_with_position() {
        let c = classify(&Fact::GoalLineCrossing {
            actor: Some(ActorId::Player(p(1))),
            point: (10500, 3400, 0),
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Goal);
        assert_eq!(c[0].fields["x"], 10500);
    }
}
