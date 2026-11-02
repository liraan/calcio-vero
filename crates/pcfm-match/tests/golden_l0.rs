//! Golden determinism test, L0 (specs/00-foundations.md §4, OI-14).
//!
//! Simulates N matches from a known seed range, concatenates the state
//! hashes, and compares with the expected value. The expected value is
//! regenerated on every `SIM_VERSION` bump (CHANGELOG-SIM.md, step 2).
//!
//! L0 only: a "season" does not exist at L0, so this is N matches, not
//! 10 seasons. The 10-season golden test is an L1 artefact.

use pcfm_fixtures::match_fixture;
use pcfm_match::{MatchConfig, simulate_from_seed};

/// Number of matches in the golden run.
const N: u64 = 10;

/// First seed of the range.
const SEED_START: u64 = 42;

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 9.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "afca5f20344dedb96f422c67f52c2861f8cb4b9f143a3ff4f9a4691368d6255619d205d6d41d804c873b0847ae08bbf49e4e401a0d579c6baf205efcc61d86b552467b9671398352977d69740e6442141ff2c5185329e7184956689fee7d370a97d92d2d13ccc1505b596d5fb540522ead6392bdea1904394abfb8a968d7f64a2250f834893dbd48949d62eecbfe535bde2f97d517c0c964fbab2f3ababdfe793e2355bc6a38410c2938269d5eb8596e28fd979f39fcc0ca54bb1afcee5da82cdf0d4c31310a064b448502a674df2b5576732cdf83d6eaafaa5b70d384cb546ac408acef280f315552569dce1f368a1a1186692d1571845cf81d14292c6ff651a18c270905de2c73998f8345ca7970ac1b8ee3993bde94d665b93138b133a913b4ebc4adff1b3fb97d88372aff04d7a0e15dfb2010e452c15437c47026d225a9";

#[test]
fn golden_l0_is_deterministic() {
    let mut combined = String::new();
    for i in 0..N {
        let seed = SEED_START + i;
        let m = match_fixture(seed).expect("fixture");
        let result = simulate_from_seed(&m, seed, MatchConfig::default());
        for b in result.state_hash.as_bytes() {
            use std::fmt::Write as _;
            let _ = write!(combined, "{b:02x}");
        }
    }
    assert_eq!(
        combined.len(),
        usize::try_from(N).expect("N fits in usize") * 64,
        "each hash is 32 bytes"
    );
    assert_eq!(
        combined, EXPECTED,
        "golden L0 hash changed: SIM_VERSION bump or a simulation regression"
    );
}
