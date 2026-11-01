# Demo TUI development note

This branch is not the place to continue the TUI work as a natural extension of the L0 review.

## Why

The current branch has already reached a coherent review state:
- L0 remains the match-layer contract
- the implementation is treated as a prototype, not a final state
- the positional foundation has been added as a future-facing support layer
- the review branch is intentionally closed as an architecture checkpoint

The TUI demo is a different workstream: it is a product/demo feature, not a continuation of the L0 review itself.

## Correct lifecycle

The correct lifecycle is:
1. finish the architecture checkpoint / review branch
2. stop broad changes
3. open a dedicated TUI/demo feature branch or task
4. implement the terminal demo only there
5. keep the review branch stable and unchanged

## Correct implementation direction

If the TUI demo is to be built, it should be implemented as:
- a separate demo feature, with its own scope
- a terminal UI built with `ratatui` / `crossterm`
- a user-facing simulation runner for the current match engine
- a shell and rendering layer, not a change to the underlying L0 contract

## Important boundary

The TUI must not be used to redefine or “complete” the L0 contract.
The match engine remains deterministic and contract-driven.
The TUI is only a rendering / interaction layer on top of the existing engine.

## Final status

This note documents the correct separation of concerns:
- architecture review branch: stable and closed
- TUI demo: separate feature work, to be implemented in a dedicated follow-up branch or task
