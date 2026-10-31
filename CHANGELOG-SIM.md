# Changelog — SIM_VERSION

This file records every bump of `SIM_VERSION`, the version identifier of the **simulation behaviour**.

It is separate from [`CHANGELOG.md`](CHANGELOG.md) (which tracks documentation and structure) and from [`CHANGELOG-REGISTRY.md`](CHANGELOG-REGISTRY.md) (which tracks registry deprecations and migrations).

**Rule:** if the outcome of a simulation changes for the same seed and the same inputs, `SIM_VERSION` is bumped, and an entry is added here **before** the change is merged.

---

## Current SIM_VERSION

```
SIM_VERSION = 2
```

This is the **second baseline**. `SIM_VERSION = 1` was the initial baseline; `SIM_VERSION = 2` is the first real bump (ADR-024, 2026-10-31). The reference against which all future bumps are measured is the current value.

**Known gap (OI-14):** no golden hash exists yet — the 10-season golden test is not implemented. The bump procedure below notes this explicitly.

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
2. **Regenerate the golden hash** — the expected state hash for the 10-season golden test (see `00-foundations` §4). **Status 2026-10-31:** the test does not exist yet (OI-14). Until it does, this step is a no-op and the entry records that explicitly.
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
