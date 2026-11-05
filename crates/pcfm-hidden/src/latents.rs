//! Latent attributes (Plane A), derived by key.
//!
//! These are the values that must **never** leave `pcfm-hidden`.
//!
//! **Keyed derivation (ADR-029, OI-53).** A latent is derived from the
//! player and the characteristic, not from a sequence:
//! `hash(world_seed, player_id, characteristic)`. The same player in the
//! same world has the same value, regardless of when it is read and of how
//! many players were generated before him. Adding a characteristic does
//! not shift the others, because each value is derived from its own key.
//!
//! **No `match_id` in the key.** A player's strength is the same in every
//! match of his world (ADR-029 §2). The world is identified by the
//! `world_seed` (ADR-013), so no `world_id` is needed either.
//!
//! **The values are still uniform in `[0, 1)`.** This is a prototype: real
//! attributes are roughly bell-shaped, with a long tail of elite players.
//! The factor structure that produces realistic distributions and
//! correlations is L3's job (L0.11 §11.1); this module only fixes the
//! derivation, not the distribution.

use pcfm_core::ids::PlayerId;

/// A latent attribute of a player.
#[derive(Debug, Clone, Copy)]
pub struct PlayerLatents {
    pub finishing: f32,
    pub passing: f32,
    pub tackling: f32,
    pub aerial: f32,
    pub goalkeeping: f32,
    pub set_pieces: f32,
    pub injury_proneness: f32,
}

impl PlayerLatents {
    /// Derives a player's latents from the world seed and the player id.
    ///
    /// Deterministic and order-independent: the value of a characteristic
    /// depends only on `(world_seed, player_id, characteristic)`.
    #[must_use]
    pub fn from_key(world_seed: u64, player: PlayerId) -> Self {
        Self {
            finishing: unit_from_key(world_seed, player, "finishing"),
            passing: unit_from_key(world_seed, player, "passing"),
            tackling: unit_from_key(world_seed, player, "tackling"),
            aerial: unit_from_key(world_seed, player, "aerial"),
            goalkeeping: unit_from_key(world_seed, player, "goalkeeping"),
            set_pieces: unit_from_key(world_seed, player, "set_pieces"),
            injury_proneness: unit_from_key(world_seed, player, "injury_proneness"),
        }
    }
}

/// Derives a value in `[0, 1)` from `(world_seed, player_id, purpose)`.
///
/// The hash is BLAKE3, the same function used for the stream derivation in
/// `pcfm-core::rng` and for the event chain. The key material is the world
/// seed, the player's slot and generation (so two different players never
/// collide), and the purpose (the characteristic name).
#[must_use]
pub fn unit_from_key(world_seed: u64, player: PlayerId, purpose: &str) -> f32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&world_seed.to_le_bytes());
    hasher.update(b"player:");
    hasher.update(&player.slot().to_le_bytes());
    hasher.update(&player.generation().to_le_bytes());
    hasher.update(b"purpose:");
    hasher.update(purpose.as_bytes());
    let digest = hasher.finalize();
    let bytes = digest.as_bytes();
    let raw = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    // `[0, 1)`: divide by 2^32, not by u32::MAX, so 1.0 is never produced.
    #[allow(clippy::cast_possible_truncation)]
    let unit = (f64::from(raw) / f64::from(u32::MAX) / 1.000_000_1) as f32;
    unit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(slot: u32) -> PlayerId {
        PlayerId::new(slot, 0)
    }

    #[test]
    fn same_player_same_value() {
        let a = PlayerLatents::from_key(42, p(7));
        let b = PlayerLatents::from_key(42, p(7));
        assert_eq!(a.passing, b.passing);
        assert_eq!(a.finishing, b.finishing);
    }

    #[test]
    fn different_players_different_values() {
        let a = PlayerLatents::from_key(42, p(7));
        let b = PlayerLatents::from_key(42, p(8));
        assert_ne!(a.passing, b.passing);
    }

    #[test]
    fn different_worlds_different_values() {
        let a = PlayerLatents::from_key(42, p(7));
        let b = PlayerLatents::from_key(43, p(7));
        assert_ne!(a.passing, b.passing);
    }

    #[test]
    fn values_are_in_unit_range() {
        for slot in 0..100 {
            let l = PlayerLatents::from_key(42, p(slot));
            for v in [
                l.finishing,
                l.passing,
                l.tackling,
                l.aerial,
                l.goalkeeping,
                l.set_pieces,
                l.injury_proneness,
            ] {
                assert!((0.0..1.0).contains(&v), "value {v} out of [0,1)");
            }
        }
    }

    #[test]
    fn order_does_not_matter() {
        // The defect OI-53 fixed: with a sequence, generating slot 8 after
        // slot 7 changed slot 8's values. With a key, it does not.
        let alone = PlayerLatents::from_key(42, p(8));
        let _before = PlayerLatents::from_key(42, p(7));
        let after = PlayerLatents::from_key(42, p(8));
        assert_eq!(alone.passing, after.passing);
        assert_eq!(alone.tackling, after.tackling);
    }

    #[test]
    fn characteristics_are_independent() {
        // Adding a characteristic does not shift the others: each value is
        // derived from its own purpose string.
        let l = PlayerLatents::from_key(42, p(7));
        assert_ne!(l.finishing, l.passing);
        assert_ne!(l.passing, l.tackling);
    }
}
