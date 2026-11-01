//! Spatial state definitions for the layered match model.
//!
//! This layer does not define the rules of the match. It provides the
//! authoritative world-space view that the simulation can drive above L0.
//! The important rule is: the renderer reads this layer; it does not invent it.

use serde::{Deserialize, Serialize};

/// A 2D vector used for positions and velocity.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
}

/// The state of the ball in world space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BallState {
    /// Ball position on the pitch.
    pub position: Vec2,
    /// Ball velocity.
    pub velocity: Vec2,
    /// Current owner, if any.
    pub owner: Option<u32>,
    /// Player who last touched the ball.
    pub last_touch: Option<u32>,
}

/// A per-player spatial state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSpatialState {
    /// Player identifier.
    pub player_id: u32,
    /// Current world-space position.
    pub position: Vec2,
    /// Current velocity.
    pub velocity: Vec2,
    /// Heading in radians or arbitrary angular measure, depending on the
    /// rendering backend.
    pub heading: f32,
    /// The player role as interpreted by the tactical layer.
    pub role: &'static str,
    /// Current zone assignment for the player.
    pub zone: &'static str,
    /// Spatial bias / spacing effect in the current structure.
    pub spacing_bias: f32,
}

/// A compact representation of how a team is shaped on the pitch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamShapeState {
    /// Total width of the shape.
    pub width: f32,
    /// Total depth of the shape.
    pub depth: f32,
    /// Compactness of the block.
    pub compactness: f32,
    /// Mean line height of the team shape.
    pub line_height: f32,
    /// Left anchor position.
    pub left_anchor: f32,
    /// Right anchor position.
    pub right_anchor: f32,
    /// Pressing line threshold.
    pub press_line: f32,
}

/// A pressure point in the relevant tactical space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressurePoint {
    /// Position of the pressure point.
    pub position: Vec2,
    /// Intensity of the pressure at that point.
    pub intensity: f32,
}

/// A compact pressure map used by tactical-state evaluation and rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressureMap {
    /// Pressure points for the home team.
    pub home: Vec<PressurePoint>,
    /// Pressure points for the away team.
    pub away: Vec<PressurePoint>,
}

/// The full world-space state used by the simulation and the renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialState {
    /// Ball state.
    pub ball: BallState,
    /// All players in the match.
    pub players: Vec<PlayerSpatialState>,
    /// Home team shape summary.
    pub home_shape: TeamShapeState,
    /// Away team shape summary.
    pub away_shape: TeamShapeState,
    /// Pressure map summary.
    pub pressure: PressureMap,
}
