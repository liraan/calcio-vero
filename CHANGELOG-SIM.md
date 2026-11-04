# Changelog — SIM_VERSION

This file records every bump of `SIM_VERSION`, the version identifier of the **simulation behaviour**.

It is separate from [`CHANGELOG.md`](CHANGELOG.md) (which tracks documentation and structure) and from [`CHANGELOG-REGISTRY.md`](CHANGELOG-REGISTRY.md) (which tracks registry deprecations and migrations).

**Rule:** if the outcome of a simulation changes for the same seed and the same inputs, `SIM_VERSION` is bumped, and an entry is added here **before** the change is merged.

---

## Current SIM_VERSION

```
SIM_VERSION = 13
```

`SIM_VERSION = 1` was the initial baseline (no code). 2 removed `CauseTrace` and `xg` (ADR-024). 3 removed `template_key`/`params` from `NarrativeEntry`. 4 removed four gusci fields from `MatchEvent`/`MatchSave`. 5 replaced `player_id` with `actor: Option<ActorId>` and added `KickOff`/`SecondHalf` (ADR-026). 6: two halves with separate clocks, `Half` field, four-phase loop, fixed-point stoppage time, boundary events emitted (ADR-026 phase 2). 7: stoppage contributions calibrated (goal 60s, foul 20s, duel 10s). 8: slice B2 — the acting agent comes from space (`pick_from_space`), `Tackle` emitted when a duel is won by the defender, `SpatialState.carrier` tracked, stream names with `match_id` (OI-22). OI-22 closed: RNG stream names now carry `match_id` (`match:{id}:actions`, `match:{id}:action_kind`), so two matches with the same world seed no longer share the engine streams. The current value is the reference against which all future bumps are measured.

9: C.1 — `Pressure` emitted beside an action (Pass, Shot, Duel), calculated with `resolve_influence` (03-entities-minimal §2.9) on the ball's point; the most influential opponent within `space_threshold_cm` presses. A step toward the outcome (ControlError, later). Golden hash regenerated.

10: A1 — one RNG stream per noise consumer in `pcfm-hidden` (`hidden:{match_id}:{latents,duel,shot,pass,save,injury,set_piece,pressure}`), replacing the single shared stream. Adding a new consumer no longer shifts the outcomes of the others (`00-foundations` §4, `01-rng.md` §2.3). Closes OI-39. Golden hash regenerated.

11: F.3-b2 — the ball crosses the goal line as a geometric fact: `Shot` carries `goal` and `on_target` (was: `goal` only, on the shot). `Goal` is emitted only from a `Fact::LineCrossing { kind: GoalLine }`, not from the shot. `Pass` is emitted for the first time (was: `resolve_pass` called, no event). `Fact::Trajectory` carries `kind` and `success`; `Fact::LineCrossing` carries `actor` and `point`. `classify` extended; six unit tests. Golden hash regenerated. Behavior change: a `Pass` event appears in the stream, and `Shot.goal` now comes from the line crossing. F.3-b1 (BallState 3D) did not bump: no behaviour change (golden identical).

12: F.4-a — agents move every tick (MD-09, MD-10, MD-12). `move_agents` reads the ball's offset from the pitch centre, translates every formation slot by a declared fraction (30%), deviates the nearest agent to the ball, steps each agent by at most `MAX_STEP_CM` (120), and enforces a minimal separation (`BODY_RADIUS_CM` 45). No RNG in the movement (S1): the variation comes from the inputs. Golden hash regenerated; the match now moves, so the hash changes. The separation pass is a `Vec` snapshot (same operations, same order, no per-pair map lookup): same behaviour, 28 s -> 5.8 s on the golden.
13: F.4-c — the ball flies and the possession resolves at landing, not at launch. `Pending` (MD-37) with a mandatory expiry (`FLIGHT_TICKS` = 15) and the expected possession; `other()` computes the opposite without mutating; the check at the top of `tick` applies the possession when the pending is due. In the pass and shot branches the carrier is released at launch, `set_ball_in_flight` is no longer a no-op, `advance_ball` runs. Declared simplification: no interception yet (OI-37), the ball lands at the destination or out; the goal stays the die's outcome (MD-20). Golden hash regenerated (behaviour changed: the ball moves between ticks).
**Known gap (OI-14):** the 10-season golden test (an L1 artefact) is not implemented. The L0 golden test (N matches, seeds 42..51) exists and is regenerated on every bump.


`SIM_VERSION` is defined in `pcfm-match` (`crates/pcfm-match/src/save.rs`) and embedded in every save (see `L0.07` §2.1).

---

## When to bump

`SIM_VERSION` is bumped when **any** of the following is true:

1. **RNG behaviour changes** — algorithm, seed derivation, or stream naming (see `L0.01` §2.6).
2. **Event emission changes** — an event type is added, removed, or its emission rules change in a way that affects the event stream (see `L0.05`).
3. **Resolution logic changes** — a `resolve_*` function in `pcfm-hidden` returns a different outcome for the same inputs (see `L0.03` §3.2).
4. **Tick loop changes** — the order of operations within a tick changes, or the tick granularity changes (see `L0.04` §2.2).
5. **Random event catalogue changes** — a `RandomEventDef` is added, removed, or its probability function changes in a way that affects the distribution (see `L0.08` §2.5).

If none of the above is true, `SIM_VERSION` is **not** bumped. A refactor that preserves behaviour does not bump `SIM_VERSION`. A new feature that does not affect the match does not bump `SIM_VERSION`.

---

## Bump procedure

A `SIM_VERSION` bump requires **all five steps**, in order. In phase 1 (ruleset disabled, no CI, single maintainer) the local C2 verification replaces the CI merge gate; see `AGENT-POLICY.md` §1 and §9.

1. **Bump the value** in `pcfm-match` (`crates/pcfm-match/src/save.rs`). The value is a monotonic integer. It never goes down.
2. **Regenerate the golden hash** — run `cargo test -p pcfm-match --test golden_l0`, copy the `left:` value from the failure, replace `EXPECTED` in `golden_l0.rs`. At L0 the test is N matches (not 10 seasons: L1 does not exist). The L1 form is a future artefact.
3. **Write the entry** in the "History" section below, using the template.
4. **Verify save compatibility** — every existing save fixture must still load. If a save cannot be reconstructed under the new `SIM_VERSION`, the entry must say so explicitly and a migration must be provided (see `L0.07` §2.3).
5. **Review by a second pair of eyes.** In phase 1 there is one maintainer: the post-push review (`AGENT-POLICY.md` §9) replaces the second reviewer. In phase 2 (ruleset active, branch + PR), the reviewer confirms that the golden hash change is expected and that the entry is honest.

The full migration procedure for **saves** lives in [`docs/MIGRATION-01-04.md`](docs/MIGRATION-01-04.md). That document points back to this changelog for the bump side and forward to the save side.

---

## Entry template

Every entry has the same shape. Copy this template and fill it in.

```markdown
## [SIM_VERSION N] — YYYY-MM-DD

### Why the bump

One paragraph. What changed and why it could not be done without changing
simulation behaviour.

### What changes for the same seed

Concrete. "Match X (seed 12345) now ends 2-1 instead of 1-1 because ...".
If the change is statistical (affects distributions but not any single
match), say so and give the distribution delta.

### Saves affected

- Can existing saves still be loaded? (yes / no / with migration)
- Can existing saves be replayed? (yes / no / with warning)
- If a migration is required, where does it live? (link)

### Golden hash

- Previous: `0x...`
- New: `0x...`
- Verified by: (reviewer name or handle)

### Related changes

- `CHANGELOG.md` entry: (link or "none")
- `CHANGELOG-REGISTRY.md` entry: (link or "none")
- ADRs affected: (list or "none")
```

---

## History

## [SIM_VERSION 8] — 2026-11-01

### Why the bump

Slice B2: the acting agent comes from space, not from a random roll.
`pick_players` (random attacker + random defender, same roll) is replaced
by `pick_from_space`: the attacker is the agent of the possessing team
nearest the ball, the defender the nearest opponent to the attacker.
`SpatialState.carrier` tracks who has the ball. A duel won by the
defender emits `Tackle` with `from`/`to`.

### What changes for the same seed

The attacker and defender are no longer random: they are the agents
nearest the ball. The event stream changes (who shoots, who passes, who
duels), so the score and the chain hash change. Seed 42 produces a
different match.

### Saves affected

No schema change. The `SIM_VERSION` bump is for the simulation behaviour,
not the save format. A v7 save loads under v8, but re-simulation differs.

### Golden hash

- Before: `d8470ee8...`
- After: the new value in `crates/pcfm-match/tests/golden_l0.rs:21`
  (regenerated from the failure, as the procedure says).

### Related changes

- `crates/pcfm-match/src/state.rs`: `SpatialState` field, `pick_from_space`,
  `Tackle` on a won duel, carrier set/cleared.
- `crates/pcfm-match/src/orchestrator.rs`: `MatchState::new(ctx.match_)`.
- `crates/pcfm-match/src/save.rs`: `SIM_VERSION` 7 -> 8.

## [SIM_VERSION 7] — 2026-11-01

### Why the bump

ADR-026 phase 2 calibration. The initial stoppage-time contributions
(goal 30s, foul 30s) were a first proposal, not a calibrated value. The
match recovery was ~2'50" for a half with 4 goals, low against the real
game. New values: goal 60s, foul 20s, duel 10s, base 0.

### What changes for the same seed

The recovery (and therefore the timestamp of the boundary events) changes.
Seed 42: first half 45'+2' -> 45'+4', second half 45'+1' -> 45'+2'.

### Saves affected

No schema change. The `SIM_VERSION` bump is for the simulation behaviour,
not the save format. A v6 save loads under v7, but the timestamps of a
re-simulation differ.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- `balance/stoppage_time.json`: goal 60, foul 20, duel 10, base 0.
- `pcfm-match::balance` tests updated.

## [SIM_VERSION 6] — 2026-11-01

### Why the bump

ADR-026 phase 2: two halves with separate clocks, `Half` field on every
event, four-phase tick loop, fixed-point stoppage time, boundary events
(`KickOff`, `HalfTime`, `SecondHalf`, `FullTime`) emitted.

### What changes for the same seed

Events carry `half: Half`; `timestamp_minute`/`timestamp_second` are
relative to the half. The match runs two halves of 45' plus a fixed-point
stoppage time each. `FullTime` is at `(Second, 45+rec2)`.

### Saves affected

Breaking: `half` added to every event. A v5 save does not load under v6.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- ADR-026 (Match boundaries and stoppage time), amended 2026-11-01.
- `balance/stoppage_time.json` (new, embedded with `include_str!`).
- `pcfm-match::orchestrator`: four-phase loop.
- `pcfm-demo`: board shows `1T`/`2T` and `45'+X'`.

## [SIM_VERSION 5] — 2026-11-01

### Why the bump

ADR-026 (accepted 2026-11-01): match boundaries and stoppage time.
`MatchEvent.actor: Option<ActorId>` replaces `player_id: PlayerId`; new
`EventType::KickOff` and `EventType::SecondHalf`; `HalfTime` and `FullTime`
emitted; two halves with separate clocks and separate stoppage time.

### What changes for the same seed

Events carry `actor` instead of `player_id`. Boundaries (`KickOff`,
`HalfTime`, `SecondHalf`, `FullTime`) have `actor: None`. Match end time is
`45+rec1` for the first half and `45+rec2` for the second, with `rec`
computed from the events of each half (fixed point, no cap; guard at 45').

### Saves affected

Breaking: `player_id` removed, `actor` added. A v4 save does not load under
v5. Migration is not provided: no v4 save exists outside test fixtures, and
the fixture is regenerated. The `docs/MIGRATION-01-04.md` procedure is the
template; a `MIGRATION-04-05.md` is not needed for phase 1.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- ADR-026 (Match boundaries and stoppage time) — accepted 2026-11-01.
- `pcfm-core`: `ActorId` (`Player | Referee`).
- `pcfm-stats::MatchEvent`: `player_id` -> `actor`.
- `pcfm-stats::EventType`: `KickOff`, `SecondHalf` added.
- `pcfm-match::narrative`: handles `actor` and the new event types.
- `pcfm-demo::viewmodel`: `EventRow.team` gains `Match` for boundary events.

## [SIM_VERSION 4] — 2026-10-31

### Why the bump

`MatchEvent` lost three fields: `career_minutes_cumulative`, `season_label`, `observed_attrs_snapshot_hash`. All three were always `None` or a duplicate of another field: `career_minutes_cumulative` equalled `minutes_played_cumulative`, `season_label` was `None` in production, `observed_attrs_snapshot_hash` was `None`. The event payload enters `chain_hash`, so the bump is mandatory. `MatchSave` also lost `config_versions`, always empty: that alone would not require a bump, but it is included in the same change.

### What changes for the same seed

The `chain_hash` differs from SIM_VERSION 3 for the same seed: the serialised `MatchEvent` has three fewer fields. Score, possession, shots and the events themselves (type, minute, player) are identical.

### Saves affected

- Load: yes, unknown fields are ignored.
- Replay: no — `can_replay()` returns false for SIM_VERSION < 4.
- Migration: none. Old saves are read-only.

### Golden hash

- Not recorded. The 10-season golden test does not exist yet (OI-14).
- Verified by: C2, locally, 2026-10-31.

### Related changes

- `crates/pcfm-stats/src/event.rs`: three fields removed from `MatchEvent`; `Hash` import removed.
- `crates/pcfm-match/src/save.rs`: `config_versions` removed; `SIM_VERSION` 3 -> 4.
- Specs: `05-match-events.md` §2.4/§2.5 (removed fields), `07-save-minimal.md` (`config_versions`).
- IDEAS: IDEA-INFRA-012 (career/season data at L1), IDEA-INFRA-013 (observed-attrs snapshot at Plane C).

---

## [SIM_VERSION 3] — 2026-10-31

### Why the bump

`NarrativeEntry` lost `template_key` and `params`. Both were produced by `narrate()` and serialised in the save, but had no consumer: `render.rs` reads only `minute` and `text`, and `locales/` does not exist. The payload is smaller; the save shape changed, so the bump is mandatory.

### What changes for the same seed

The `chain_hash` is unchanged (narrative is not part of the chain). The save JSON differs: `result.narrative[*]` has four fields instead of six. Score, stats, events and `state_hash` are identical. Old saves written at SIM_VERSION 2 load (serde ignores unknown fields) but cannot be replayed under SIM_VERSION 3.

### Saves affected

- Load: yes, unknown fields are ignored.
- Replay: no — the sim version differs, `can_replay()` returns false.
- Migration: none.

### Golden hash

- Not recorded. The 10-season golden test does not exist yet (OI-14).
- Verified by: C2, locally, 2026-10-31.

### Related changes

- `crates/pcfm-match/src/narrative.rs`: `NarrativeEntry` reduced to `{minute, event_id, text, importance}`. `template_for` → `importance_for`; `render_text` takes `EventType`.
- `crates/pcfm-demo/src/render.rs`: test fixture updated.
- IDEAS: IDEA-INFRA-011 (narrative localisation at L1/L3).

---

## [SIM_VERSION 2] — 2026-10-31

### Why the bump

ADR-024 removed `CauseTrace` and `Factor` from L0 (supersedes ADR-023). The event payload changed: `MatchEvent.cause_trace` is gone, `ShotOutcome.xg_value` is gone, the latent-derived modifiers (`injury_risk_modifier`, `temperament_modifier`, `risk_modifier`) are gone. Because the event payload enters `chain_hash`, the same seed now produces a different chain hash. A bump was mandatory.

### What changes for the same seed

Every match: the `chain_hash` differs from SIM_VERSION 1 for the same seed, because the serialised `MatchEvent` has fewer fields. The score, possession and shots are **not** expected to change (the resolution logic is identical; only the serialised payload is smaller). Verified by the test suite and the test-runner (seed 42): no match result changed, only the hash.

### Saves affected

- Can existing saves still be loaded? **Yes** — `serde_json` ignores unknown fields, and the removed fields simply do not deserialise.
- Can existing saves be replayed? **No.** Replay re-simulates under the new rules; the chain hash will not match the saved one. Old saves are therefore **read-only**. The contamination is declared, not hidden.
- Migration: none. The fix does not retroactively clean JSON written before the bump.

### Golden hash

- Previous: not recorded (no golden test existed at SIM_VERSION 1 either).
- New: not recorded. OI-14 tracks the missing golden test.
- Verified by: C2 (build, clippy, 102/102 tests, fmt, test-runner 6/6), locally, 2026-10-31.

### Related changes

- ADR-024 (Remove `CauseTrace` at L0) — accepted, supersedes ADR-023.
- `specs/L0-match/05-match-events.md` §2.3 amended.
- `specs/L0-match/10-ground-truth.md` §2.2 amended.
- `specs/L0-match/06-match-output.md` §2.3 amended (xG removed).
- Anti-leak test: `crates/pcfm-match/tests/anti_leak.rs` (fails pre-fix, passes post-fix).
- IDEAS: IDEA-INFRA-009 (CauseTrace at L1).

---

<!--
  NOTES FOR FUTURE EDITORS:

  - Do not edit a past entry. If a past entry was wrong, add a correction
    entry and reference the one it corrects. The history is append-only.
  - Do not bump SIM_VERSION "to be safe". A bump is a promise to the user
    that their saves and replays are affected. Bump only when necessary.
  - If you are unsure whether a change requires a bump, ask in an issue
    before merging. The cost of a missed bump is a silent determinism
    regression, which is the worst kind of bug this project can have.
  - This file is linked from:
      - specs/00-foundations.md §4 (determinism)
      - specs/L0-match/01-rng.md §2.6
      - specs/L0-match/07-save-minimal.md §2.3
    If you move or rename this file, update those links.
-->
