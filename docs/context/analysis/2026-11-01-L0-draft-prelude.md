---
title: "L0 rewrite prelude — repo state, verification, and drafting scope"
version: "1.0"
status: "draft"
last_updated: "2026-11-01T10:35Z"
auditor: "Copilot audit"
conforms_to: "00-foundations (v2.0); docs/context/NEW-SESSION-PROMPT.md"
---

# L0 rewrite prelude

**Scope:** prepare a clean L0 drafting pass on the `review-l0-branch`, without changing the main branch state. This document explains the goal, the assumptions, the verified code state, and the exact scope of the L0 rewrite.

**Status:** working draft for review by a second AI or maintainer. This is a handoff note, not the final specification.

## 1. Objective

The goal is to rework the L0 specification in a way that is:

- consistent with the current codebase
- consistent with the repo-wide doctrine
- explicit about what is implemented vs what is still a design target
- ready to be reviewed by a second AI before final ratification

The underlying intent is to avoid rewriting L0 from stale assumptions. The new draft should be grounded in the live repository state rather than in older archival material or earlier notes that were written before the current repo structure was stabilized.

## 2. Working assumptions

These assumptions are the basis for the drafting pass:

1. L0 is still the match layer, not the whole game.
2. The repo is intentionally structured around a set of distinct crates and a deterministic design philosophy.
3. The actual code is the primary source of truth for implementation status, while spec files and ADRs define the intended contract.
4. The L0 draft should distinguish clearly between:
   - covered and implemented behavior
   - documented but not yet fully implemented behavior
   - future L1/L2 extension points
5. The draft should not claim “full realism” or “full completion” where the code is intentionally a prototype.

## 3. Files reviewed

The following files were used to ground the work:

- `README.md`
- `STRUCTURE.md`
- `Cargo.toml`
- `crates/pcfm-core/Cargo.toml`
- `crates/pcfm-core/src/lib.rs`
- `crates/pcfm-core/src/rng.rs`
- `crates/pcfm-core/src/event_bus.rs`
- `docs/context/SESSION-STATE.md`
- `docs/context/analysis/2026-10-31-22-28-review.md`
- `docs/context/analysis/2026-11-01-10-25-review.md`
- `specs/_index.md`
- relevant files under `specs/L0-match/` if present in the live repo

## 4. What was verified in code

This is the verification pass that informs the rewrite.

### 4.1 Workspace structure and crate boundaries

The codebase is a Rust workspace with clearly separated crates. The workspace root and crate layout confirm that the project is intentionally modular and designed around responsibilities instead of a single monolithic game engine.

Verified in:

- `Cargo.toml`
- `crates/pcfm-core/Cargo.toml`
- `crates/pcfm-core/src/lib.rs`

### 4.2 Deterministic RNG is a core design feature

The core crate contains a deterministic RNG design based on named streams and a derived seed. This is a meaningful signal that the project does not treat randomness as an incidental implementation detail; it is part of the behavioural contract.

Verified in:

- `crates/pcfm-core/src/rng.rs`

Key conclusions:

- stream names are explicit and validated
- seeded derivation is deterministic
- same seed + same stream name = same sequence
- the project intentionally prefers deterministic order and predictable outputs over ad hoc randomness

### 4.3 Event bus semantics are explicit and bounded

The event bus is designed around insertion order, handler registration order, and in-process dispatch. It is intentionally simple and scoped, which is coherent with an early L0 prototype stage.

Verified in:

- `crates/pcfm-core/src/event_bus.rs`

Key conclusions:

- event dispatch is deterministic and ordered
- handlers are registered per event kind
- the bus documents known limitations instead of hiding them
- this is a clean place to describe the “current prototype contract” versus the “future hardened contract” in L0 rewrite work

### 4.4 Repo hygiene and review status are also relevant to L0

The repo has active review documents that explicitly record gaps between the specification and the current code. Those reviews are important because they show that L0 is not a fully exhausted area; it is a ratified contract with an implemented prototype that still has known deltas.

Verified in:

- `docs/context/SESSION-STATE.md`
- `docs/context/analysis/2026-10-31-22-28-review.md`
- `docs/context/analysis/2026-11-01-10-25-review.md`

Key conclusions:

- some L0 gaps are documented and tracked
- the repo explicitly distinguishes between settled doctrine and prototype implementation
- the L0 draft should preserve that distinction instead of flattening everything into a falsely complete spec

## 5. What this means for the L0 rewrite

The draft should not read like a naive restatement of the original specification. It should instead read like:

- the project’s current contract at L0
- the subset that is implemented today
- the known gaps between the formal L0 contract and the prototype code
- the future boundary where L1/L2 begins

This is the correct framing for the current repo state.

## 6. Drafting scope for the second AI

The second AI should focus on the actual L0 document, not on repo hygiene or workflow tasks.

The review should cover exactly these areas:

1. What L0 is, in plain architectural terms.
2. Which behaviors are part of the ratified L0 contract.
3. Which behaviors are implemented in the current prototype.
4. Which gaps are still open and should be framed as “not yet implemented” rather than “undefined”.
5. How to preserve the doctrine around determinism, plane separation, and prototype maturity.

## 7. Out of scope for this pass

The following should be left out of the L0 rewrite unless the repo state clearly requires them:

- workflow setup and branch protection details
- local hook installation instructions
- analysis retention policy details
- branch-level repo hygiene tasks
- wider L1/L2 roadmap planning

Those items are tracked elsewhere and should not be mixed with the spec rewrite itself.

## 8. Current recommendation

The correct working mode for the next step is:

- keep this branch as the draft review branch
- keep the L0 rewrite in a focused document file
- allow the second AI to revise the specification text itself
- keep this report as the grounding note for what was validated and why

## 9. Output expected from the next pass

The next pass should produce a clean L0 draft that:

- is concise but complete
- matches the actual repo structure
- avoids claiming full implementation where the repo is still a prototype
- explicitly reflects the difference between doctrine, implemented behavior, and deferred work

This file is the ground-truth handoff for that work.
