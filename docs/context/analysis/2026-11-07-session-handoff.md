---
title: "Session handoff — 2026-11-07"
version: "1.0"
status: "analysis"
last_updated: "2026-11-07"
conforms_to: "SESSION-STATE.md, ADR-032, ADR-033, IDEA-DESIGN-003/004, OI-66/68/69/70/71"
---

# Session handoff — 2026-11-07

**Who this is for:** an agent that has never opened the repo. Every claim below is verifiable with `git log`, `git show`, or a grep; nothing is from memory.

## 1. State of the repo (verified)

- HEAD = origin/main = `0a9012f`; tree clean; gates startup/closing green.
- LCV in the bookmark = `fe2f945` (S1: the commit before the bookmark update).
- 205 tests, 10 `pcfm-*` crates (moving to `ds-*`, ADR-032), 30 ADRs (28 accepted, 2 superseded: 002→027, 023→024), `SIM_VERSION` 23.
- OIs: 66, 68 closed; 65, 69, 71 partial; 67, 70 open (plus older OIs).
- Last C2-green: `9b9c6f6` (F.7 step 10). The commit after, `d5ac111` (perceived_ball), carries `Verified: C2-not-run`. **Full C2 is mandatory before the next code commit.**

## 2. What was done (39 commits)

- OI-66 closed: `.githooks/pre-push` requires `git diff --quiet HEAD` before `gates.sh`.
- OI-68 closed: the three historical `## Decisioni operative …` sections moved verbatim from `SESSION-STATE.md` to `docs/context/analysis/2026-11-06-decisions-log.md`. The bookmark keeps a one-line pointer.
- OI-67 finding (D4, rule not written): the 5-voci "Nota di handoff" form contradicted §F (`Done`/`Plan` duplication). Corrected to 2 voci + 15-line cap. The rule in `NEW-SESSION-PROMPT.md` is **not** written (D3: the practice works).
- OI-69 partial: `.github/FUNDING.yml`, Ko-fi badge + `## Support` in README, external texts in `docs/context/REPO-DESCRIPTION.md`. Fiscal consult (Canarie, ISD 651 Gruppo IV) is the maintainer's responsibility.
- ADR-032 accepted: crate prefix `ds-*`. `pcfm-*` survives in history (ADR 001–031, CHANGELOG-SIM, analysis/, old commits). Refactor tracked as Plan step `ADR32` `[open]`, not executed.
- OI-65 partial: `Player` lost `preferred_foot`; `Foot` enum removed; 14 call sites updated; `PlayerLatents` gained `left_foot`/`right_foot` (keyed, `#[allow(dead_code)]` like `explosive_force`). `SCHEMA_VERSION` 1→2. Not done: catalogue terms in `ds-calculator` (blocked by OI-64).
- F.7 `[done]` (all 11 steps): step 9 `heading_from_velocity`; step 10 formula reads fatigue (`Formula::Sum`, `fatigue_coefficient`/`fatigue_floor` in `balance/movement.json`, 5 new tests, `SIM_VERSION` 22→23, CHANGELOG-SIM entry 23); step 11 SIM_VERSION executed incrementally (20→21→22→23).
- OI-70 new: fatigue has no producer. `PlayerBody.fatigue = Player.physical.fatigue = 0.0` in every match. OI-54 said otherwise ("MD-40 producer"): corrected with a dated note. Prerequisite of IDEA-DESIGN-003 and OI-63.
- IDEA-DESIGN-003/004 (sketches): 003 is "the match arc" → D6 split into four phenomena with four producers (OI-70 accumulated fatigue, OI-48 half-time + side change, OI-59 season baseline). 004 is "the deciding goalkeeper" → open question (e) on the foot (ADR-031).
- F.6 `[next]`, step 1: `perceived_ball(spatial, from, ball) -> Option<(i32,i32)>` in `perception.rs`. Signature drops `tick` (the delay needs MD-40/OI-70). One test.
- ADR-033 accepted: **body heading and gaze are two things**. `AgentState` will gain `gaze: u16`; `heading` stays the motion direction (F.7 step 9); the cone reads `gaze`, not `heading`. At L0 the gaze is a declared default (the ball); L2 modulates. Resolves IDEA-PERCEPTION-002 (L0 part) and amends MD-34 (the collapse "orientamento = sguardo" is removed, dated note in the historical log). `gaze` does not exist yet: the implementation is the next session.
- OI-71 partial: F.6 step 2 is not executable without "turn to face play". The real-match filter dismantled (A) "filter only the chaser" (leaves the shortcut: 21 of 22 agents still react to the global ball) and (B) "filter all movement with `heading`" (freezes agents: no see, no move, no turn, no see). The root cause is the collapse; ADR-033 removes it. F.6 step 2 waits for `gaze`.

## 3. What was NOT done (declared)

- `AgentState.gaze`: ADR accepted, field not implemented. Touches `spatial.rs`, `perception.rs`, `state.rs`, tests, golden, `SIM_VERSION` 24.
- F.6 step 2/3: wait for `gaze`.
- ADR32 (`ds-*` refactor, 10 crates, ~177 `use`): Plan step `[open]`, dedicated session, one crate per commit with `cargo test --workspace` green after each.
- OI-65 step 3 (catalogue `left_foot`/`right_foot` in `ds-calculator`): blocked by OI-64 (pass formula does not exist).
- OI-70 (fatigue producer): blocked by a decision on MD-40, not executed.
- Full C2 on `d5ac111`: last green is `9b9c6f6`.
- B2 manual (Plan vs Detail): F.6 `[next]` with step 2 blocked by `gaze`; ADR32 `[open]` as alternative next focus. Divergence tracked, not red.

## 4. Recommended next focus

Implement `AgentState.gaze` (ADR-033). Order:

1. `AgentState` gains `gaze: u16`; `from_squads` initialises it (ball at the centre, everyone looks at it).
2. `visible_to` and `perceived_ball` read the gaze vector, not the heading vector.
3. `heading` stays derived from velocity (F.7 step 9), unchanged.
4. Tests: the two cone tests updated (they set `heading = 0` for "north"; with `gaze` they must set `gaze`); new test "agent running east but looking west does not see the ball to the east".
5. `move_agents` can now use `perceived_ball` (F.6 step 2) without freezing.
6. `SIM_VERSION` 23→24, golden regenerated if it changes (likely **yes**: `gaze` initialised to "ball at the centre" changes defenders' cones at the first tick), CHANGELOG-SIM entry 24.
7. Full C2 **before** the commit, not after.

Alternative focuses without touching the golden: ADR32 (`ds-*` refactor, one crate per commit) or OI-70 (fatigue producer, unblocks IDEA-DESIGN-003).

## 5. Traps paid today (for a new agent)

- `sed` with a pattern that occurs in more than one function is a trap. Fixing `perceived_ball` also touched `visible_to` (same `let...else`). Fallback: `awk` with a whole-line anchor, or `cat >` rewriting the whole file.
- `awk` with a miscounted `skip=N` eats extra lines. The `Product(items) => { … }` pattern was duplicated in `collect_*`; `Self::Ratio(a,b) => {` was deleted. After any `awk` that rewrites a file: `wc` before/after and `git diff` in the same block.
- 4-byte UTF-8 in `sed` (emoji, `→`) fails silently. Use full-line replacement or awk with octal escapes (`\342\206\222` = `→`).
- `commit-msg` hook: the LCV must point at the commit that is HEAD **at the moment of the bookmark commit** (S1). Two consecutive docs commits without a bookmark block the push (E4b requires HEAD to touch `SESSION-STATE.md`).
- Trailers: no blank line between `Direction-Check:` and `Verified:`; otherwise the parser sees only the last and the hook rejects. Verify with `git interpret-trailers --parse < /tmp/msg.txt` **before** the commit.
- `Direction-Check:` required if the diff touches `IDEAS.md` or `WORLD-MODEL.md`. `Rules-Audit:` required if it touches `NEW-SESSION-PROMPT.md` or `DECISION-PATTERNS.md`.
- E4b red while writing the bookmark: normal. The gate reads `HEAD`; the bookmark commit you are about to make resolves it.
- `wc` "revert guard": `after <= before` is wrong for in-place mutations. The correct guard is `after < before`.
- Golden green ≠ behaviour exercised: 0 shots in seeds 42–51. The real coverage is `shot_forced.rs` and the integration tests.

## 6. How to start (for a new agent)

1. Startup ritual (`NEW-SESSION-PROMPT.md` §"Startup ritual"): repo alignment → bookmark → ideas backlog → reports → `gates.sh startup` → `gates.sh resume` → foundations → numbers → report to the maintainer.
2. Read, before proposing anything: `SESSION-STATE.md` §1–§3 **plus** the section `## Note (2026-11-07)` (it lives outside the three sections: it contains the "one session, one focus" method); `docs/adr/033-body-heading-vs-gaze.md` (the decision to execute); `docs/adr/032-crate-prefix-ds.md` (if the focus is `ds-*`); `docs/context/analysis/2026-11-05-session-handoff.md`; `docs/context/analysis/2026-11-06-decisions-log.md`.
3. Do not re-read what the ritual already reads. Declare what you did **not** read (D1).
4. First command after the ritual: `git log --oneline -5` to confirm `0a9012f` is HEAD and no local commit is unpushed. If `## main...origin/main` shows `[ahead N]`, stop and ask.
5. The first operational block should be a **read-only** pass over the file you intend to touch. Mutations start only after the maintainer has chosen the focus.
