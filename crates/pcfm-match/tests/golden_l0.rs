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
const EXPECTED: &str = "775c8edde5363a2389cf5b1cd9ee6695d4d55b6ca0fa5dcdf2a4ab357126168b646e8913bf6430182298a83634b7784b6bf5389978fc2621b09329ea2ca1909fa9d5a8d3ca764e480d0f332250bd126b0acc98e910c0f57591b729633e761d3024e934261291ee11af63d6a2623d95949b9d842e933be43761309ab2403415930e32575ba05ab2400fc315631826745b0137220170f2844a7f0716aac4f0b6cec40ede605d82e2a310a1ba30073a9c8f4b169577a39148579303f3afe04c03d7d4999111be51d67db572f6fe97298177d1c487cbeaec4b2610095bde7e330cb6dda33a803f29d380d87261c68793b004becfde2b753a2ffaff430716c87f97d842dbecc638d9a502753f064f977cb0d9375a665dbcdcb1a992a2835850db89ee3edd08418da2997379e8bdabc45d53b8d8114f7e565d558bc493c4b62c10fd21";

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
