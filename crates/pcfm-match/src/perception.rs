//! Perception of an agent: what it sees, from `SpatialState`.
//!
//! Plane B: an agent sees the ball and the distance to the nearest agents,
//! not the full map. The perception is a local computation; it is not stored
//! in the event (a later slice adds `PositionSnapshot`).
//!
//! See `docs/context/L0-POSITIONAL-SLICE.md` §3.3.

use pcfm_core::ids::PlayerId;
use pcfm_model::Squad;

use crate::spatial::{AgentState, SpatialState};

/// Squared Euclidean distance between two positions, in centimetres squared.
///
/// Squared, not the square root: the comparison order is the same, and it
/// avoids a non-deterministic `sqrt`.
#[must_use]
pub const fn distance_squared(a: (i32, i32), b: (i32, i32)) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    dx * dx + dy * dy
}

/// One entry of a nearest-neighbour list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Neighbour {
    /// The other agent.
    pub id: PlayerId,
    /// Squared distance, in centimetres squared.
    pub distance_squared: i64,
}

/// The `n` nearest agents of a squad to `from`, sorted by squared distance
/// then by `PlayerId` (deterministic tie-break).
///
/// Agents not on the pitch are skipped.
#[must_use]
pub fn nearest_teammates(
    spatial: &SpatialState,
    squad: &Squad,
    from: PlayerId,
    n: usize,
) -> Vec<Neighbour> {
    let Some(origin) = spatial.agent(from).map(|a: &AgentState| a.position) else {
        return Vec::new();
    };
    let mut list: Vec<Neighbour> = squad
        .players
        .iter()
        .filter(|p| p.id != from)
        .filter_map(|p| {
            spatial.agent(p.id).map(|a| Neighbour {
                id: p.id,
                distance_squared: distance_squared(origin, a.position),
            })
        })
        .collect();
    list.sort_by(|a, b| {
        a.distance_squared
            .cmp(&b.distance_squared)
            .then(a.id.slot().cmp(&b.id.slot()))
    });
    list.truncate(n);
    list
}

/// The `n` nearest opponents to `from`, sorted by squared distance then by
/// `PlayerId`.
#[must_use]
pub fn nearest_opponents(
    spatial: &SpatialState,
    opponents: &Squad,
    from: PlayerId,
    n: usize,
) -> Vec<Neighbour> {
    nearest_teammates(spatial, opponents, from, n)
}

/// The agent nearest to a point, tie-broken by `PlayerId`. `None` if the
/// spatial state is empty.
#[must_use]
pub fn nearest_to_point(spatial: &SpatialState, point: (i32, i32)) -> Option<PlayerId> {
    spatial
        .iter()
        .map(|(id, a)| (distance_squared(point, a.position), *id))
        .min_by(|a, b| a.0.cmp(&b.0).then(a.1.slot().cmp(&b.1.slot())))
        .map(|(_, id)| id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcfm_model::Pitch;

    use pcfm_core::ids::SquadId;
    use pcfm_model::{
        Foot, FormationId, PhysicalStateMinimal, Player, PsychologicalStateMinimal, RoleId,
    };

    fn make_squad(slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("P{}", slot_base + i),
                    RoleId::new("midfielder").unwrap(),
                    Foot::Right,
                    180,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(slot_base, 0),
            players,
            vec![],
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn distance_squared_is_symmetric() {
        let a = (0, 0);
        let b = (3, 4);
        assert_eq!(distance_squared(a, b), 25);
        assert_eq!(distance_squared(b, a), 25);
    }

    #[test]
    fn nearest_teammates_skips_self() {
        let (home, away) = (make_squad(0), make_squad(100));
        let s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let list = nearest_teammates(&s, &home, PlayerId::new(0, 0), 3);
        assert_eq!(list.len(), 3);
        assert!(!list.iter().any(|n| n.id == PlayerId::new(0, 0)));
    }

    #[test]
    fn nearest_teammates_is_sorted() {
        let (home, away) = (make_squad(0), make_squad(100));
        let s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let list = nearest_teammates(&s, &home, PlayerId::new(0, 0), 10);
        for w in list.windows(2) {
            assert!(w[0].distance_squared <= w[1].distance_squared);
        }
    }

    #[test]
    fn nearest_to_point_is_deterministic() {
        let (home, away) = (make_squad(0), make_squad(100));
        let s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let a = nearest_to_point(&s, s.ball());
        let b = nearest_to_point(&s, s.ball());
        assert_eq!(a, b);
    }
}
