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

use crate::{Characteristic, Datum, Formula};

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
    /// Weight of the fatigue term (ADR-030; prototype, not calibrated).
    pub fatigue_coefficient: f64,
    /// Floor of the fatigue modifier: 1 - c*fatigue is clamped here.
    pub fatigue_floor: f64,
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
    /// `a = accel_ref * explosive_force * (weight_ref / weight) * max(floor, 1 - c*fatigue)`
    ///
    /// The fatigue modifier is ADR-030 (one scalar, multiplicative with a
    /// floor). It is identity at fatigue = 0, so the golden is unchanged
    /// when the match never accumulates fatigue (F.7, no producer yet).
    #[must_use]
    pub fn formula(&self) -> Formula {
        Formula::Product(vec![
            Formula::Const(self.accel_ref_cm_per_tick2),
            Formula::Term(Characteristic::ExplosiveForce),
            Formula::Ratio(
                Box::new(Formula::Const(self.weight_ref_kg)),
                Box::new(Formula::Datum(Datum::Weight)),
            ),
            Formula::Clamp(
                Box::new(Formula::Sum(vec![
                    Formula::Const(1.0),
                    Formula::Product(vec![
                        Formula::Const(-self.fatigue_coefficient),
                        Formula::Datum(Datum::Fatigue),
                    ]),
                ])),
                self.fatigue_floor,
                1.0,
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
        let chars = f.declared_characteristics();
        let data = f.declared_data();
        assert_eq!(chars, vec![Characteristic::ExplosiveForce]);
        assert_eq!(data, vec![Datum::Weight, Datum::Fatigue]);
    }

    #[test]
    fn movement_formula_validates() {
        let f = MovementBalance::load().formula();
        assert!(f.validate().is_ok());
    }
}
