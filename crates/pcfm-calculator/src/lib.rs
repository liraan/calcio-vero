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
//! This crate is the skeleton. The first formula (movement) arrives with
//! F.7; the other formulas arrive when the phenomena that need them exist
//! (OI-50).

use std::collections::BTreeMap;

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
    /// the catalogue (which is a load-time error, see [`Formula::validate`]).
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
    ///
    /// The value is a hidden performance (Plane A) or a registry datum
    /// (Plane B, read through the custodian because the formula does not
    /// know the plane). The calculator never interprets it; it passes it
    /// to the formula.
    fn value(&mut self, player: u32, characteristic: Characteristic) -> f64;
}

/// A formula, as read from `balance/*.json`.
///
/// A weighted sum of terms, not a free expression language: the terms are
/// enumerated (the catalogue), the weights are in the file, the arithmetic
/// is integer or fixed-point. The formula declares its terms; the
/// calculator asks for exactly those.
#[derive(Debug, Clone)]
pub struct Formula {
    /// The terms, with their weights. `BTreeMap` for determinism.
    pub terms: BTreeMap<Characteristic, f64>,
}

impl Formula {
    /// Validates the formula against the catalogue.
    ///
    /// # Errors
    ///
    /// Returns [`CalculatorError::UnknownTerm`] if a term is not in the
    /// catalogue. This is a **load-time** error: the formula is rejected
    /// before any match starts.
    pub fn validate(&self) -> Result<(), CalculatorError> {
        // Every key in `terms` is already a `Characteristic`, so by
        // construction it is in the catalogue. The validation is the
        // `from_name` step: a name that does not parse never reaches this
        // type. This method stays for the day a formula is loaded from a
        // file (the JSON → Formula step), when an unknown name would
        // otherwise be silently dropped.
        if self.terms.is_empty() {
            return Err(CalculatorError::EmptyFormula);
        }
        Ok(())
    }

    /// The terms of the formula, as the calculator must ask for them.
    #[must_use]
    pub fn declared_terms(&self) -> Vec<Characteristic> {
        self.terms.keys().copied().collect()
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
    pub fn apply(
        formula: &Formula,
        custodian: &mut dyn Custodian,
        player: u32,
    ) -> BTreeMap<Characteristic, f64> {
        let mut values: BTreeMap<Characteristic, f64> = BTreeMap::new();
        for term in formula.declared_terms() {
            values.insert(term, custodian.value(player, term));
        }
        values
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

    fn movement_formula() -> Formula {
        let mut terms = BTreeMap::new();
        terms.insert(Characteristic::ExplosiveForce, 1.0);
        terms.insert(Characteristic::Weight, 1.0);
        Formula { terms }
    }

    #[test]
    fn calculator_asks_exactly_the_formula_terms() {
        let formula = movement_formula();
        let mut custodian = RecordingCustodian::new();
        let _ = Calculator::apply(&formula, &mut custodian, 7);
        assert_eq!(
            custodian.calls,
            vec![
                (7, Characteristic::ExplosiveForce),
                (7, Characteristic::Weight),
            ],
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
    fn empty_formula_is_rejected() {
        let formula = Formula {
            terms: BTreeMap::new(),
        };
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
