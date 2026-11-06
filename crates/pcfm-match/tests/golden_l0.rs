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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 22.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "9c632da435908e37e2aef0530ad48960b9be7cb9316e7f90101f0f5c08d6716aec902defd79ef58c79ddf94fac8854b3c387180ce099082450dc541203d55c4119b844dcb5a204d1ca76332e000871298df5d3be9590a425169db112dde6c6fc81e3e542c1ffb472a8fe4f1d4200b8edda0a1cf148d2a033ebbd7b8cfb0e4c81778ac6cdabf61cdad685f146648062c3dc8a4c78952c97de53d463a103381483985bc64c451652f9d398934e84df3846307eff2f6017daba49c2f0b8a76f20cc83ffb73aba1f8f3b808e508b0a73826a5cf7d93fb96639efa7c674eb888ee3d1305b80ecd0745022be7a73139deb397d18382b7f17a8cc6c25ebf24b8e4db3d06ac4e2e21fd6b06a202b176c92d2e3749e96d74f708bb7955fc199330bef5a2532ddada99a1d102d9126ce03d5008fd9e38a880bca0df6dbb180532ecaaec102";

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
