# Layered architecture and live tactical state

This note records the fixed architectural position for the project:

- L0 is the match engine and causal state
- L1 is world-space / spatial state
- L2 is tactical policy and live team behaviour
- L3 is presentation and rendering

The design goal is not “one giant L0 that does everything”. The goal is a nested system where each layer owns one kind of truth and communicates only downward/upward through explicit contracts.

## 1. The rule

We must never let a renderer invent the semantic state of the match.

That means:

- screenshots, 2D views, and 3D views are projections
- the simulation is authoritative
- phase, possession, pressure, tactical intent, and team shape are facts of the model
- the renderer can display them, but not decide them

## 2. Layer responsibilities

### L0 — Match / causal state

This layer owns:

- score
- possession state
- tick progression
- event chaining
- match phase
- tactical commands
- deterministic rules
- snapshots that record the state over time

This is the source of truth for match logic. It is the only layer that can emit authoritative state changes.

### L1 — World-space / spatial state

This layer owns:

- ball position and velocity
- player positions and velocities
- role shape and spacing
- pressure map
- team shape summaries
- geometry used by 2D/3D renderers

This layer materialises the match from the L0 state; it does not create new semantics.

### L2 — Tactical policy / live behaviour

This layer owns:

- pressing intensity
- width and depth adjustments
- aggressive counterpressing
- press-high / low-block behaviour
- inversion of the fifths
- tactical command application
- role overrides and local spatial intent

This is where live changes in the match are interpreted and applied.

### L3 — Presentation / rendering

This layer owns:

- 2D view
- 3D reconstruction
- text / HTML / replay overlays
- camera choices
- visual polish

It reads L0/L1/L2. It does not author state.

## 3. Why this matters for realism

If we let the renderer “fill the gaps”, we lose the simulation.

Examples:

- a 3D renderer invents “the team is pressing high” even though the match state never says that
- a tactical UI edits the shape visually instead of by state mutation
- a snapshot is derived from a frame instead of from the authoritative tick state

All of those create a false narrative. They are not a football match; they are an artistic approximation without a contract.

## 4. Required model pieces

The current architecture must include:

- PhaseState
- TeamTacticState
- TacticalCommand
- TickSnapshot
- BallState
- PlayerSpatialState
- SpatialState
- TeamShapeState
- PressureMap

These are the core ingredients required to move from a semantic match model to a realistic football model.

## 5. TODO (current implementation status)

### Done

- clarify layered architecture in docs
- define tactical and phase-state model in `crates/pcfm-match/src/tactics.rs`
- define tick snapshots in `crates/pcfm-match/src/result.rs`
- add spatial world-state model in `crates/pcfm-match/src/spatial.rs`
- keep rendering separate from core state

### In progress

- integrate tactical state into `MatchState`
- record tactical state in snapshot output
- define per-tick spatial update from L0 + L2 to L1
- define how tactical commands are validated and applied in the loop

### Planned next

- add explicit `BallState` and `PlayerSpatialState` update logic
- add `TeamShapeState` derivation from roles and spacing
- add pressure map logic for home/away teams
- add live command application for: high press, counterpress, invert fifths, push wings, drop line
- add rendering adapter for 2D/3D that reads the spatial state and not the other way around

## 6. Architectural summary

The correct relationship is:

- L0 decides what happened
- L2 decides how the team behaves
- L1 converts that behaviour into spatial movement
- L3 displays the result

This keeps the simulation deterministic, auditable, and extensible without collapsing the architecture into a single giant engine.
