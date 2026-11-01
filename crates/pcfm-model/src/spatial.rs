//! Spatial foundation for the future game model.
//!
//! This module is intentionally written as a forward-looking foundation for the
//! positional and FM-style gameplay layer. It is not a replacement for the
//! current L0 match contract; it is the primitive layer that future systems can
//! build on without reworking the deterministic match model.
//!
//! The goal is to provide a stable geometry vocabulary for:
//! - player positions and movement
//! - ball state
//! - field zones and sides
//! - future tactical, possession, and pressing logic
//!
//! It remains intentionally separate from the hidden/latent Plane A boundary and
//! from the current deterministic match engine.

use serde::{Deserialize, Serialize};

use crate::error::ModelError;

/// A full 2D coordinate in metres on the pitch.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position2D {
    /// Horizontal position in metres.
    pub x: f32,
    /// Vertical position in metres.
    pub y: f32,
}

impl Position2D {
    /// Creates a new position, rejecting non-finite values.
    pub fn try_new(x: f32, y: f32) -> Result<Self, ModelError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(ModelError::InvalidValue {
                field: "spatial.position",
                value: x.max(y),
            });
        }
        Ok(Self { x, y })
    }

    /// Returns the Euclidean distance to another position.
    #[must_use]
    pub fn distance_to(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A 2D velocity in metres per second.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Velocity2D {
    /// Horizontal velocity in metres per second.
    pub x: f32,
    /// Vertical velocity in metres per second.
    pub y: f32,
}

impl Velocity2D {
    /// Creates a new velocity, rejecting non-finite values.
    pub fn try_new(x: f32, y: f32) -> Result<Self, ModelError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(ModelError::InvalidValue {
                field: "spatial.velocity",
                value: x.max(y),
            });
        }
        Ok(Self { x, y })
    }

    /// Returns the speed magnitude.
    #[must_use]
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// An orientation in radians.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Heading(f32);

impl Heading {
    /// Creates a heading from radians.
    pub fn try_new(radians: f32) -> Result<Self, ModelError> {
        if !radians.is_finite() {
            return Err(ModelError::InvalidValue {
                field: "spatial.heading",
                value: radians,
            });
        }
        Ok(Self(radians))
    }

    /// Returns the underlying radians.
    #[must_use]
    pub const fn radians(self) -> f32 {
        self.0
    }
}

/// The side of the pitch a team is attacking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FieldSide {
    /// Team attacks towards the positive x axis.
    Home,
    /// Team attacks towards the negative x axis.
    Away,
}

/// A coarse pitch zone for future tactical reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FieldZone {
    /// Final third of the pitch.
    FinalThird,
    /// Middle third of the pitch.
    MiddleThird,
    /// Defensive third of the pitch.
    DefensiveThird,
}

/// A minimal pose for a player in a future positional layer.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerPose {
    /// Current position on the pitch.
    pub position: Position2D,
    /// Current velocity vector.
    pub velocity: Velocity2D,
    /// Body orientation.
    pub heading: Heading,
}

impl PlayerPose {
    /// Creates a new player pose.
    pub fn try_new(position: Position2D, velocity: Velocity2D, heading: Heading) -> Result<Self, ModelError> {
        Ok(Self {
            position,
            velocity,
            heading,
        })
    }
}

/// A minimal representation of the ball in a future FM-style spatial model.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BallState {
    /// Ball position on the pitch.
    pub position: Position2D,
    /// Ball velocity vector.
    pub velocity: Velocity2D,
}

impl BallState {
    /// Creates a new ball state.
    pub fn try_new(position: Position2D, velocity: Velocity2D) -> Result<Self, ModelError> {
        Ok(Self { position, velocity })
    }
}

/// The geometric bounds of the pitch.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PitchBounds {
    /// Length in metres.
    pub length_m: f32,
    /// Width in metres.
    pub width_m: f32,
}

impl PitchBounds {
    /// Creates new pitch bounds.
    pub fn try_new(length_m: f32, width_m: f32) -> Result<Self, ModelError> {
        if !length_m.is_finite() || !width_m.is_finite() || length_m <= 0.0 || width_m <= 0.0 {
            return Err(ModelError::InvalidValue {
                field: "spatial.pitch_bounds",
                value: length_m.max(width_m),
            });
        }
        Ok(Self { length_m, width_m })
    }

    /// Returns `true` if a position is within the pitch rectangle.
    #[must_use]
    pub fn contains(self, position: Position2D) -> bool {
        position.x >= 0.0
            && position.x <= self.length_m
            && position.y >= 0.0
            && position.y <= self.width_m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_accepts_valid_values() {
        let p = Position2D::try_new(12.0, -7.5).unwrap();
        assert_eq!(p.x, 12.0);
        assert_eq!(p.y, -7.5);
    }

    #[test]
    fn position_rejects_nan() {
        assert!(Position2D::try_new(f32::NAN, 0.0).is_err());
    }

    #[test]
    fn distance_is_calculated() {
        let a = Position2D::try_new(0.0, 0.0).unwrap();
        let b = Position2D::try_new(3.0, 4.0).unwrap();
        assert!((a.distance_to(b) - 5.0).abs() < 1e-5);
    }

    #[test]
    fn velocity_magnitude_works() {
        let v = Velocity2D::try_new(3.0, 4.0).unwrap();
        assert!((v.magnitude() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn pitch_bounds_validate_and_contain() {
        let bounds = PitchBounds::try_new(105.0, 68.0).unwrap();
        assert!(bounds.contains(Position2D::try_new(0.0, 0.0).unwrap()));
        assert!(!bounds.contains(Position2D::try_new(106.0, 0.0).unwrap()));
    }

    #[test]
    fn roundtrip_through_serde() {
        let pose = PlayerPose::try_new(
            Position2D::try_new(5.0, 7.0).unwrap(),
            Velocity2D::try_new(1.5, -0.5).unwrap(),
            Heading::try_new(1.57).unwrap(),
        )
        .unwrap();
        let json = serde_json::to_string(&pose).unwrap();
        let back: PlayerPose = serde_json::from_str(&json).unwrap();
        assert_eq!(pose, back);
    }
}
