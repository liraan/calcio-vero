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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 25.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1d70c3eb3d8728342ede6b3e6e834892557cf87d285e0afae2aad0877294ac289cd1a7a0d9f2d2250f9d622d1b8a3f36b350c3a973ede64e1da4f79e787ebe4413ea4501d0b8917cc078629d628148a0f5b30e52d5c4347218abcd93484e5014a30c9cee948b14a256f93fbead3ece53c7e757abbb4bab429771ff794043648ed8e97dd4847c54a7826fe15be23925f806d9319758f4dc7f32e2030e6599abd3985bc64c451652f9d398934e84df3846307eff2f6017daba49c2f0b8a76f20cc4bbda056c7d4ae3808d3bf70f713528c406045b0b4bf18815d063018cc775e0e10dd14cec52c121afb8771e0b18fccd971e3fb0769cee73886faf862c7c9ed3b724b5406849584a6a64612534e8e7ffe806e31e3f0755ebfdf4bd66da1f224862cde9815a9d8d09907e2a8fbf63de45f390b060115427afb9dda019156bafa69";

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
