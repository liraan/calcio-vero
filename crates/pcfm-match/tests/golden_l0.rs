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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 10.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "ee1dd119663c1df7bb6a7669ff43085d81b8bb6f8834a094c01eedcd29e6bc7a6750ebd0f2d1cdd1b7f30f9ab601089558d1715158808fa0483febe465688814fbdfbcaadb9ea6178ef9785d8e766a60f8ae15ea98352fc6978f7aea282e6a9fd56ae9a9d0db023e72b2360fc9f9863bc52a578344575d2af1d6c685600d1399778ee01402b66d9bcf394dedb00cb22bbb3080cf88ddf430ddb86dbc49c023bace523f228f45f34b8fd364d1f48d33b66629fd388cfab79adc41103dc76f6a78af8df370aeb432f36dddded831d67d29dcd6561808d8eaf2c7f17b5f856629ac25bcba33a24ff0866bac96e2ac6d7f897e98ed781e480d903b679a45f2026bffb1faa7a67b93d9de6fdf43cfd57a36ed7b758844b4c153d959544d08409e004fae335287e41c2edb29b5527a858bede8768968bdb1a35d73aa588a3b6c250c52";

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
