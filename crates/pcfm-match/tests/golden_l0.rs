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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 15.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "4d2d4b83367d9de57f42a09f713b38b38389fbed3d1403e58fe63bdaff2da5c5b6d45c3670a079158813a7372476056249355806c52fa87bde2fe76ff9886b9ffc9a49919d69c467107ce4e47ac8d49001b135dfcee550962d210a5ae83ba14dfecddb38be8af2fd489c972f98202188c4c220e238dbb32f0eb43a5f32cfef7d5080186b7e30eea32f6240ffff39e51ce48ec7a7934d7e40502dd2004218a2ad3c66b83af4f757997ff8992e4eb5b308c4d1981e541e648e2723c09371347bb3984c13a39b220b37a5fb1fefe89348df060492efe6835949c5b35cb74ff0c0d6550963d2bde9522b9a28508e51f614e8071d0405373bb7c5f6efe12616f97e5595924f1844853893d8c051e312af2ef94e253d6f80d0a994b62a0bcfe3dac6b28c87b33ee08ca13765a572c7ba4d32fade5b181afae79b9c46dd27c8dc42f9d0";

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
