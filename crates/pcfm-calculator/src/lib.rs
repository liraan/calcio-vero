//! `pcfm-calculator` — the second of the three components of ADR-028 §5.
//!
//! The engine has three components, and each has one responsibility:
//!
//! - the **custodian** (`pcfm-hidden`) holds the hidden performances and
//!   answers one characteristic at a time;
//! - the **calculator** (this crate) applies a formula from
//!   `balance/*.json` to a situation and a player;
//! - the **orchestrator** (`resolve_*`) decides when, with which
//!   situation, for which formula.
//!
//! The calculator does not choose which characteristics to read: the
//! formula declares its terms, and the calculator asks the custodian for
//! exactly those. A formula with an unknown term fails at **load time**,
//! not in the middle of a match. The mechanical test: a mock custodian
//! records every call, and the test verifies the calculator asked for
//! exactly the formula's terms.
//!
//! # The grammar
//!
//! A formula is an expression on a **fixed grammar** of operators
//! (L0.11 §4.5): product, ratio, clamp. The sum is the product with
//! weight 1. A formula cannot invent an operator: a new operator is an
//! amendment to the model, not a formula (L0.11 §4.10). The grammar is
//! fixed because the phenomenon decides it: in a real match a stronger
//! player accelerates more and a heavier one less, and the two effects
//! compensate in ratio, they do not add up. A weighted sum cannot
//! express that.

pub mod movement;
use std::collections::BTreeSet;

use thiserror::Error;

/// A characteristic the custodian knows how to answer.
///
/// This is the catalogue of terms, L0.11 §4.6, made code. A formula may
/// only use terms that appear here; anything else fails at load time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Characteristic {
    /// Explosive force (Plane A, physical).
    ExplosiveForce,
    /// Maximal force (Plane A, physical).
    MaximalForce,
    /// Acceleration (Plane A, physical).
    Acceleration,
    /// Max speed (Plane A, physical).
    MaxSpeed,
    /// Height (Plane B, registry data).
    Height,
    /// Weight (Plane B, registry data).
    Weight,
}

impl Characteristic {
    /// The name used in `balance/*.json` and in error messages.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ExplosiveForce => "explosive_force",
            Self::MaximalForce => "maximal_force",
            Self::Acceleration => "acceleration",
            Self::MaxSpeed => "max_speed",
            Self::Height => "height",
            Self::Weight => "weight",
        }
    }

    /// Parses a name from `balance/*.json`. `None` if the name is not in
    /// the catalogue (which is a load-time error).
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "explosive_force" => Some(Self::ExplosiveForce),
            "maximal_force" => Some(Self::MaximalForce),
            "acceleration" => Some(Self::Acceleration),
            "max_speed" => Some(Self::MaxSpeed),
            "height" => Some(Self::Height),
            "weight" => Some(Self::Weight),
            _ => None,
        }
    }
}

/// The custodian API, as seen by the calculator.
///
/// The calculator does not know the custodian's internal shape: it asks
/// one characteristic at a time, and it gets a value. The real
/// implementation wraps `pcfm-hidden::GroundTruth`; the test
/// implementation records the calls, to verify the formula asked for
/// exactly its terms.
pub trait Custodian {
    /// Returns the value of `characteristic` for `player`, in `[0, 1)`.
    fn value(&mut self, player: u32, characteristic: Characteristic) -> f64;
}

/// A formula, as read from `balance/*.json`.
///
/// An expression on a fixed grammar: `Term` (a characteristic),
/// `Const` (a number), `Product` (a list, applied left to right),
/// `Ratio` (numerator, denominator), `Clamp` (inner, min, max). The
/// grammar is enumerated; a formula cannot invent an operator.
#[derive(Debug, Clone)]
pub enum Formula {
    /// A term from the catalogue.
    Term(Characteristic),
    /// A constant (a number declared in `balance/*.json`).
    Const(f64),
    /// The product of the operands, applied left to right.
    Product(Vec<Self>),
    /// The ratio numerator / denominator.
    Ratio(Box<Self>, Box<Self>),
    /// The inner value, clamped to [min, max].
    Clamp(Box<Self>, f64, f64),
}

impl Formula {
    /// The terms of the formula, as the calculator must ask for them,
    /// in sorted order (deterministic).
    #[must_use]
    pub fn declared_terms(&self) -> Vec<Characteristic> {
        let mut set = BTreeSet::new();
        self.collect_terms(&mut set);
        set.into_iter().collect()
    }

    fn collect_terms(&self, out: &mut BTreeSet<Characteristic>) {
        match self {
            Self::Term(c) => {
                out.insert(*c);
            }
            Self::Const(_) => {}
            Self::Product(items) => {
                for item in items {
                    item.collect_terms(out);
                }
            }
            Self::Ratio(a, b) => {
                a.collect_terms(out);
                b.collect_terms(out);
            }
            Self::Clamp(inner, _, _) => inner.collect_terms(out),
        }
    }

    /// Validates the formula: it must have at least one term.
    ///
    /// # Errors
    ///
    /// Returns [`CalculatorError::EmptyFormula`] if the formula has no
    /// term (a constant alone is not a formula).
    pub fn validate(&self) -> Result<(), CalculatorError> {
        if self.declared_terms().is_empty() {
            return Err(CalculatorError::EmptyFormula);
        }
        Ok(())
    }
}

/// The calculator: applies a formula to a player, asking the custodian
/// for exactly the formula's terms.
pub struct Calculator;

impl Calculator {
    /// Applies `formula` to `player`, using `custodian` for the values.
    ///
    /// The mechanical test (in `tests/`) uses a recording custodian and
    /// verifies that the calls are exactly the formula's terms, no more,
    /// no less.
    #[must_use]
    pub fn apply(formula: &Formula, custodian: &mut dyn Custodian, player: u32) -> f64 {
        Self::eval(formula, custodian, player)
    }

    fn eval(formula: &Formula, custodian: &mut dyn Custodian, player: u32) -> f64 {
        match formula {
            Formula::Term(c) => custodian.value(player, *c),
            Formula::Const(v) => *v,
            Formula::Product(items) => {
                let mut acc = 1.0;
                for item in items {
                    acc *= Self::eval(item, custodian, player);
                }
                acc
            }
            Formula::Ratio(a, b) => {
                let num = Self::eval(a, custodian, player);
                let den = Self::eval(b, custodian, player);
                if den == 0.0 { num } else { num / den }
            }
            Formula::Clamp(inner, min, max) => {
                Self::eval(inner, custodian, player).clamp(*min, *max)
            }
        }
    }
}

/// Errors of the calculator.
#[derive(Debug, Error)]
pub enum CalculatorError {
    /// A formula has no terms.
    #[error("formula has no terms")]
    EmptyFormula,
    /// A term is not in the catalogue.
    #[error("unknown term: {0}")]
    UnknownTerm(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A custodian that records every call, to verify the calculator asks
    /// for exactly the formula's terms.
    struct RecordingCustodian {
        calls: Vec<(u32, Characteristic)>,
    }

    impl RecordingCustodian {
        fn new() -> Self {
            Self { calls: Vec::new() }
        }
    }

    impl Custodian for RecordingCustodian {
        fn value(&mut self, player: u32, characteristic: Characteristic) -> f64 {
            self.calls.push((player, characteristic));
            0.5
        }
    }

    /// A custodian with a fixed value per characteristic.
    struct FixedCustodian {
        force: f64,
        weight: f64,
    }

    impl Custodian for FixedCustodian {
        fn value(&mut self, _player: u32, c: Characteristic) -> f64 {
            match c {
                Characteristic::ExplosiveForce => self.force,
                Characteristic::Weight => self.weight,
                _ => 0.0,
            }
        }
    }

    /// The movement formula, as decided for F.7:
    /// `a = accel_ref * explosive_force * (weight_ref / weight)`.
    /// Expressed on the fixed grammar, it is a product of three factors,
    /// the third of which is a ratio.
    fn movement_formula() -> Formula {
        Formula::Product(vec![
            Formula::Const(2.0),
            Formula::Term(Characteristic::ExplosiveForce),
            Formula::Ratio(
                Box::new(Formula::Const(78.0)),
                Box::new(Formula::Term(Characteristic::Weight)),
            ),
        ])
    }

    #[test]
    fn calculator_asks_exactly_the_formula_terms() {
        let formula = movement_formula();
        let mut custodian = RecordingCustodian::new();
        let _ = Calculator::apply(&formula, &mut custodian, 7);
        let mut asked: Vec<Characteristic> = custodian.calls.iter().map(|(_, c)| *c).collect();
        asked.sort_by_key(|c| c.name());
        asked.dedup();
        assert_eq!(
            asked,
            vec![Characteristic::ExplosiveForce, Characteristic::Weight],
            "the calculator must ask for exactly the formula's terms, no more, no less"
        );
    }

    #[test]
    fn calculator_does_not_ask_for_a_term_not_declared() {
        let formula = movement_formula();
        let mut custodian = RecordingCustodian::new();
        let _ = Calculator::apply(&formula, &mut custodian, 7);
        assert!(
            !custodian
                .calls
                .iter()
                .any(|(_, c)| *c == Characteristic::MaxSpeed),
            "the formula does not declare max_speed, the calculator must not ask for it"
        );
    }

    #[test]
    fn ratio_compensates_mass() {
        // a = 2.0 * 0.8 * (78 / 78) = 1.6 for a 78 kg player
        // a = 2.0 * 0.8 * (78 / 97.5) = 1.28 for a 97.5 kg player
        // The ratio decides, the two do not add up.
        let formula = movement_formula();
        let mut light = FixedCustodian {
            force: 0.8,
            weight: 78.0,
        };
        let a_light = Calculator::apply(&formula, &mut light, 1);
        assert!((a_light - 1.6).abs() < 1e-9, "78 kg: {a_light}");

        let mut heavy = FixedCustodian {
            force: 0.8,
            weight: 97.5,
        };
        let a_heavy = Calculator::apply(&formula, &mut heavy, 1);
        assert!(
            a_heavy < a_light,
            "heavier must accelerate less: {a_heavy} vs {a_light}"
        );
    }

    #[test]
    fn empty_formula_is_rejected() {
        let formula = Formula::Const(1.0);
        assert!(matches!(
            formula.validate(),
            Err(CalculatorError::EmptyFormula)
        ));
    }

    #[test]
    fn catalogue_names_roundtrip() {
        for c in [
            Characteristic::ExplosiveForce,
            Characteristic::MaximalForce,
            Characteristic::Acceleration,
            Characteristic::MaxSpeed,
            Characteristic::Height,
            Characteristic::Weight,
        ] {
            assert_eq!(Characteristic::from_name(c.name()), Some(c));
        }
    }

    #[test]
    fn unknown_name_is_rejected() {
        assert_eq!(Characteristic::from_name("strength"), None);
        assert_eq!(Characteristic::from_name(""), None);
    }
}
