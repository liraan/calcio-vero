---
title: "L0 draft verification checklist"
version: "1.0"
status: "draft"
last_updated: "2026-11-01T10:46Z"
auditor: "Copilot audit"
conforms_to: "00-foundations (v2.0); docs/context/NEW-SESSION-PROMPT.md"
---

# L0 draft verification checklist

**Purpose:** keep the L0 rewrite grounded in operational decisions instead of assumptions. Every item below should be verified against the repo before the draft is considered stable.

**Scope:** this checklist is for the `review-l0-branch` L0 rewrite pass only. Its purpose is to ensure the rewriting stays coherent with the repository’s doctrine, current code, and review notes.

## 1. Checklist of operational decisions

| ID | Decision area | Must verify against | Status | Notes |
|---|---|---|---|---|
| D1 | L0 scope is match-only and does not absorb L1/L2 concerns | `00-foundations.md`, `README.md`, `specs/_index.md` | ⬜ | L0 must remain narrow and match-focused |
| D2 | L0 is ratified as a contract, while implementation remains a prototype | `SESSION-STATE.md`, `docs/context/analysis/*`, `specs/L0-match/00-overview.md` | ⬜ | This distinction must remain explicit |
| D3 | Plane A stays isolated behind `GroundTruth` | `00-foundations.md`, `ADR-003`, `L0.10`, `04-match-engine.md` | ⬜ | No direct hidden-state access |
| D4 | Determinism is central and must remain explicit | `01-rng.md`, `Cargo.toml`, `crates/pcfm-core/src/rng.rs` | ⬜ | No casual use of time/random sources |
| D5 | Event bus ordering remains insertion-ordered and single-threaded per match | `02-event-bus.md`, `crates/pcfm-core/src/event_bus.rs` | ⬜ | Do not allow async ordering drift |
| D6 | Observations are emitted as `MatchEvent`, not latent state | `05-match-events.md`, `06-match-output.md` | ⬜ | Output must be event-derived |
| D7 | Ratings and stats are derived from events, not direct hidden access | `06-match-output.md`, `00-foundations.md` | ⬜ | This is a core protection rule |
| D8 | Save/load is replay-friendly and should not serialize raw Plane A | `07-save-minimal.md` | ⬜ | Do not de-scope this boundary |
| D9 | Random events and modifiers stay separate from the engine core | `08-random-events.md`, `09-context-modifiers.md`, `04-match-engine.md` | ⬜ | Separate concerns remain important |
| D10 | Do not claim implementation completeness where code remains partial | `SESSION-STATE.md`, review notes, current code | ⬜ | State prototype vs ratified contract honestly |
| D11 | Do not add future L1/L2 concepts into the L0 draft | `README.md`, `specs/_index.md`, `00-foundations.md` | ⬜ | Keep L0 focused |
| D12 | Terminology stays aligned with current project language | `README.md`, `docs/GLOSSARY.md`, `PLUGIN_DEVELOPER_GUIDE.md` | ⬜ | Use project-specific terms consistently |
| D13 | Review gaps remain visible, not hidden | `docs/context/analysis/*.md`, `SESSION-STATE.md` | ⬜ | A draft should not erase known gaps |

## 2. Pass criteria before closing the draft

The draft is considered stable only if all of the following are true:

- it preserves the Three Planes doctrine
- it keeps L0 within its match-only scope
- it clearly distinguishes ratified contract from prototype implementation
- it keeps the `GroundTruth` boundary explicit
- it does not promise more than the code and the review notes support
- it does not add L1 or L2 concepts without clear upstream justification
- it reads consistently with the rest of the repo and not as an isolated artifact

## 3. Review rule

If a decision is uncertain, do not finalize it silently.

Instead:

- leave it as an explicit open item
- record the reason it remains unresolved
- cite the relevant source file or document that drove the uncertainty

This keeps the rewrite disciplined and protects it from silent contradiction.

## 4. Minimal verification sequence

When reviewing a new section in the L0 draft, use this exact pattern:

1. Check whether the section modifies the contract or only the prose.
2. Check the relevant source file(s) in `specs/L0-match/`.
3. Check the supporting doctrine in `00-foundations.md`.
4. Check the current code files in `crates/pcfm-core` where the concept is implemented.
5. Check review notes for any known gap or exception.
6. Only then finalize the wording.

## 5. Operational reminder

This branch is meant to preserve clarity and reduce accidental drift. The L0 draft is not a place for unverified design choices.

Every time a decision is made, ask:

- what file in the repo says this is the rule?
- is this a design requirement or a code fact?
- does the repo already state this somewhere else?
- if not, do we need to leave it open instead of asserting it?

That is the working method for this branch.
