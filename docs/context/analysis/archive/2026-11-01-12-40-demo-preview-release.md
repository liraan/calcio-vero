# Demo preview release plan — TUI + 2D field renderer

## Status

This is a release plan and validation protocol, not a claim that the preview was built in this session.

This API environment does not expose a shell or a Rust checkout, so I could not actually run `cargo build`, `cargo run`, or `cargo test` here. The material below records the exact preview-release work that is required and the commands needed to validate it locally.

## Goal

Ship a preview release of `pcfm-demo` that is clearly aligned with the project doctrine:

- deterministic simulation remains the source of truth
- TUI/2D rendering remains a projection layer
- the renderer reads game state and does not invent gameplay semantics

The release objective is a single, simple demo that makes the project understandable to a user without requiring a full game UI.

## Scope of the preview release

### Included in the preview

- deterministic match simulation from a seed
- a compact TUI board view
- a functional 2D field renderer
- score output and match summary
- JSON export for machine-readable inspection
- optional HTML snapshot for presentation/debug use

### Explicitly excluded

- rich tactical AI
- irregular or full-featured 3D rendering
- gameplay decisions invented in the renderer
- non-deterministic UI state
- hidden runtime telemetry

## Architecture guardrails

The preview must preserve the same architecture used throughout the repo:

- `L0` match engine remains authoritative
- `L1` world-space and tactical state remains structured and explicit
- `L3` field/TUI projection remains read-only

A renderer is acceptable only if it converts existing state into a display; it is not acceptable if it creates match truth.

## Proposed preview feature set

### 1) TUI preview

Provide a small board-style terminal output using a deterministic layout.

Example scope:

- score line at the top
- status line for phase / possession
- 2D pitch area with players and ball markers
- event summary at the bottom

This should be implemented as a dedicated view layer, not by modifying `simulate_match` logic.

### 2) 2D field renderer

Provide a field projection that reads the authoritative spatial state and renders it into a grid.

Accepted renderer contract:

- reads a world-space model
- maps coordinates into a visual grid
- renders ball and player positions
- optionally overlays possession and phase details
- does not mutate the underlying match state

### 3) Public CLI contract

A preview-friendly CLI shape could be:

```bash
pcfm-demo --seed 42 --tui
pcfm-demo --seed 42 --field
pcfm-demo --seed 42 --json
pcfm-demo --seed 42 --html /tmp/match.html
```

This keeps it easy to show and easy to test.

## Release acceptance criteria

A preview is acceptable only if all of the following hold:

1. `cargo fmt --check` passes
2. `cargo clippy --all-targets --all-features -- -D warnings` passes
3. `cargo test --workspace` passes
4. the same seed produces the same visible output
5. different seeds visibly change the field view without mutating the model
6. the renderer reads from state and does not “decide” sports semantics
7. the CLI is documented in help text and README snippets

## Local validation commands

Run the following commands in a local checkout with Rust installed:

```bash
git checkout main
git pull --ff-only
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run --bin pcfm-demo -- --seed 42 --tui
cargo run --bin pcfm-demo -- --seed 42 --field
cargo run --bin pcfm-demo -- --seed 42 --json
cargo run --bin pcfm-demo -- --seed 42 --html /tmp/match-preview.html
```

## Evidence to record after validation

Use the following template:

```text
Branch: <branch>
Commit: <git rev-parse HEAD>
Result: PASS | FAIL | SKIPPED
cargo fmt --check: <result>
cargo clippy: <result>
cargo test --workspace: <result>
preview TUI: <result>
preview 2D field: <result>
notes: <summary>
```

## Release note content

A release note for the preview should say something like:

- preview build of the match engine demo
- includes deterministic match simulation
- includes TUI preview board and 2D field renderer
- project remains architecture-first and deterministic by design
- rendering remains a projection layer, never a source of truth

## Recommended next implementation order

1. Make the field projection deterministic and pure
2. Add the CLI entry points for `--tui` and `--field`
3. Add one test verifying that the rendering does not mutate the world state
4. Add one golden-output snapshot for a fixed seed
5. Use the demo as the preview release artifact

## Final note

This is the correct preview-release shape for the repository and it respects the project direction already documented in the repo. It does not claim that the preview was already compiled here; it records the exact build and validation steps needed to produce a real preview release on a local machine.
