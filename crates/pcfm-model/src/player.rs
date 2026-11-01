//! Player and its minimal physical and psychological state.
//!
//! See `specs/L0-match/03-entities-minimal.md` §2.1–§2.3.
//!
//! # Plane separation
//!
//! `Player` contains **no latent attributes**. Latent attributes live in
//! `pcfm-hidden`, indexed by `PlayerId`, and are accessed only via
//! `GroundTruth::resolve_*`. This is the whole point of ADR-003 (option B):
//! the match engine calls `resolve_*` and receives an outcome; it never
//! reads a latent number.
//!
//! # Future-facing spatial state
//!
//! A player may carry a future-facing spatial pose. This is intentionally
//! optional and separate from the current L0 player contract. It gives the
//! project a position/movement foundation without disturbing the deterministic
//! match engine or Plane A access rules.

use pcfm_core::ids::PlayerId;
use serde::{Deserialize, Serialize};

use crate::error::ModelError;
use crate::spatial::PlayerPose;

/// The player's preferred foot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Foot {
    /// Right foot.
    Right,
    /// Left foot.
    Left,
    /// Both feet.
    Both,
}

/// A role identifier.
///
/// At L0, the role is a free-form string. From L3 onward, it will be validated
/// against the registry (`pcfm-registry`). The type is stable: only the
/// validation logic will change.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RoleId(String);

impl RoleId {
    /// Creates a new `RoleId`.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::EmptyRoleId`] if the string is empty.
    pub fn new(id: impl Into<String>) -> Result<Self, ModelError> {
        let id = id.into();
        if id.is_empty() {
            return Err(ModelError::EmptyRoleId);
        }
        Ok(Self(id))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The minimal physical state of a player, as seen by the match engine.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.2.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalStateMinimal {
    /// Freshness, 0.0–1.0.
    pub condition: f32,
    /// Match sharpness, 0.0–1.0.
    pub match_sharpness: f32,
    /// Accumulated fatigue, 0.0–1.0.
    pub fatigue: f32,
}

impl PhysicalStateMinimal {
    /// Creates a new `PhysicalStateMinimal`, validating every field.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::InvalidValue`] if any field is outside [0, 1]
    /// or is NaN.
    pub fn try_new(condition: f32, match_sharpness: f32, fatigue: f32) -> Result<Self, ModelError> {
        check_unit("condition", condition)?;
        check_unit("match_sharpness", match_sharpness)?;
        check_unit("fatigue", fatigue)?;
        Ok(Self {
            condition,
            match_sharpness,
            fatigue,
        })
    }
}

impl Default for PhysicalStateMinimal {
    fn default() -> Self {
        Self {
            condition: 1.0,
            match_sharpness: 1.0,
            fatigue: 0.0,
        }
    }
}

/// The minimal psychological state of a player, as seen by the match engine.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.3.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PsychologicalStateMinimal {
    /// Confidence, 0.0–1.0.
    pub confidence: f32,
    /// Morale, 0.0–1.0.
    pub morale: f32,
    /// Accumulated pressure load, 0.0–1.0.
    pub pressure_load: f32,
}

impl PsychologicalStateMinimal {
    /// Creates a new `PsychologicalStateMinimal`, validating every field.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::InvalidValue`] if any field is outside [0, 1]
    /// or is NaN.
    pub fn try_new(confidence: f32, morale: f32, pressure_load: f32) -> Result<Self, ModelError> {
        check_unit("confidence", confidence)?;
        check_unit("morale", morale)?;
        check_unit("pressure_load", pressure_load)?;
        Ok(Self {
            confidence,
            morale,
            pressure_load,
        })
    }
}

impl Default for PsychologicalStateMinimal {
    fn default() -> Self {
        Self {
            confidence: 0.5,
            morale: 0.5,
            pressure_load: 0.0,
        }
    }
}

/// The minimal player entity, as seen by the match engine.
///
/// See `specs/L0-match/03-entities-minimal.md` §2.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    /// The unique identifier of the player.
    pub id: PlayerId,
    /// The display name.
    pub name: String,
    /// The role identifier.
    pub role: RoleId,
    /// The preferred foot.
    pub preferred_foot: Foot,
    /// The height in centimetres.
    pub height_cm: u16,
    /// The minimal physical state.
    pub physical: PhysicalStateMinimal,
    /// The minimal psychological state.
    pub psychological: PsychologicalStateMinimal,
    /// Optional future-facing positional pose.
    #[serde(default)]
    pub pose: Option<PlayerPose>,
}

impl Player {
    /// Creates a new `Player`, validating every field.
    ///
    /// # Errors
    ///
    /// - [`ModelError::EmptyPlayerName`] if `name` is empty.
    /// - [`ModelError::InvalidHeight`] if `height_cm` is zero.
    pub fn try_new(
        id: PlayerId,
        name: impl Into<String>,
        role: RoleId,
        preferred_foot: Foot,
        height_cm: u16,
        physical: PhysicalStateMinimal,
        psychological: PsychologicalStateMinimal,
    ) -> Result<Self, ModelError> {
        let name = name.into();
        if name.is_empty() {
            return Err(ModelError::EmptyPlayerName);
        }
        if height_cm == 0 {
            return Err(ModelError::InvalidHeight(height_cm));
        }
        Ok(Self {
            id,
            name,
            role,
            preferred_foot,
            height_cm,
            physical,
            psychological,
            pose: None,
        })
    }

    /// Attaches a future-facing positional pose to the player.
    #[must_use]
    pub fn with_pose(mut self, pose: PlayerPose) -> Self {
        self.pose = Some(pose);
        self
    }

    /// Returns `true` when a future-facing positional pose is attached.
    #[must_use]
    pub fn has_pose(&self) -> bool {
        self.pose.is_some()
    }
}

/// Checks that `value` is in [0, 1] and not NaN.
fn check_unit(field: &'static str, value: f32) -> Result<(), ModelError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(ModelError::InvalidValue { field, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial::{Heading, PlayerPose, Position2D, Velocity2D};
    use pcfm_core::ids::PlayerId;

    fn valid_player() -> Player {
        Player::try_new(
            PlayerId::new(1, 0),
            "Alice",
            RoleId::new("striker").unwrap(),
            Foot::Right,
            180,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        )
        .unwrap()
    }

    #[test]
    fn valid_player_is_accepted() {
        let p = valid_player();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.height_cm, 180);
        assert!(!p.has_pose());
    }

    #[test]
    fn player_accepts_future_pose() {
        let p = valid_player().with_pose(PlayerPose::try_new(
            Position2D::try_new(5.0, 7.0).unwrap(),
            Velocity2D::try_new(1.0, 0.5).unwrap(),
            Heading::try_new(0.25).unwrap(),
        ).unwrap());
        assert!(p.has_pose());
    }

    #[test]
    fn empty_role_id_is_rejected() {
        assert!(matches!(RoleId::new(""), Err(ModelError::EmptyRoleId)));
    }

    #[test]
    fn empty_player_name_is_rejected() {
        let result = Player::try_new(
            PlayerId::new(1, 0),
            "",
            RoleId::new("striker").unwrap(),
            Foot::Right,
            180,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        );
        assert!(matches!(result, Err(ModelError::EmptyPlayerName)));
    }

    #[test]
    fn zero_height_is_rejected() {
        let result = Player::try_new(
            PlayerId::new(1, 0),
            "Alice",
            RoleId::new("striker").unwrap(),
            Foot::Right,
            0,
            PhysicalStateMinimal::default(),
            PsychologicalStateMinimal::default(),
        );
        assert!(matches!(result, Err(ModelError::InvalidHeight(0))));
    }

    #[test]
    fn nan_condition_is_rejected() {
        let result = PhysicalStateMinimal::try_new(f32::NAN, 1.0, 0.0);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "condition",
                ..
            })
        ));
    }

    #[test]
    fn out_of_range_condition_is_rejected() {
        let result = PhysicalStateMinimal::try_new(1.5, 1.0, 0.0);
        assert!(matches!(
            result,
            Err(ModelError::InvalidValue {
                field: "condition",
                ..
            })
        ));
    }

    #[test]
    fn player_roundtrips_through_serde_json() {
        let p = valid_player();
        let json = serde_json::to_string(&p).unwrap();
        let back: Player = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }
}
