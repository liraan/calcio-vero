# Branch test matrix — PCFM

## Status

This report documents the test execution plan for the current workstream split.

Important note: this API session does not provide a shell or a local Rust checkout, so I could not actually run `cargo test` on the repo branches from here. I am therefore recording the exact commands that must be run locally, together with the expected branch-by-branch validation flow.

This is not a fake pass report. It is a test protocol and evidence template for the repository owner or maintainer to run in a real local environment.

## Correct folder

The repository’s own guidance says that `docs/context/analysis/` is the location for review outputs and working evidence. This file belongs there because it is session-scoped operational evidence, not normative repo documentation.

## Branches under review

- `main`
- `workstream/l0-authoritative-match-state`
- `workstream/l1-spatial-world-state`
- `workstream/l3-field-projection`

## Required local commands

Run this from a local git checkout of the repository in a shell with Rust installed.

### 1) Baseline on `main`

```bash
git checkout main
git pull --ff-only
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

Expected result:
- the project is green on the canonical branch
- any drift from the documented baseline is visible immediately

### 2) Validate `workstream/l0-authoritative-match-state`

```bash
git checkout workstream/l0-authoritative-match-state
git pull --ff-only
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

What to verify:
- the authoritative match state remains deterministic
- no rendering logic leaks into the simulation layer
- the L0 behavior is stable against `main`

### 3) Validate `workstream/l1-spatial-world-state`

```bash
git checkout workstream/l1-spatial-world-state
git pull --ff-only
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

What to verify:
- world-space definitions compile cleanly
- spatial model remains separable from rendering
- no new rules are introduced in a projection layer

### 4) Validate `workstream/l3-field-projection`

```bash
git checkout workstream/l3-field-projection
git pull --ff-only
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

What to verify:
- projection code remains read-only
- field renderers do not mutate world state
- no gameplay logic is disguised as rendering

## Evidence template to paste after each run

Use the following block after each branch run:

```text
Branch: <branch>
Commit: <git rev-parse HEAD>
Result: PASS | FAIL | SKIPPED
cargo test --workspace: <summary>
cargo clippy --all-targets --all-features -- -D warnings: <summary>
cargo fmt --check: <summary>
Notes: <what changed or regressed>
```

## Acceptance rule

A branch is considered structurally valid only when:

- `cargo fmt --check` passes
- `cargo clippy --all-targets --all-features -- -D warnings` passes
- `cargo test --workspace` passes
- the branch remains aligned with the layer split:
  - L0 = authoritative state
  - L1 = spatial world description
  - L3 = projection/rendering only

## Decision

At the moment, this is a recorded validation protocol, not a real execution result. The reason is procedural: this environment does not grant shell access or a live checkout, and I am not willing to invent test output.

If a local machine is available, the above commands should be run exactly as written and the results recorded against each branch.
