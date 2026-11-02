# Decision map — PCFM

**Purpose:** the map of *decisions*, not of steps. The Plan in
`SESSION-STATE.md` executes; this file shows the forks the Plan cannot
represent as a sequence. Read it before choosing a step that touches the
engine model.

**Date:** 2026-11-02. **Status:** sketch, not ratified. This is a map, not
an ADR.

## The motif (level 0, not negotiable)

Every event carries the fields to understand why. Source: `L0-POSITIONAL-SLICE.md` §0,
`00-foundations.md` pillar 5, §6-ter criterion 2.

Open: is the motif staged (L0 = local causality, L2 = global)? See OI-33.

## The structural gap (level 1, verified)

The engine does not represent the conditions under which a player acts.
It knows: position (static after kickoff), carrier. It does not know:
pressure, coverage, run. Evidence: `pcfm-hidden/src/context.rs` (no
Context carries conditions); `resolve_pressure` discards the latent
(`let _ = ...`) and uses `noise > 0.5`; `AgentState.velocity` is `(0,0)`
hardcoded.

ADR-019 D2 (accepted 2026-10-27) ratifies an individual functional state
enum. It is a photograph of that date. **Contracts are photographs**: D2
says *what* a functional state is, not *when* to add it, and not that the
individual form is the right one.

## The design question (level 2)

How should conditions be represented so that they are *causes*, not
*labels*?

Reframe (2026-11-02, pause): in a real match, pressure is not an
individual state. It is a **relation** between space, time, and agents.
It is recognised by the behaviour it produces (hurried pass, shielding,
loss), not by an internal label. The missing dimension is therefore an
observable *relational configuration*, not an individual state.

That reframe favours B/C over A, and makes `pick_from_space` (which
already computes the nearest defender distance) the first brick.

### The forks

    A. Persistent individual state (ADR-019 D2)
       FunctionalState per agent, updated each tick.
       A.1 FunctionalState in AgentState
       A.2 tick updates it
       A.3 resolve_* reads it
       A.4 Pressure emitted with a real cause
       A.5 Slip/ControlError with a real input
       Risks: SIM_VERSION bump, orchestrator.rs touched (note §4 says
       "no change"), D3 test: structure without content today?

    B. Local relational condition (no persistent state)
       A ConditionContext built at resolve time: distance from carrier,
       possession, surface. B.1 new type; B.2 resolve_* extended.
       Risks: does not answer "what does the player do in between";
       does not pay the ADR-019 D2 debt, it bypasses it.

    C. Hybrid (B now, A after)
       Pressure with a relational ConditionContext (small, verifiable),
       functional state deferred.
       Risks: two condition semantics coexist.

    D. Nothing now
       Step 19 + minimal C1 (Pressure as it is today).
       Risks: a fake Pressure is "a die with a name": violates §6-ter
       criterion 2. Must be declared as prototype, not as model.

### Transversal node T — step 19

Valid in every fork: without a consumer, every new field added in a
slice violates §6-ter criterion 4. Cost: low (no SIM_VERSION, no
Note-Read, no engine files). T.1 narrate_event reads fields; T.2 test
on Tackle with from/to.

### D3-test (a check, not a fork)

Does this structure have a consumer *today*? FunctionalState today has
none (`resolve_pressure` uses nothing). ConditionContext today has one
(Pressure without input is a die, and §6-ter criterion 2 forbids it).
Applied honestly, the D3-test favours B/C, not A.

## Direction (2026-11-02)

Relational, not individual. B is the direction; C is the first
verifiable step; A (D2 of ADR-019) remains a tracked debt, not the
answer. The pressure model may stay an arbitrary prototype, but with an
input that exists and that, if changed, must change the outcome.

**Mechanical test (D5):** two different distances, two different outcome
distributions. If the outcome does not change, it is still a die.

## Mapping to open items

| OI | Node |
|---|---|
| OI-25 movement model | A.5, B.4 |
| OI-26 tick granularity | A.2 |
| OI-27 L0↔L2 hook (#5) | independent, decides if L2 will be a bolt-on |
| OI-28 §3.4 vs §6-ter contradiction | before any slice C |
| OI-29 velocity hardcoded | same node as OI-25, seen from another side |
| OI-30 balance/ empty | A.5, B.4 |
| OI-31 spec↔code drift on EventType | before adding Slip/ControlError |
| OI-32 narrative.rs.tmp orphan | hygiene, not blocking |
| OI-33 staged motif | level 0.1 |
| OI-34 D2 tension in existing code | tracked, not blocking |
| IDEA-DESIGN-001 | the reframe itself (relational conditions) |

## What this map is not

- Not a plan: the plan is in `SESSION-STATE.md`.
- Not an ADR: nothing here is ratified.
- Not a descent: a session executes **one step**, not a branch. The map
  is consulted, not followed.
