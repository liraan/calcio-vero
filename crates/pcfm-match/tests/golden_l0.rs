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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 12.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "9b10b6a57d61a08831ceddd465e706f65248b3897a962fbfcd9aa6700d8705e74966154e9d8e163da991b2c750463b8cfad345495222081e46c6add4d12e9b2b6a31ee7600126dca799360f1d85178e83685380a1e184eb60c5948669a9e4f39188dd79dce934a237b412ee49bc298b8b392d2720c8152bb49eec3a30004cfd3e501b052d371ecdc25e83fe6cdc99ef1939cb7f6ff58ee7a7976e7dacc07afeb1219ea2a580b45f2c7f5299869c9da9634e23db8fc8d24f2a499b4a26206d5658f4b0b121cd7b38baf3b500f3b5e64f98e2e6e4804c1e3cb35b85c435d4ad6cdb5a8a23bc9a7a4ecfcbbb8dd66f3e930e8fd56f858ceb8c7ab67b7d1987fbce067584e62bfc81af83256668657d0fb96dcb72cfb3ebe28b020ef4b7a8f2b7acda1a856e76f40d54493ea884fdd5d9fdd6bd5294bc7c08f27fc85f6710bf9886c";

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
