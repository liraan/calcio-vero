# Agent policy — PCFM

**Purpose:** define what an agent (Warp, DeepSeek-TUI, or a future custom bot) may and may not do on this repository, and how its actions are approved. This is a **policy**, not a specification: it defines the guardrails, not the implementation.

**Last updated:** 2026-10-29 (commit `08eb158`)

## 1. Principles

- The agent **proposes**, the human **approves**. No action on the remote repo happens without explicit confirmation (exception: the audit-report push in §8).
- **Phase 2 (ruleset active):** every change goes through a **branch and a pull request**. No direct push to `main` by anyone. **Phase 1 (ruleset disabled, single maintainer):** direct push to `main` is allowed only to the maintainer (audit-report exception: see §8).
- The agent **does not merge** pull requests. Only the human merges.
- The agent **does not force-push** `main`, **does not delete** `main`, **does not create tags** without explicit confirmation.
- The agent works on `agent/<task>-<date>` branches. These branches are intentionally **not protected**: they are the agent's workspace. `main` is protected by a GitHub ruleset **when active** (server-side).

## 2. What the agent may do autonomously

- Read any file in the repository.
- Run `cargo test`, `cargo clippy`, `cargo fmt --check`.
- Run `git status`, `git diff`, `git log`, `git branch`.
- Propose a diff or a patch in the chat.
- Create a local branch (`agent/<task>-<date>`).
- Commit on that branch (the local pre-commit hook enforces fmt, clippy, test on Rust changes).
- Push that branch to `origin` (the branch is not protected; `main` is).

## 3. What requires explicit confirmation

- Opening a pull request.
- Modifying any file under `docs/adr/`, `specs/`, `docs/context/`.
- Modifying `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`.
- Modifying `LICENSE*`, `.github/`, `.githooks/`.
- Any operation not listed in §2.

## 4. What the agent must never do

- `git push --force` on `main` (blocked by the ruleset, not just by policy).
- `git push` directly to `main` (blocked by the ruleset).
- Merge a pull request.
- `git tag`, `git reset --hard`, `git clean -fd` on the remote.
- Modify files outside the repository.
- Disable or bypass the ruleset on `main`.

## 5. How approval works

- The agent shows the exact command it wants to run, and the diff it will produce.
- The human reviews and confirms (or rejects).
- Only then the agent executes.
- If the agent is uncertain, it asks. It never guesses.

## 6. Defences in place

| Defence | Where | What it blocks |
|---|---|---|
| **GitHub ruleset on `main`** | Server-side (GitHub) | Direct push, force-push, deletion, merge without PR + 1 approval |
| **Pre-commit hook** (`.githooks/pre-commit`) | Local | Commit with Rust code that fails fmt, clippy, or test |
| **Branch naming** (`agent/<task>-<date>`) | Convention | Not a security control: traceability |
| **PR + merge by human** | Process | Unreviewed changes reaching `main` |

The ruleset is the **primary** defence. The hook is a local safety net. The branch naming is traceability, not security.

**Current status (2026-10-27):** the ruleset is **disabled**. Only the
maintainer pushes. It must be reactivated **before** granting a write token
to any agent. The pre-commit hook remains active.

## 7. Revocation

If the agent misbehaves, or if its access must be withdrawn:

- **Revoke the token**: the agent's GitHub token is revoked from GitHub settings (Settings → Developer settings → Personal access tokens). The agent can no longer push anywhere.
- **Disable the ruleset bypass**: if a bypass actor was configured, remove it.
- **Delete `agent/*` branches** that contain unreviewed work.
- **Document the incident** in `docs/context/SESSION-STATE.md` (what happened, what was revoked).

No agent action is irreversible: `main` is protected, and `agent/*` branches can be deleted.

## 8. Analysis agents (audit agents)

The audit agent is a **separate role** from the chat agent (Warp, DeepSeek-TUI, or a future custom bot). Its default output is a single Markdown analysis file, not code changes.

- **Default:** produces `docs/context/analysis/YYYY-MM-DD-HH-MM-review.md` from a prompt (see `docs/context/GITHUB-SESSION-PROMPT.md`). It does not modify code, configuration, or any other file.
- **Analysis-only agents with a write token (e.g. Hermes).** An audit agent that produces a full repository audit (build + test + clippy + security scan + coherence) may push the report directly to `main`, provided: (a) the file matches `docs/context/analysis/YYYY-MM-DD-HH-MM-review.md`; (b) the file is the **only** change in the commit; (c) the commit message follows the repo convention (`docs(analysis): ...`); (d) the maintainer verifies every finding with `grep`/`sed` before any action is taken. If any condition is violated, the exception does not apply: branch + PR in Phase 2, or the maintainer commits manually in Phase 1. See `docs/context/GITHUB-SESSION-PROMPT.md` for the interaction contract.
- **Exception:** the maintainer may explicitly authorize a scoped modification (which files, what kind of change) in the same session. Authorization is written, not inferred. The agent never decides on a scoped modification by itself.
- **Asynchronous by design.** The audit agent is not a synchronous channel: for time-critical fixes, use the chat agent or the maintainer directly.
- **Even when authorized**, the change follows the standard flow: verify (`NEW-SESSION-PROMPT.md` rule 8), atomic commit, and — in Phase 2 — branch + PR (rule 1).

## 9. Rollback, post-push review, residual risks

The closing ritual may now proceed without a pre-push pause (E2, green-path derogation, 2026-10-31). This section defines the safeguards that replace it, and the risks it accepts.

**Post-push review.** The maintainer reviews the closing report after the push, not before. The report opens with `SESSION CLOSED — no further work executed (E3)` and lists, per commit: subject, trailers (`DPs-Applied`, `Rules-Audit`, `Verified`), and any `Deferred:` entry. A `Deferred:` that the maintainer judges improper is a policy violation, not a stylistic choice.

**Rollback.** A wrong auto-close is undone with `git revert <commit>` followed by a push. History is preserved; no rewriting. The revert commit message names the reverted SHA and the reason. Reopening the session after the revert follows the normal ritual.

**Residual risks (judgment-bound, accepted).** D5 requires that a judgment-bound condition produce a visible artifact. These three are the known residual risks, declared not hidden:

- **Blast radius** (D4 question 4). Predicting downstream effects requires judgment. If the pre-mortem misses it, the artifact is the report: the maintainer sees the rule and can contest it. Not mechanizable.
- **D5 depends on the maintainer reading the report.** A visible artifact that nobody inspects is theatre. This is a pact, not an enforcement: the value of the protocol is proportional to the review that follows it.
- **Auto-push skips the human check before the push.** Mitigated by rollback (above) and by post-push review (above). Not eliminated.

**Not a guarantee.** Auto-closing skips the human check on the *content* of the work (C2 and the gates check mechanics, not judgment). A green closing is not a correctness proof: it is a statement that the protocol was followed. When the work is uncertain, the maintainer can still pause before the closing (E2, first paragraph).

## 10. What this policy is not

- It is not a substitute for `CONTRIBUTING.md` (process for humans).
- It is not a specification of the agent's implementation.
- It is not a guarantee: a misconfigured agent can still do damage. The policy is a contract, not a sandbox.

