# TODO — layered match architecture

Status: active
Last updated: 2026-11-01

## Objective

Build a nested match system in which:

- L0 remains the definitive simulation layer
- L1 gives real spatial state to the match
- L2 applies tactical policy and live behavior
- L3 renders without inventing semantics

## Current state

- [x] architecture principle clarified
- [x] tactical phase model defined
- [x] tick snapshots defined
- [x] spatial layer defined
- [x] documentation updated to keep the layered design explicit

## Next steps

- [ ] integrate tactical state into `MatchState`
- [ ] integrate `PhaseState` updates into the tick loop
- [ ] add `SpatialState` creation/update logic from L0 + L2
- [ ] add live tactical command validation and expiry logic
- [ ] add `BallState` and player spatial updates per tick
- [ ] add `TeamShapeState` and pressure-map generation
- [ ] add 2D/3D rendering adapter that reads authoritative state only
- [ ] add tests for layered consistency and deterministic snapshots

## Design constraints

- no renderer may invent missing semantics
- no L0 state may be replaced by a visual approximation
- no 2D/3D layer may mutate the game state in place
- all live tactical changes must be logged as events and recorded in snapshots

## Notes

This is the architecture that allows us to approach realistic football without collapsing the project into a giant non-layered simulation.
