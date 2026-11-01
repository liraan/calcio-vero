# Tick-state and temporal semantics

This note documents the core architectural point behind the temporal model:

- a static screenshot is a projection of a state
- a tick is the actual unit of simulation time
- the event chain and the tick-state are the source of truth
- rendering layers (text, HTML, 2D, 3D) are not semantics, they are views

## Why this matters

If a match is interpreted only through static images or screenshots, then the meaning of an action becomes ambiguous:

- the same action can appear different depending on the frame chosen
- temporal variance is confused with semantic variance
- causality between events becomes hard to verify
- the simulation cannot be reconstructed deterministically from static views alone

This is the main weakness of using screenshots as if they were the underlying truth of the simulation.

## Correct separation of responsibilities

The architecture must separate the layers clearly:

### 1. Simulation layer (L0 / tick loop)
This is the actual source of truth.

It owns:
- tick progression
- game state mutation over time
- deterministic event generation
- append-only event chain
- temporal state captured per tick

### 2. Temporal state layer
The actual temporal semantics belong to the tick-state record, not to a rendered frame.

Each tick should be associated with a record containing at least:
- tick id
- score
- possession counters
- event count / chain length
- state hash

This state is what allows the project to reconstruct and verify the entire timeline.

### 3. Rendering layer
Rendering layers are just views over the tick-state or event chain.

Examples:
- text renderer
- HTML renderer
- 2D demo / tactical view
- 3D reconstruction / viewer

These layers do not create semantics. They only project the state into a human- or machine-readable representation.

## The key rule

The game must be modeled as:

- tick = truth
- frame = projection
- event chain = causal and append-only integrity layer
- rendering = display layer

A screenshot is useful to display a moment, but it is not the definition of the moment.

## Why this is important for 2D / 3D

The same principle applies to 2D and 3D viewers:

- 2D and 3D are not replacements for L0
- they are spatial projections of the state
- they depend on the tick-state and event history, not the other way around
- they can be added later without redefining the match contract

The spatial layer must be built on top of the tick-state and event chain, not instead of them.

## What we should record

For a robust temporal model, the simulation should explicitly store:

- tick index
- score at that tick
- possession state at that tick
- event count / chain length
- state hash
- optional derived spatial status (pose, ball state, projected field state)

This record lets us answer questions like:
- what was the state at tick N?
- what changed between tick N and N+1?
- what event or chain of events produced this state?
- can a 2D / 3D reconstruction be generated faithfully from the same source data?

## Final architectural statement

The temporal integrity of the model must never be inferred from static screenshots.

The sound architecture is:

- simulation tick as source of truth
- event chain as append-only causal proof
- tick-state snapshots as explicit temporal record
- rendering layers as views over that truth

This is the basis that makes future 2D / 3D reconstruction reliable without rewriting or weakening the current match engine contract.
