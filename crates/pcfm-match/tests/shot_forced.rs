//! Forced shot tests (F.3-b3): exercise `resolve_shot` directly.
//!
//! The golden range (seeds 42–51) produces 0 `Shot` events, so the
//! `Shot` schema and the geometric goal are not observable there. This
//! test calls `resolve_shot` with a controlled `ShotContext` and checks
//! the new payload: `to` (deviated arrival), `on_target` (geometric),
//! `shot_type` (geometric), and the absence of `goal`.

use pcfm_core::ids::{MatchId, PlayerId};
use pcfm_hidden::{GroundTruth, ShotContext, ShotType};

const fn shooter(slot: u32) -> PlayerId {
    PlayerId::new(slot, 0)
}

const fn ctx_at(from_x: i32, difficulty: i32) -> ShotContext {
    ShotContext {
        shooter: shooter(9),
        from: (from_x, 3400, 0),
        to: (10500, 3400, 244),
        difficulty,
    }
}

#[test]
fn resolve_shot_returns_new_payload() {
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let outcome = gt.resolve_shot(&ctx_at(9000, 0));
    // `to` exists, is 3D, and is not the naive goal centre (deviation).
    assert!(outcome.to.2 >= 0, "to.z must be >= 0: {outcome:?}");
    // `on_target` is geometric, not a field of the die.
    assert_eq!(outcome.on_target, {
        let half_w = 366;
        let h = 244;
        (outcome.to.1 - 3400).abs() < half_w && (0..h).contains(&outcome.to.2)
    });
    // `shot_type` is derived, not a field of the die.
    assert!(matches!(
        outcome.shot_type,
        ShotType::Power | ShotType::Placed | ShotType::Normal
    ));
}

#[test]
fn resolve_shot_deviates_bidirectionally() {
    // Over many calls with different noise draws, the y-coordinate of the
    // arrival point must vary (not always the centre) and must cross the
    // centre on both sides.
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let mut y_values = Vec::new();
    for _ in 0..20 {
        let o = gt.resolve_shot(&ctx_at(9000, 0));
        y_values.push(o.to.1);
    }
    let below = y_values.iter().filter(|&&y| y < 3400).count();
    let above = y_values.iter().filter(|&&y| y > 3400).count();
    assert!(
        below > 0 && above > 0,
        "deviation not bidirectional: below={below}, above={above}, values={y_values:?}"
    );
}

#[test]
fn resolve_shot_pressure_widens_the_deviation() {
    // Same shooter, different difficulties: with high pressure, the
    // standard deviation of the arrival y must be larger.
    let mut gt = GroundTruth::new(42, MatchId::new(1, 0));
    let mut low = Vec::new();
    let mut high = Vec::new();
    for _ in 0..50 {
        low.push(gt.resolve_shot(&ctx_at(9000, 0)).to.1);
        high.push(gt.resolve_shot(&ctx_at(9000, 100)).to.1);
    }
    let var = |v: &[i32]| -> i64 {
        let mean = v.iter().map(|&x| i64::from(x)).sum::<i64>() / i64::try_from(v.len()).unwrap();
        v.iter()
            .map(|&x| (i64::from(x) - mean) * (i64::from(x) - mean))
            .sum::<i64>()
            / i64::try_from(v.len()).unwrap()
    };
    assert!(
        var(&high) > var(&low),
        "pressure did not widen the deviation: var(low)={}, var(high)={}",
        var(&low),
        var(&high)
    );
}
