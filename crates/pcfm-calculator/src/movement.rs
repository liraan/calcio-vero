//! The movement formula: the first formula of the new engine (F.7).
//!
//! The phenomenon, in a real match: a stronger player accelerates more,
//! a heavier one less, and the two effects compensate in ratio, they do
//! not add up. The grammar is `product`, `ratio`, `clamp` (L0.11 §4.5).
//!
//! The formula, in the grammar:
//!
//! `a = accel_ref * explosive_force * (weight_ref / weight)`
//!
//! The constants come from `balance/movement.json`, which is embedded at
//! compile time: the engine performs no I/O. A balance change is a
//! `SIM_VERSION` bump (CHANGELOG-SIM).

use serde::Deserialize;

use crate::{Characteristic, Formula};

/// The embedded balance file.
const MOVEMENT_JSON: &str = include_str!("../../../balance/movement.json");

/// The constants of the movement formula.
#[derive(Debug, Clone, Deserialize)]
pub struct MovementBalance {
    /// Reference acceleration, in cm/tick².
    pub accel_ref_cm_per_tick2: f64,
    /// Reference top speed, in cm/tick.
    pub v_max_ref_cm_per_tick: f64,
    /// Reference weight, in kg.
    pub weight_ref_kg: f64,
    /// Reference height, in cm.
    pub height_ref_cm: f64,
}

impl MovementBalance {
    /// Loads the embedded balance file.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_movement_balance_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(MOVEMENT_JSON).expect("embedded movement balance is valid JSON")
    }

    /// The movement formula, built from the balance values.
    ///
    /// `a = accel_ref * explosive_force * (weight_ref / weight)`.
    #[must_use]
    pub fn formula(&self) -> Formula {
        Formula::Product(vec![
            Formula::Const(self.accel_ref_cm_per_tick2),
            Formula::Term(Characteristic::ExplosiveForce),
            Formula::Ratio(
                Box::new(Formula::Const(self.weight_ref_kg)),
                Box::new(Formula::Term(Characteristic::Weight)),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_movement_balance_is_valid() {
        let b = MovementBalance::load();
        assert!(b.accel_ref_cm_per_tick2 > 0.0);
        assert!(b.v_max_ref_cm_per_tick > 0.0);
        assert!(b.weight_ref_kg > 0.0);
        assert!(b.height_ref_cm > 0.0);
    }

    #[test]
    fn movement_formula_declares_force_and_weight() {
        let f = MovementBalance::load().formula();
        let terms = f.declared_terms();
        assert!(terms.contains(&Characteristic::ExplosiveForce));
        assert!(terms.contains(&Characteristic::Weight));
        assert_eq!(terms.len(), 2, "only force and weight: {terms:?}");
    }

    #[test]
    fn movement_formula_validates() {
        let f = MovementBalance::load().formula();
        assert!(f.validate().is_ok());
    }
}
