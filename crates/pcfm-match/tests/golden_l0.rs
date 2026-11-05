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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 19.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "cde6dadbd692f314b87ca5d8b2caae52091e5b826ec27bc6ead1959dbcec0f21ec902defd79ef58c79ddf94fac8854b3c387180ce099082450dc541203d55c4119b844dcb5a204d1ca76332e000871298df5d3be9590a425169db112dde6c6fc81e3e542c1ffb472a8fe4f1d4200b8edda0a1cf148d2a033ebbd7b8cfb0e4c81778ac6cdabf61cdad685f146648062c3dc8a4c78952c97de53d463a1033814837f3db5dec49261f41d421d6d582f93565099dee60c84a3b96ad68f6067eb49bd642c05ea98ba248ece02927afa5ed00a727386e28d359d10957f2bbd191a9fb815c6c35c425a8ce11d3d12b6cb7345ffd33d8e19ff809939ef592195163c653b6ac4e2e21fd6b06a202b176c92d2e3749e96d74f708bb7955fc199330bef5a25e7e59ac3ada6b287d374540f5337ca250e9b1732fe2565b97daf99b62eab7318";

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
