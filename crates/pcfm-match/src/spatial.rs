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

// Note (2026-11-02): `velocity` is `(0, 0)` and `heading` is never read.
// They are reserved fields for the per-agent slice, not part of any model
// today. They are the two declared limits of the influence formula
// (`03-entities-minimal` §2.9): heading (orientation) and time to arrive
// (velocity). See OI-25 and OI-29.

/// The state of the ball (MD-25). `Possessed` means at the carrier's feet
/// (velocity zero, no physics). `InFlight` means travelling with its own
/// velocity. `Loose` means free and at rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallState {
    /// At the carrier's feet.
    Possessed,
    /// Travelling with a velocity.
    InFlight,
    /// Free and at rest.
    Loose,
}
/// Gravity in centimetres per tick squared (MD-18). Derived: g ≈ 9.8 m/s²
/// at 10 ticks/s rounds to ~10 cm/tick². A different value is a bug, not
/// an arbitrary parameter.
const BALL_GRAVITY: i16 = 10;

/// Horizontal friction numerator per tick (MD-18). Arbitrary, declared.
const BALL_FRICTION_NUM: i32 = 990;

/// Horizontal friction denominator per tick (MD-18). Arbitrary, declared.
const BALL_FRICTION_DEN: i32 = 1000;

/// Speed below which a grounded ball stops, in cm/tick (MD-18). Arbitrary,
/// declared.
const BALL_STOP_THRESHOLD: i16 = 1;

/// The spatial state of a match: every agent, plus the ball.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialState {
    agents: BTreeMap<PlayerId, AgentState>,
    ball: BallState,
    ball_position: (i32, i32, i32),
    ball_velocity: (i16, i16, i16),
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
    /// grid mirrored on the `x` axis. The ball starts at the centre, loose.
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
            ball: BallState::Loose,
            ball_position: (length / 2, width / 2, 0),
            ball_velocity: (0, 0, 0),
            carrier: None,
        }
    }

    /// The state of one agent, or `None` if the id is not on the pitch.
    #[must_use]
    pub fn agent(&self, id: PlayerId) -> Option<&AgentState> {
        self.agents.get(&id)
    }

    /// The state of one agent, mutably, or `None` if the id is not on the pitch.
    ///
    /// The only mutable door for an agent's position: the simulation must
    /// not reach into the map directly (`00-foundations` §3).
    pub fn agent_mut(&mut self, id: PlayerId) -> Option<&mut AgentState> {
        self.agents.get_mut(&id)
    }

    /// Sets the position of an agent. Returns `false` if the id is not on
    /// the pitch (nothing is changed).
    ///
    /// No clamping here: the pitch bounds belong to the movement model
    /// (F.3), not to the primitive. A target derived from the ball can fall
    /// outside the pitch; what the agent does then is a model decision
    /// declared in F.3, not hidden in a setter.
    pub fn set_position(&mut self, id: PlayerId, position: (i32, i32)) -> bool {
        let Some(agent) = self.agents.get_mut(&id) else {
            return false;
        };
        agent.position = position;
        true
    }

    /// The ball position, in centimetres, 2D `(x, y)`.
    #[must_use]
    pub const fn ball(&self) -> (i32, i32) {
        (self.ball_position.0, self.ball_position.1)
    }

    /// The ball position, in centimetres, 3D `(x, y, z)`.
    #[must_use]
    pub const fn ball_3d(&self) -> (i32, i32, i32) {
        self.ball_position
    }

    /// The ball state.
    #[must_use]
    pub const fn ball_state(&self) -> BallState {
        self.ball
    }

    /// Puts the ball in flight from `position` with `velocity`. A no-op if
    /// the ball is possessed (physics does not run on a possessed ball,
    /// MD-25): the caller must release the carrier first.
    pub const fn set_ball_in_flight(
        &mut self,
        position: (i32, i32, i32),
        velocity: (i16, i16, i16),
    ) {
        if matches!(self.ball, BallState::Possessed) {
            return;
        }
        self.ball = BallState::InFlight;
        self.ball_position = position;
        self.ball_velocity = velocity;
    }

    /// The ball velocity, in centimetres per tick, 3D `(vx, vy, vz)`.
    /// Zero when the ball is not in flight.
    #[must_use]
    pub const fn ball_velocity(&self) -> (i16, i16, i16) {
        self.ball_velocity
    }

    /// Advances the ball one tick if it is in flight (MD-17/18).
    ///
    /// Applies gravity to `vz`, gravity to `z`, friction to the horizontal
    /// velocity, and the stop threshold. `Possessed` and `Loose` are
    /// untouched: only an in-flight ball moves. The constants are declared
    /// (MD-24): gravity is ~10 cm/tick² (derived from g ≈ 9.8 m/s² at the
    /// tick rate), friction `f = 990/1000` per tick (arbitrary, declared),
    /// stop threshold 1 cm/tick (arbitrary, declared).
    pub fn advance_ball(&mut self) {
        let BallState::InFlight = self.ball else {
            return;
        };
        let (mut vx, mut vy, mut vz) = self.ball_velocity;
        let (mut x, mut y, mut z) = self.ball_position;
        vz = vz.saturating_sub(BALL_GRAVITY);
        vx = i16::try_from(i32::from(vx) * BALL_FRICTION_NUM / BALL_FRICTION_DEN).unwrap_or(vx);
        vy = i16::try_from(i32::from(vy) * BALL_FRICTION_NUM / BALL_FRICTION_DEN).unwrap_or(vy);
        x = x.saturating_add(i32::from(vx));
        y = y.saturating_add(i32::from(vy));
        z = z.saturating_add(i32::from(vz));
        if z < 0 {
            z = 0;
            vz = 0;
        }
        self.ball_position = (x, y, z);
        self.ball_velocity = (vx, vy, vz);
        if z == 0 && vx.abs() <= BALL_STOP_THRESHOLD && vy.abs() <= BALL_STOP_THRESHOLD {
            self.ball = BallState::Loose;
            self.ball_velocity = (0, 0, 0);
        }
    }
    /// The agent carrying the ball, if any.
    #[must_use]
    pub const fn carrier(&self) -> Option<PlayerId> {
        self.carrier
    }

    /// Sets the carrier. `None` means the ball is loose.
    pub const fn set_carrier(&mut self, carrier: Option<PlayerId>) {
        self.carrier = carrier;
        if carrier.is_some() {
            self.ball = BallState::Possessed;
            self.ball_velocity = (0, 0, 0);
        } else if matches!(self.ball, BallState::Possessed) {
            self.ball = BallState::Loose;
        }
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
        assert_eq!(s.ball_3d(), (10500 / 2, 6800 / 2, 0));
        assert_eq!(s.ball_state(), BallState::Loose);
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

    #[test]
    fn set_position_moves_the_agent() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        assert!(s.set_position(id, (1000, 2000)));
        assert_eq!(s.agent(id).unwrap().position, (1000, 2000));
    }

    #[test]
    fn set_position_unknown_id_changes_nothing() {
        let mut s = make_state();
        let before = s.len();
        assert!(!s.set_position(PlayerId::new(999, 0), (1, 1)));
        assert_eq!(s.len(), before);
    }

    #[test]
    fn agent_mut_reaches_the_position() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.agent_mut(id).unwrap().position = (500, 500);
        assert_eq!(s.agent(id).unwrap().position, (500, 500));
    }

    #[test]
    fn set_carrier_sets_possessed() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.set_carrier(Some(id));
        assert_eq!(s.ball_state(), BallState::Possessed);
        assert_eq!(s.ball_velocity(), (0, 0, 0));
    }

    #[test]
    fn set_ball_in_flight_is_a_noop_when_possessed() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.set_carrier(Some(id));
        let before = s.ball_3d();
        s.set_ball_in_flight((100, 200, 50), (10, 5, 20));
        assert_eq!(s.ball_3d(), before);
        assert_eq!(s.ball_state(), BallState::Possessed);
    }

    #[test]
    fn set_ball_in_flight_works_when_loose() {
        let mut s = make_state();
        s.set_ball_in_flight((100, 200, 50), (10, 5, 20));
        assert_eq!(s.ball_3d(), (100, 200, 50));
        assert_eq!(s.ball_state(), BallState::InFlight);
        assert_eq!(s.ball_velocity(), (10, 5, 20));
    }

    #[test]
    fn advance_ball_is_a_noop_when_possessed() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.set_carrier(Some(id));
        let before = s.ball_3d();
        s.advance_ball();
        assert_eq!(s.ball_3d(), before);
        assert_eq!(s.ball_state(), BallState::Possessed);
    }

    #[test]
    fn advance_ball_moves_z_down_and_x_forward_under_gravity() {
        let mut s = make_state();
        s.set_ball_in_flight((5000, 3400, 200), (100, 0, 0));
        s.advance_ball();
        // gravity pulls z down by 10 (vz 0 -> -10), x advances by ~99 (friction 0.99)
        let (x, y, z) = s.ball_3d();
        assert!(z < 200, "z should fall under gravity, got {z}");
        assert!(x > 5000, "x should advance, got {x}");
        assert_eq!(y, 3400);
        let (_, _, vz) = s.ball_velocity();
        assert!(vz < 0, "vz should be negative after gravity, got {vz}");
    }

    #[test]
    fn advance_ball_lands_and_stops() {
        let mut s = make_state();
        s.set_ball_in_flight((5000, 3400, 0), (0, 0, 0));
        // With zero velocity and zero height, the ball is already at rest
        // and the first tick should mark it Loose (threshold).
        s.advance_ball();
        assert_eq!(s.ball_3d(), (5000, 3400, 0));
        assert_eq!(s.ball_state(), BallState::Loose);
    }
}
