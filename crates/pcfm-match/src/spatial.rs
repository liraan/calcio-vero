//! Spatial state for the positional engine (ADR-019, slice A).
//!
//! The position is **match state**, not player identity: it lives in
//! `pcfm-match`, not in `pcfm-model`. See
//! `docs/context/L0-POSITIONAL-SLICE.md` §3.1.
//!
//! Coordinates are integer centimetres (ADR-019 D1: determinism). The pitch
//! is 10500 × 6800 cm by default. The origin is the home team's goal line,
//! bottom-left: `x` grows toward the away goal, `y` toward the far touchline.

use std::collections::BTreeMap;

use pcfm_core::ids::PlayerId;
use pcfm_model::{Pitch, Squad};

/// A player's position, velocity and heading on the pitch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentState {
    /// Position in centimetres, `(x, y)`.
    pub position: (i32, i32),
    /// Velocity in centimetres per tick, `(vx, vy)`.
    pub velocity: (i16, i16),
    /// Heading in degrees, `0..=359`, `0` = north (positive `y`), clockwise.
    pub heading: u16,
}

/// The spatial state of a match: every agent, plus the ball.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialState {
    agents: BTreeMap<PlayerId, AgentState>,
    ball: (i32, i32),
    /// The agent carrying the ball, if any.
    carrier: Option<PlayerId>,
}

/// The 4-4-2 grid, as `(x percent, y percent)` per slot.
///
/// Slot order: GK, LB, CB, CB, RB, LM, CM, CM, RM, ST, ST. It matches
/// `Squad.players` **by index**. The formation parser is deferred (D3): a
/// non-4-4-2 formation falls back to this grid.
const GRID_4_4_2: [(i32, i32); 11] = [
    (5, 50),
    (25, 15),
    (25, 38),
    (25, 62),
    (25, 85),
    (50, 15),
    (50, 38),
    (50, 62),
    (50, 85),
    (75, 38),
    (75, 62),
];

impl SpatialState {
    /// Builds the initial spatial state from a pitch and two squads.
    ///
    /// Home players are placed on the 4-4-2 grid; away players are the same
    /// grid mirrored on the `x` axis. The ball starts at the centre.
    ///
    /// # Panics
    ///
    /// Panics if a squad does not have exactly 11 players. `Squad` enforces
    /// this at construction, so the panic is unreachable in practice.
    #[must_use]
    pub fn from_squads(pitch: &Pitch, home: &Squad, away: &Squad) -> Self {
        let length = i32::from(pitch.length_m) * 100;
        let width = i32::from(pitch.width_m) * 100;
        let mut agents = BTreeMap::new();
        for (i, player) in home.players.iter().enumerate() {
            let (px, py) = GRID_4_4_2[i];
            agents.insert(
                player.id,
                AgentState {
                    position: (length * px / 100, width * py / 100),
                    velocity: (0, 0),
                    heading: 90,
                },
            );
        }
        for (i, player) in away.players.iter().enumerate() {
            let (px, py) = GRID_4_4_2[i];
            agents.insert(
                player.id,
                AgentState {
                    position: (length - length * px / 100, width * py / 100),
                    velocity: (0, 0),
                    heading: 270,
                },
            );
        }
        Self {
            agents,
            ball: (length / 2, width / 2),
            carrier: None,
        }
    }

    /// The state of one agent, or `None` if the id is not on the pitch.
    #[must_use]
    pub fn agent(&self, id: PlayerId) -> Option<&AgentState> {
        self.agents.get(&id)
    }

    /// The ball position, in centimetres.
    #[must_use]
    pub const fn ball(&self) -> (i32, i32) {
        self.ball
    }

    /// The agent carrying the ball, if any.
    #[must_use]
    pub const fn carrier(&self) -> Option<PlayerId> {
        self.carrier
    }

    /// Sets the carrier. `None` means the ball is loose.
    pub const fn set_carrier(&mut self, carrier: Option<PlayerId>) {
        self.carrier = carrier;
    }

    /// The number of agents on the pitch.
    #[must_use]
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    /// Returns `true` if no agent is on the pitch.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Iterates over every `(PlayerId, AgentState)` pair, in `PlayerId` order.
    pub fn iter(&self) -> impl Iterator<Item = (&PlayerId, &AgentState)> {
        self.agents.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcfm_core::ids::SquadId;
    use pcfm_model::{Foot, PhysicalStateMinimal, Player, PsychologicalStateMinimal, RoleId};

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
            pcfm_model::FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn make_state() -> SpatialState {
        SpatialState::from_squads(&Pitch::default(), &make_squad(0), &make_squad(100))
    }

    #[test]
    fn twenty_two_agents_on_the_pitch() {
        assert_eq!(make_state().len(), 22);
    }

    #[test]
    fn same_input_same_output() {
        let a = make_state();
        let b = make_state();
        assert_eq!(a, b);
    }

    #[test]
    fn ball_at_centre() {
        let s = make_state();
        assert_eq!(s.ball(), (10500 / 2, 6800 / 2));
    }

    #[test]
    fn goalkeepers_at_the_extremes() {
        let s = make_state();
        let home_gk = s.agent(PlayerId::new(0, 0)).unwrap();
        let away_gk = s.agent(PlayerId::new(100, 0)).unwrap();
        assert_eq!(home_gk.position.0, 10500 * 5 / 100);
        assert_eq!(away_gk.position.0, 10500 - 10500 * 5 / 100);
    }

    #[test]
    fn every_position_is_inside_the_pitch() {
        let s = make_state();
        for (_, a) in s.iter() {
            assert!((0..=10500).contains(&a.position.0), "x out: {a:?}");
            assert!((0..=6800).contains(&a.position.1), "y out: {a:?}");
        }
    }

    #[test]
    fn away_is_mirrored_on_x() {
        let s = make_state();
        let home_lb = s.agent(PlayerId::new(1, 0)).unwrap();
        let away_lb = s.agent(PlayerId::new(101, 0)).unwrap();
        assert_eq!(home_lb.position.1, away_lb.position.1);
        assert_eq!(home_lb.position.0 + away_lb.position.0, 10500);
    }
}
