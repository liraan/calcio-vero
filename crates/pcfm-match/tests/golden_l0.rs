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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 7.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "d8470ee868d0d69338b290ad9080031e269fbb3bd4840244d723b8f0fc9247ff000f7503128852aec713c237330724113991ae6353e11c4471d2b1e8dfd6c79956fb23b251608b39e3970bed2ca60bc49ec8e166b31ebe8772a7b7eb7657b3e7529bbe7ba0098edce01a76cf5a22f61ab7f9fcd9e555ea6ccb1877b0d4278f9780bd9cffc79948cde1c80c19b4894987470f2daaa8d5eb9ba1a3c233f2b00d949b6831bb448a37493cb6d7ece7a4f460a455a4a12ff0e55158826cf48c479b9aee2eea9baabb8a31b312cb762816a57df9c30254043ff9d5b2daebfbe96ad8f0bb2346707d03fcd1f29ef1931a3f07b4c44903f19feb26a1692dbda0a7d56d7529948cbe275d6fbc714017c02bfa0344cacb846dbd83e2261e2146709a248926ecae4a42bf6c03f71bccdbf5b7318fabcd9cdff358d500a48376fab5adfab83a";

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
