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

/// The expected hash of the concatenated state hashes, at `SIM_VERSION` 18.
/// Regenerate when `SIM_VERSION` changes.
const EXPECTED: &str = "1335b19f58ccd9cacd99ce511a60359cd611cbd2e365e43f4e8f1ccc6304e036ec902defd79ef58c79ddf94fac8854b3c387180ce099082450dc541203d55c41e1cc14adc35e3d51d091c25f67f3cd5072e9c3fb564e6cfbaef4f44985d4ac49974532df27356f478986382ebe73830aa85054c87efe81eb729ba2b019b43eb8795908d21be7ed28262f62bd2f3166d66f27f9599bf6adaa2feab5c785d759b24c6a00fa84453ddc1612da10c6b7e743d9cd3d18b5f179c67502939c89c5759a0d8512fb2dc84a63feb69617c67620882c7671c929b3cfdab4fdf6ea5dff6bd66f3ba519671b1c2901645841692b427690c26026813710cdeff5efa46d04e0736ac4e2e21fd6b06a202b176c92d2e3749e96d74f708bb7955fc199330bef5a253ccad5785bfd4fe15cf813c5b51912d8bcbc730ec154fa38b8980e156af43b17";

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
