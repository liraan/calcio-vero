# Workstream onboarding — PCFM

This document is a living onboarding note for the architectural split that keeps the codebase coherent and explainable.

It is intentionally short and opinionated. It is not a substitute for the normative repo docs (`README.md`, `STRUCTURE.md`, `docs/ARCHITECTURE.md`, `docs/WORLD-MODEL.md`). It exists to make the layer split obvious to the next person who resumes work.

## Core rule

The project follows a strict separation between:

- simulation state
- world-space state
- presentation/projection state

The key invariant is:

- the simulation is the source of truth
- the world model describes reality in a structured way
- the renderer reads and projects, but never invents or mutates the authoritative state

In other words:

- L0 owns the match decisions
- L1 owns the world-space description
- L3 owns the view that shows it

## The three workstreams

### 1) workstream/l0-authoritative-match-state

Purpose:
- keep the match engine authoritative
- own match logic, tick progression, decisions, scoring, possession, phase, event publication
- maintain the canonical match state used by the rest of the system

Responsibilities:
- match tick loop
- rules and deterministic simulation
- event production and state transitions
- authoritative match metadata

Rules:
- this is the source of truth
- no renderer logic here
- no visual projection should be mixed into the simulation

### 2) workstream/l1-spatial-world-state

Purpose:
- define the structured spatial model of the match world
- represent ball, players, pressure, shape, and relevant tactical or spatial information in a consistent way

Responsibilities:
- `Vec2` and world-space vectors
- ball state
- player spatial state
- team shape summaries
- pressure map
- any compact spatial representation consumed by later layers

Rules:
- this layer describes the world, it does not render it
- it is still authoritative for spatial facts, but it remains a model, not a view
- the renderer should never create or overwrite world-space data here

### 3) workstream/l3-field-projection

Purpose:
- project the authoritative state into a readable 2D field view
- render the match in ASCII/TUI/board form
- keep the display completely dependent on the model

Responsibilities:
- `FieldProjection`
- `FieldRenderer`
- grid conversion from world coordinates to rendered coordinates
- ASCII/TUI textual representation

Rules:
- the renderer reads state; it does not invent gameplay state
- it must never decide possession, phase, tactics, or shape on its own
- if a field is not present in the model, it must be absent or derived from the model, not fabricated

## The architectural invariant

The following principles must remain true while the project evolves:

1. No rendering layer is allowed to become a second simulation engine.
2. No view layer is allowed to create gameplay truth.
3. Match logic stays in the authoritative match state.
4. Spatial facts remain in the spatial world model.
5. The field projection is a consumer of state, not a producer of state.

## How to read the repo

When exploring code, use this mental model:

- `pcfm-match` is the authoritative engine layer
- spatial and tactical declarations describe the world and the live state
- `projection` is the presentation layer layered on top

This is the same separation described by the project doctrine: simulation knows the truth, presentation shows a view, and the user is never the source of truth.

## Practical rule for contributors

If a change would make the renderer decide something that should be decided by the match engine, it is the wrong layer.

If a change would make the simulation attempt to directly draw a field in the same code path as game logic, it is the wrong layer.

If a change would make a projection layer invent possession, pressure, shape, or tactical intent without a source in the model, it is a wrong abstraction.

## Current workstream state

The current split is:

- `workstream/l0-authoritative-match-state`
  - authoritative simulation state
- `workstream/l1-spatial-world-state`
  - world-space model and spatial facts
- `workstream/l3-field-projection`
  - projection and renderer layer

These branches are intended to keep each layer isolated and easy to reason about without collapsing all logic into one flat module.

## Summary

The project is not trying to build a single flat match engine that both simulates and renders. It is trying to preserve a clear chain of responsibility:

- simulation decides
- world model describes
- projection visualises

That separation is part of the project’s design and should remain stable as the codebase grows.
