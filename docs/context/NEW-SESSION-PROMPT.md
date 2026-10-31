# New Session Prompt — PCFM

**Purpose:** prompt to paste as the first message of a new chat, to resume work on the project without losing context. Update this file when the project state or the operating rules change.

**Last updated:** 2026-10-31 (B1-bis: a git commit block may run twice; check `git log --oneline -2` after a commit, reset --hard to the first. Earlier: A2 exact exit words; OI-16 granularity; Decisions trailer; closing 2a unexpected observations; B1-bis transient M; IDEA-006 splash; IDEA-016 alternatives; gates startup|closing; closing Gate A text; chiudi; heredoc lesson; D4/D5; pure-vs-gap; Filter agent; E2 green-path; §G Git; Step 2 capture+2b+2c; DECISION-PATTERNS; AGENT-POLICY §9)

---

## How to use

1. Open a new chat.
2. Copy the content of the block below (from `# Resuming the PCFM project` to the last line, without the 4 backticks).
3. Paste it as the first message.
4. If the chat is resumed after a long pause, read `docs/context/SESSION-STATE.md` first: it says where we are, what is open, and what the next agreed step is.

---

## Prompt

````
# Resuming the PCFM project — new session

I am resuming work on **PC Football Manager (PCFM)**. The previous chat ended at a precise point. Here is the context so we can pick up without losing anything.

## The project

- **Repository:** https://github.com/liraan/pcfootballmanager (public)
- **What it is:** a community alternative to commercial football management games. Transparent, offline-first, deterministic. Doctrine of the Three Planes (Plane A = Ground Truth, Plane B = Observation, Plane C = Estimation).
- **Language:** technical documentation in English; conversation with me in Italian.
- **Status:** L0 (match engine) is complete and implemented. L1–L3 are planned.

## Where we are (structure, not numbers)

**For current numbers (crates, tests, ADR count), read `SESSION-STATE.md` and `STRUCTURE.md` §5. This section only gives the stable structure: it does not duplicate counts that change every session.**

- **Crates:** 9 `pcfm-*` crates: `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures`.
- **`docs/context/`** holds briefing material (not normative): `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `AGENT-POLICY.md`, `GITHUB-SESSION-PROMPT.md`.

## Why these files matter (the commands are in the Startup ritual)

The **Startup ritual** below has the exact commands. This section explains **why** each file is read, so you know what you are looking for.

- **`docs/context/SESSION-STATE.md`** — the bookmark. What the next step is, what was done, what is open. Read it first.
- **`docs/context/IDEAS.md`** — open and rejected ideas. What is **not** decided yet; do not implement from it.

Then, when the task requires it:

- **Architecture:** `docs/ARCHITECTURE.md` (how the crates interact) and `docs/CODE_TOUR.md` (reading path).
- **Vision:** `docs/WORLD-MODEL.md` (levels, taxonomy, simulation vs presentation).
- **Doctrine (non-negotiable):** `specs/00-foundations.md` (v2.0, pillar 1 = local determinism) and `specs/_index.md`.
- **Contract:** `docs/adr/019-match-engine-contract.md` (agent-first, positional, N agents: players + referee).

## What remains to be done

**The live plan is in `docs/context/SESSION-STATE.md` §"Next steps".** Do not
maintain a second list here: it diverges. **Read the bookmark before proposing
anything.** Do not assume a step is still open because it was open in a
previous session: verify with `git log` and the bookmark. Read it for the
ordered next steps, the current priority, and the files touched.

## Operating rules (important — we established them together)

**Some rules are hard stops: violating them means stop and report. They are marked 🛑: D1, D2, C2.**

**A. Mode**

- **A1 — Execution.** One command per message, one file at a time, with countdown. At the start of each multi-step plan, declare the total number of tasks; each response states `Task i/N`. When announcing a mini-session, state the expected number of commands (`3/4`) if it is predictable; if it is not, say so (`commands expected: not predictable`) and proceed in blocks. No prose around the commands. The pasted output of each command is the implicit "ok" to proceed **if the verification is clean**; if it shows a problem, stop and fix before moving on.
- **A2 — Pausa, riprendi, chiudi.** The word **"pausa"** may be written by the maintainer **or** by the assistant. It suspends execution (no commands, no file edits) and opens a discussion in prose. **Exactly two words exit the pause, and they must be written exactly:**
    - **"riprendi"** — exits the pause and resumes execution **without** closing. E5 (ideas from pause) fires first, then the agreed work resumes.
    - **"chiudi"** — exits the pause **and** starts the closing ritual (E2).
    **Everything else does not exit the pause.** "Ok", "va bene", silence, an "ok" to an unrelated question, the natural end of a task: none of these resume or close. There is no "equivalent": the two words are exact.
    The assistant does not resume on its own. Pausa does not trigger the closing ritual.
- **B1 — Tools and pitfalls.** Use `sed`/`awk` for targeted edits. Delimiters: use `@` or `~`; if the text contains apostrophes, use a heredoc (`<< 'EOF'`). BRE pitfalls: `\(` `\)` are groups, use `[(]` `[)]`; a pattern with `**` fails. For multi-line replacements with backticks/em-dash/arrows, use a `c\`/`i\`/`a\` command via quoted heredoc, not a one-liner. **Heredoc limit (tested 2026-10-29): the problem is the code fence (` ``` `), not the length.** A 44-line heredoc without fences works; a 12-line heredoc with a fence breaks. Use 4-space indentation for code blocks inside heredocs, never ` ``` `. Long heredocs are fine as long as they contain no fences. To verify an anchor with invisible characters, use `sed -n 'Np' file | cat -A`. Mojibake in output is a *display* artefact: confirm with `grep -c`. Before `c`/`a`/`i`, verify the anchor with `sed -n 'Np'`; never trust line numbers from a previous state. After `git checkout --`, re-run ALL seds, not only the ones that seem missing.
- **B1-bis — Tool fallbacks (observed).** Each rule below comes from a real session where the primary approach failed. None is speculative.
    - **`sed` one-liner `s@pattern@replacement@` > ~500 chars** fails on Git Bash with `unterminated s command` (observed 2026-10-31, OI-2 rewrite attempt). Fallback: `awk` + `/tmp` + `mv`, always with verification in the same block.
    - **`ed` is not installed on Git Bash** (observed 2026-10-31). Do not propose it. `awk` + `/tmp` + `mv` is the default for long replacements.
    - **`grep '^\s*pub fn'` is too narrow**: `pub const fn` is public API (observed 2026-10-31, R-10 in `pcfm-hidden`). Use `grep -nE 'pub (const )?fn'` or a semantic pattern, not a keyword-specific one.
    - **Line numbers from a previous state go stale**: locate with `grep -n 'semantic-pattern'` first, then `sed -n 'Np'` only to verify the *content* of that freshly-located line. This is the operational reading of B1's "never trust line numbers from a previous state".
    - **Mojibake in output is a display artefact**: confirm with `grep -c`, never visually.
    - **A failed command may have partially executed** (observed 2026-10-31: `awk` with a syntax error in a variable still ran and corrupted `SESSION-STATE.md`; `wc` unchanged, `git diff` revealed it). After any command that produced an error, verify with `git diff <file>`, not only `wc`/`grep`.
    - **A transient `M` may appear after `awk > tmp && mv`** (observed 2026-10-31, not diagnosed). `git status --short` showed `M` on a file immediately after a chain `awk ... > f.new && mv f.new f`, then `git diff` was empty and the next `git status` was clean. Cause not established (hypothesis: the status was read mid-chain, not mtime). If you see an `M` that `git diff` does not confirm, run `git diff HEAD <file>` before acting.
    - **A `git commit` block may run twice** (observed 2026-10-31: two commits with the same message, 12 seconds apart; the second contained a duplicated `Files touched` entry). The `commit-msg` hook validates trailers, not duplicates. After any commit, run `git log --oneline -2`: if two commits share the same subject, the block ran twice. Fix: `git reset --hard <the first>`, then re-verify the working tree.
    - **`block=$(printf ...)` with nested quotes is unreliable** (observed 2026-10-31, three failures in one session). A multi-line string built with `printf` and captured in a variable, containing apostrophes and nested double quotes, breaks the shell quoting. Fallback: write the block to a file with a quoted heredoc (`cat > /tmp/f << 'EOF'`), then pass it to `awk` with `-v b="$(cat /tmp/f)"`. The same rule as the code fence: the problem is nested quoting, not the length.
- **B2 — Verify after every edit.** `sed -n 'Np' file` to confirm. `git diff file` before `git add`. If the diff shows unrelated changes, stop. If between the sentinels there is no content, the command succeeded but was silent. If the content is not what was expected, stop and report it.
- **B3 — Output sentinel.** Every command must be wrapped in the sentinel, using this exact template: `echo ""; echo -e "\033[1m► OUTPUT START\033[0m"; <command>; echo -e "\033[1m► OUTPUT END\033[0m"; echo ""`. The sentinel text is **generated by echo**, never written literally in the command block — a line `► OUTPUT START` alone in bash fails with `bash: ►: command not found`. Bold is lost on copy/paste to chat: expected, not a bug. A command without sentinel is a violation.
- **B4 — Mutation protocol.** A *mutation* is `sed -i`, `cat > file`, `git mv`, `git add`. Rules: (1) **No two mutations in one block.** After a mutation, the *same* block contains a verification (`sed -n 'Np'`, `wc -l`, `grep -c`); if the mutation fails, the verification shows it. Never chain a write and a delete. (2) **`sed` patterns are ASCII.** For lines with emoji, arrows (`→`), or other non-ASCII, use line-number + full-line replacement (`sed -i 'Ns@.*@new@' file`). Multibyte patterns fail silently on Git Bash (observed 2026-10-31: `sed` with 🔴🟡 silently did nothing; a `sed d` on a stale line number deleted the wrong row). (3) **Heredoc writes are verified.** After `cat > file`, run `wc -l file` and `head -3 file` in the same block. (4) **On failure, stop.** Do not retry with a different pattern in the same block; report and fix in the next command.

**C. Git and quality**

- **C1 — Commit and push.** Work in the local repo, commit in Git Bash, push. The GitHub web interface is for emergencies only. Every change: edit locally → `git add` → `git commit`. Small, atomic commits (one per file, or per logical group), clear messages in English, imperative mood. A rename is ONE logical commit, not one per occurrence. **Push at the end of the session or at a checkpoint, not after every commit.** *Phase 1 (now):* ruleset disabled, single maintainer, direct push to `main` allowed, PR optional. *Phase 2 (when an agent gets a write token):* ruleset active, branch + PR mandatory, no direct push. See `docs/context/AGENT-POLICY.md`.
- 🛑 **C2 — Quality gate (baseline, mandatory).** The pre-commit hook (`.githooks/pre-commit`, one-off install: `git config core.hooksPath .githooks`) runs fmt + clippy + test only if Rust files are staged. The hook does **not** replace the full check: also run `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin pcfm-test-runner -- --check --seed 42`. If anything is red, stop: do not close, do not push, do not "fix later".
- **C2-bis — Beyond the baseline (encouraged).** The goal is a clean, optimized, secure repository, not merely passing the commands above. If your toolchain or model capabilities allow additional checks — e.g. `cargo-audit`, `cargo-deny`, `miri`, coverage (`cargo-tarpaulin`), mutation testing, fuzzing, manual review of hot paths — run them and report the result. Do not limit yourself to the baseline.
  - **Caveat (D2 applies).** "Optimized" means *verified* optimization: reduce allocations, profile, remove redundancy, **without changing behaviour**. Any change to the simulation model requires an ADR and a `SIM_VERSION` bump. Do not introduce `unsafe`, `HashMap` in simulation paths, or other banned patterns in the name of optimization.

**D. Context and doctrine**

- 🛑 **D1 — Honesty and ideas.** If you have not read a file, say so. Do not invent numbers, commit hashes, or file contents. "I don't know" > invention. **When facing ambiguity, or when the context has not been fully read, prefer completeness over speed: read the full context before acting.** New ideas go to `docs/context/IDEAS.md` (status `sketch`, with prerequisites and open questions) **before** discussion, not after. Do not implement from an idea until it has an ADR (if it touches doctrine) or a step in `SESSION-STATE.md` (if it is a task).
- 🛑 **D2 — Doctrine and codebase.** No `unwrap`/`expect`/`panic!` in simulation paths. No `HashMap` in simulation paths. No `unsafe` outside `pcfm-bridge` (future). `pcfm-hidden` contains Plane A: no public fields, not `Serialize`, custom `Debug`; the only door is `resolve_*` + `drain_observations`; never add getters on latent values. `pcfm-testkit` has no `pcfm-*` dependencies. The simulation model is a prototype: do not "optimise" without an ADR and a `SIM_VERSION` bump. The doctrine wins: if a change contradicts `00-foundations`, a doctrine ADR is required. `docs/adr/` contains only ADRs.
- **D3 — Bee doctrine.** A structure is added only when concrete content requires it *today* — code that uses it, or a test that verifies it. A structure that only a *future* consumer would need is an idea, not a design: it goes to `IDEAS.md`, not into an ADR, a type, or a level. When the content is little, the temptation is to design much; resist it. This applies to the agent's proposals as much as to the maintainer's.
- **D4 — Decision pre-mortem.** After any **operational decision** — anything that changes the protocol's behaviour (a rule, a new file in `docs/context/`, a protocol step, a hard-stop, a closing/startup step) — before writing it, ask four questions and state the answer with evidence:
    1. **Coherence:** if everything decided so far were implemented, would it contradict an existing rule? If yes → fix or amend, do not proceed.
    2. **Leak:** does it open a way to appear compliant without being compliant? (self-controlled conditions, unverified fields, silent exceptions.) If yes → close the leak before writing.
    3. **Existence:** does this already exist — in this prompt, in Git, in a standard tool? If yes → use the existing one, do not duplicate.
    4. **Blast radius:** what will it weaken downstream? Which future rule, gate, or artifact does it make harder to enforce? This is the one judgment-bound question: if you cannot see the downstream effect, say so — it is a finding, not a pass.

    Output (evidential, not declarative): `pre-mortem: coherence=none; leak=none (checked: self-controlled condition, unverified field, silent exception); exists=none (checked: prompt, Git, standard tool); blast=none (checked: D1-D5, C2, E2-E5)`. A finding is classified by the exit clause below.

    **Self-application:** D4 applies to itself. The first iteration of any rule change is D4 on D4.

    **Mandatory triggers:** (a) new rule; (b) removed rule; (c) new file in `docs/context/`; (d) any change to D1/D2/C2, startup, closing, or a closing step; (e) any decision the maintainer declares will be implemented. For minor edits (typos, formatting, moving bullets within a section) the pre-mortem is implicit and produces no output.

    **Exit clause (objective, pure vs gap).** A finding **blocks** the session (pause, no closing) if it is **pure**: (a) exposes a latent (D2); (b) violates an invariant (`SIM_VERSION`, `chain_hash`, schema); (c) touches a doctrine ADR or a rule in this file that the finding itself would weaken; (d) is **not tracked** as an OI or step with an executable first action (§F). Otherwise it is a **gap**: it goes to `SESSION-STATE.md` with the verification action, the closing commit carries `Deferred: OI-XX` (validated by the hook), and the session closes. **Tracking does not exempt a D2 or invariant violation** — (a) and (b) always block, even if tracked. Default when in doubt: pure.

    **Filter agent (FA).** The maintainer is the FA for **gaps that require judgment** (D4 question 4, D5's visible-as-fallback conditions). FA does not apply to **pure** findings: those block per the exit clause until the maintainer decides. Invoking the FA is **always a pause (A2), in prose** — no commands, no file edits, until the maintainer writes "riprendi". The agent presents the gap **once** (finding, options, default, cost) and does not iterate on a fix without a decision. No decision → the gap goes to `SESSION-STATE.md` Open items with a first action, and the session proceeds (E3).

- **D5 — Mechanical and objective (visible as fallback).** The default lens for every discussion and every rule: a condition should be verifiable mechanically and objectively, by the maintainer, without trusting the agent's word. Where the condition is **inherently judgment-bound** (e.g. E2's "when the work seems complete", D4's "blast radius"), the rule must at minimum produce a visible artifact — a commit trailer, a report field, a `git log` entry. A condition controlled only by the agent's judgment, with no artifact, is a leak, even when the agent is honest. This applies to every rule in this file, including D5 itself.

**E. Session lifecycle**

- **E1 — Startup ritual mandatory.** At the start of every new session, before proposing anything, execute the Startup ritual. Not optional. If a step is impossible (e.g. no network for `git pull`), report the failure and proceed with what is possible. Never invent output.
- **E2 — Closing ritual: explicit approval, with "chiudi" as the only closing command.** The closing ritual is never automatic. When the work seems complete, pause (A2), present a status report (what was done, what was not, what is open), and wait. The closing executes **only** on the exact word **"chiudi"**, written as such (A2: no equivalent). "Riprendi", "ok", "va bene", silence, an "ok" to an unrelated question, or the natural end of a task are **not** approval to close: they do not close.

    **Green-path (reduced friction, not reduced approval).** When all of the following hold — (a) C2 green (full quality gate, no exception); (b) Gate A green; (c) Gate B green, including the manual checks B2, B4, B5, each declared in the status report; (d) Step 2b (rules self-audit) clean, or no rule was touched this session; (e) no finding classified as 🔴 during this session remains open without a `Deferred: OI-XX` trailer in the closing commit (validated by the hook: the OI must exist) — the agent presents a **short** status report ("all green, nothing to decide, details: …") and still waits for **"chiudi"**. The green-path reduces the report, not the approval. If any condition is false, the pause is mandatory and the maintainer decides.

    **Rollback** for a wrong closing: `git revert <commit>` + push (see `AGENT-POLICY.md` §9). The report always opens with `SESSION CLOSED — no further work executed (E3)`.
- **E3 — After the closing, the default is "track", not "execute".** Once the bookmark commit is made, the session is closed. Any new work that emerges afterwards is added to `SESSION-STATE.md` as a step (with its first concrete action) **and is not executed in the same session**. Three exceptions, and only three: (a) the bookmark is *internally inconsistent* (e.g. `Last commit verified` or `Files touched` does not match reality) — a broken bookmark is worse than an incomplete one; (b) a quality-gate failure (C2) is discovered; (c) the maintainer explicitly asks for the work to be done now. "It would be useful for the next session" is **not** an exception: that is precisely what the step list is for. The test is: *if this is not done now, can the next session reconstruct the state from the repo?* If yes, track it. If no, do it. This rule exists because every reopening after the closing makes the bookmark less trustworthy, and the bookmark is the memory of the project.
- **E4 — Bookmark update.** The bookmark is updated in three cases: (a) a significant step is closed; (b) before a push (the bookmark update is the last commit before the push); (c) the context limit approaches. In all three: write `SESSION-STATE.md` complete according to the Bookmark contract, commit, report. In emergency mode (c), no new work starts: the session closes, the next session resumes from a coherent bookmark.
- **E5 — Ideas from pause.** On "riprendi" after a pause (A2), if ideas emerged during the pause, the **first command** writes them to the right place before any agreed work resumes: design ideas to `IDEAS.md` (status `sketch`, with prerequisites and open questions); operational rules to this file; project decisions or open items to `SESSION-STATE.md` Open items. E5 is the operational trigger of D1; D1 remains the principle.

**F. Bookmark contract**

`SESSION-STATE.md` has a fixed structure. Sections, in this order:

1. **Last commit verified** (S1 semantics: the commit before the bookmark update).
2. **Open items** — decisions deferred, pending verifications, ideas in flight. Each entry: what, why open, first action to close it. Open items are not the Plan: they are what is *not decided*, not what *will be done*.
3. **Plan** — ordered steps with priority. Each step: what, dependency, first concrete action (the command to run, the file to read, the question to ask). A step without a first action is a step the next session will skip.
4. **Files touched this session** (dated).
5. **Done (storico)** — closed items, with commit ranges.

The startup ritual extracts §1–§4. The closing ritual updates all five. Open items and Plan are distinct: Plan = what will be done; Open items = what is not yet decided or verified.

**G. What Git already does (use it, do not reimplement)**

If Git provides the mechanism, use it. Session metadata lives in **commit trailers**, not in chat prose, so it is verifiable from `git log` at any later date.

- **Commit trailers** (block of `key: value` lines at the bottom of the commit message, after a blank line). The `commit-msg` hook (`.githooks/commit-msg`, one-off install: `git config core.hooksPath .githooks`) validates them. Template: `git config commit.template .gitmessage` (one-off, same install step). **The hook fires when `docs/context/SESSION-STATE.md` is staged** (closing commit), not by subject: a closing commit with an arbitrary subject is still validated. Required on closing commits:
    - `DPs-Applied:` — which decision patterns were used (`DP-01, DP-03` or `none`).
    - `Rules-Audit:` — output of Step 2b (`N changes, 0 contradictions, 0 leaks`, or `findings=[...]`).
    - `Verified:` — `C2-green` when the full quality gate ran and passed.
    - `Decisions:` — what the session decided and where it went (`N (X -> IDEA-006; Y -> OI-15)`, or `none`). This is the trailer that captures the residue of the conversation: a design decision, an open question, an unexpected observation. Written by the agent from a reread of the session, not from memory.
    - `Deferred:` — an explicit deferral of a 🔴 or a step (`OI-XX (reason)`), only if the referenced OI exists in `SESSION-STATE.md`.
- **Reading trailers**:
    - `git log -1 --format='%(trailers:key=DPs-Applied,valueonly)'`
    - `git log --format='%h %s | %(trailers:key=DPs-Applied,valueonly)' -20`
- **DP archival check** (no counter, no date field). Run at every closing if active DPs ≥ 3. Two commands:
    - `git log --format='%h %(trailers:key=DPs-Applied,valueonly)' -6 --grep='checkpoint\|session-state'`
    - For each active DP not listed in the two most recent closings, propose archival in the report. Archival is a maintainer decision, not automatic.
- **Regression isolation**: `git bisect run <script>` isolates the commit that introduced a failing behaviour (the anti-leak test is the script).
- **Post-hoc annotations**: `git notes add` on an old commit, never rewrite history.
- **Release exclusion**: `.gitattributes` with `export-ignore` for `docs/context/` and `DECISION-PATTERNS.md` when packaging.

If a standard tool already does what we need, use it: `awk` over `sed` for long replacements (B1-bis), `git log --format` over hand-parsed prose, `git revert` over re-edit for rollback.

**Rule precedence:** the hard stops (D1, D2, C2) win over everything else. D1 (honesty) first. In any other conflict, ask the maintainer.

## Startup ritual (mandatory — see E1)

At the start of every new chat, **before proposing anything**, ask the maintainer to run these commands in order and paste the output. Do not assume the state described in this prompt is still true: verify it.

### 1. Repo alignment

    git status --short
    git pull --ff-only
    git log --oneline -5
    git rev-parse HEAD

Check: working tree clean (or only known untracked files), HEAD matches `Last commit verified` in `SESSION-STATE.md` (semantics S1: it points to the commit *before* the bookmark update), no unexpected commits.

### 2. Read context (bookmark, reports, backlog)

Three reads, in this order, all required.

**Bookmark.**

    cat docs/context/SESSION-STATE.md

Extract sections 1-4 of the Bookmark contract (§F above).

**Analysis reports (continuity).**

    ls docs/context/analysis/*.md

For each report in `docs/context/analysis/` (not `analysis/archive/`), read the executive summary and any finding still open. If a report says "Pass" but has unverified findings, do not trust the verdict: verify each finding with `grep`/`sed` before acting.

**Ideas backlog (what is not decided yet).**

    grep -nE "^### IDEA-|^- \*\*Status:\*\*" docs/context/IDEAS.md

Read the ID, title, and status of every idea. Do not implement from `IDEAS.md`. The purpose is to know what has already been thought of (and rejected), so the session does not re-propose it or reopen a REJ.

**Agent policy (phase rules, push, rollback) — grep only.**

    grep -nE 'Phase 1|Phase 2|ruleset|push|rollback' docs/context/AGENT-POLICY.md | head -15

Read the matched lines. If the file structure changes, update this grep in the same commit (Gate A checks `Last updated`).

**Decision patterns (observed defaults) — grep only.**

    grep -nE '^## DP-|^   - (Source|Rule):' docs/context/DECISION-PATTERNS.md

Read the IDs and the two-line rule. A DP applies only to an identical case; a variant requires asking. Max 10 active, archivable after two consecutive closings with `DPs-Applied: none` (visible via `git log`).

### 2-bis. Bookmark coherence (fast gate)

    scripts/gates.sh startup

Runs the fast subset of the gates: A1, A3, B1, B3, B6, B7 (and reports B2 as manual; A2 is skipped because step 4 runs the tests). It reads the bookmark you just read, so run it **after** step 2 and **before** step 3: if it is red, the bookmark is internally inconsistent and the session starts on a false base.

If red: report it to the maintainer **before** proceeding. Do not fix it silently: a broken bookmark is worse than an incomplete one, and the maintainer must decide whether to reopen the previous session (E3a) or proceed with a declared caveat.

**Why these checks run twice.** A1, A3, B1, B3, B6, B7 run here and again at closing step 3, on purpose: here they catch a stale bookmark *before it is used*; at closing they confirm the bookmark is still true *after the session's edits*. The two runs cost under 0.1 s together and catch different failures. A2 is skipped here because step 4 runs the tests.
### 3. Read foundations (doctrine, rules)

Two reads, in this order, both required.

**Doctrine (non-negotiable).**

    cat specs/00-foundations.md

At minimum: §1 (pillars), §2 (design pillars), §3 (Three Planes), §5 (banned things).

**Operating rules.**

    cat docs/context/NEW-SESSION-PROMPT.md

This very file — operating rules (A1, A2, B1-B4, C1, C2, D1, D2, D3, E1-E5), rule precedence, the startup and closing rituals, what NOT to do.

### 4. Confirm the numbers

    cargo test --workspace 2>/dev/null | grep "test result"
    ls crates/ | grep pcfm | wc -l
    for f in docs/adr/*.md; do sed -n '3p' "$f"; done | grep -c 'Status:\*\* accepted'

Compare with `SESSION-STATE.md` and this prompt. If they diverge, the docs are stale — report it before proceeding.

### 5. Report to the maintainer

Say explicitly:
- What you read (files, line ranges)
- What you did **not** read (and why)
- The state of the repo (aligned? clean? numbers?)
- The next step according to `SESSION-STATE.md`
- Any divergence between docs and reality

**Do not propose a step before step 5 is done.** "I have not read X" is a valid state, not a failure.

## What NOT to do

- Do not give "lots of comments" scattered through the code: the codebase is already documented.
- Do not assume the status of an ADR from memory: read `STRUCTURE.md` §5 (single source of truth). ADR-012, 013, 015, 016, 018, 019 are **accepted**, not proposed — but verify, do not assume.
- Do not assume ADR counts: they change. Use `STRUCTURE.md` §5 for the current number.
- Do not propose Python scripts (Python is not installed). Use Bash/Git Bash.
- Do not assume CI exists: there is no `.github/workflows/`. Clippy/test/build/fmt/check are **manual**, and must be run after every change (the pre-commit hook helps for Rust).
- Do not reactivate the ruleset on `main` without explicit instruction. It is disabled in Phase 1.

## Closing ritual (mandatory — see E2)

After the maintainer approves the closing (E2), run these checks in order. If any is red, stop and fix before closing.

Steps run **in order**. A step is "done" only when its output is pasted in chat and is green. Do not start step N+1 before step N's output is shown. If a step is skipped or run out of order, say so explicitly and re-run it before closing. The quality gate (step 1) and the pre-close gates (step 3) must be green **before** the bookmark update (step 4), not after.

### 1. Repo alignment and real verification

    git status --short
    git log --oneline -5
    git rev-parse HEAD

If there are local commits not pushed:

    git push
    git log --oneline -3
    git status -sb

Then the full quality gate, as defined in **C2** (baseline mandatory) and **C2-bis** (additional checks encouraged). The baseline commands are not repeated here: C2 is the single source of truth for the gate.

If anything is red: **stop**. Do not declare the session closed. Fix first.

### 2. Session capture (before gates)

Capture session patterns into the rule files, **not** into the report.

**2a — Workflow.** For each command that failed and was retried with a different approach, tool that turned out unavailable, anchor that went stale, **or command that succeeded with an unexpected observation** (odd output, transient state, a result that contradicts a previous run): if likely to recur, add to B1-bis (observed) or B4 (mutation). If one-off, no action.

**2b — Rules self-audit.** Only if this session modified `NEW-SESSION-PROMPT.md`, `DECISION-PATTERNS.md`, a hard-stop, the startup, or the closing. If no rule was touched, write `Rules-Audit: skipped, no rule changes` as the closing commit trailer.

For each rule change:
1. Which existing rule does it touch? Cite `file:line`.
2. Does it contradict one? If yes, fix or revert **now** (E3(a) applied to rules).
3. Does it open a leak? Who controls the condition? A self-controlled condition is a leak.
4. Does it assume a mechanism that does not exist (counter, CI, second reviewer, hook)? If yes, create the mechanism now or do not write the rule.

If any answer is bad: **pause (A2), ask the maintainer.** Do not trace. Do not defer. The closing is paused.

Output trailer: `Rules-Audit: findings=[...]` (paused) or `Rules-Audit: N changes, 0 contradictions, 0 leaks` (clean).

**DP capture.** If a recurring judgment pattern emerged (not a workflow fix), add it to `DECISION-PATTERNS.md` with `Source:` pointing to the session and a commit hash. No source → no DP.

**2c — DP archival check** (only if active DPs ≥ 3). Run `git log --format='%h %(trailers:key=DPs-Applied,valueonly)' -6 --grep='checkpoint\|session-state'` and compare against active DPs. A DP absent from the two most recent closings is proposed for archival in the report; archival is a maintainer decision, not automatic.

### 3. Pre-close gates

Run `scripts/gates.sh` first: it performs the mechanical subset of Gate A and Gate B and exits non-zero on failure. Then do the manual checks (B2 Plan vs Detail order, B4 Prime azioni match reality, B5 Files touched vs git status) as described below.

Two gates, run in order. If either fails, fix and re-run it before step 4. Do not note a failure for later; the bookmark must be true when the closing commit is made.


**Gate A — documentation coherence.**

`scripts/gates.sh` implements three checks:

- **A1** — ADR count: the rows in `STRUCTURE.md` §5 (`| 0NN |`), the rows in `specs/_index.md` (`| [0NN]`), and the files in `docs/adr/` (excluding `000-template.md`) must be the same number.
- **A2** (closing only) — the README badge `tests-N_passing` must equal the real test count from `cargo test --workspace`.
- **A3** — no hardcoded counts (`N tests`, `N crates`, `N ADRs`) in `NEW-SESSION-PROMPT.md`.

If any is red, the stale file is fixed and the fix is committed in the same session. The script is the source of truth; this text describes what it checks.

If you touched a file X (renamed it, changed its path, changed its content), verify that references to X are updated everywhere:

    git diff --name-only HEAD~N..HEAD | while read f; do
      base=$(basename "$f")
      grep -rn "$base" --include='*.md' --include='*.rs' . | head -3
    done

(replace `N` with the number of commits in this session)

If `docs/context/NEW-SESSION-PROMPT.md` changed this session, `Last updated` (line 5) must mention the change. `sed -n '5p' docs/context/NEW-SESSION-PROMPT.md` and compare against `git diff --name-only <last-verified>..HEAD`. If the file changed and `Last updated` does not say so, update it in the same session.

**Gate B — fresh-session simulation (mechanical).**

Do not answer this from memory. Simulate: the next session reads `NEW-SESSION-PROMPT.md`, runs the startup ritual, reads `SESSION-STATE.md`, the two latest reports in `analysis/`, and `IDEAS.md` (headers and status). Would it resume without asking a question the bookmark already answers?

Run these checks. A failed check is fixed before step 4, not noted for later.

1. **Next step has a first action.** The first open step in the Plan must say what to do first (a command, a file to read, a question). `grep -n "Primo passo\|First step\|Ratificare" SESSION-STATE.md`. If the first step has no action, add it.
2. **No contradiction between Plan and Detail.** The Plan's first step and the same step's Detail/DoD must agree on the order (e.g. "ratify then test" vs "test then ratify"). Read both. If they disagree, fix the Plan (it is what the next session reads first).
3. **Pending actions are Open items.** Any action that is not a Plan step (a ratification, a verification, a decision) must be an OI-NN. `grep -c "OI-" SESSION-STATE.md`. If a pending action is not listed, add it.
4. **"Prime azioni" match reality.** For the first open step, the "Prime azioni" entry must not describe work already done. Read it; if it says "write X" and X exists, update it.
5. **Every touched file is in `Files touched` (except the bookmark itself).** The closing commit is the one that contains `SESSION-STATE.md`, so it cannot be compared against itself. Run `git status --short` before the commit: every modified/added file that is **not** `SESSION-STATE.md` must appear in the `Files touched` list. Then, after the commit, run `git show --stat HEAD` and check that the only file not pre-listed is `SESSION-STATE.md`. If another file appears, the list was incomplete.
6. **Proposed/deferred ADRs are surfaced.** `for f in docs/adr/*.md; do sed -n '3p' "$f"; done | grep -c proposed`. Each `proposed` ADR that blocks a step must appear in the Plan or Open items.
7. **New reports are cited.** `ls -t docs/context/analysis/*.md | head -2`. Each must be cited in the bookmark (as a finding, an OI, or a step).

If any check fails, update `SESSION-STATE.md` and re-run the failed check. The bookmark is the memory of the project; a memory that contradicts itself is worse than a short one. Gate B is a **gate**, not a reflection: step 4 does not start until all seven pass.

### 4. Bookmark update

Update all five sections of the Bookmark contract (§F above). In particular: `Last commit verified` (S1 semantics), Open items (new verifications, decisions deferred, ideas), Plan (next steps, each with first concrete action), Files touched this session (dated), Done (closed items, commit ranges).


**Before writing the bookmark, a conversation sweep.** Reread the session (since the last closing) and list every design decision, open question, and unexpected observation that is not yet in a file. Each goes to its home: design ideas → `IDEAS.md` (status `sketch`), open questions → Open items, workflow lessons → B1-bis, decisions that change the protocol → the rule files (Step 2b already covers them). Then write the `Decisions:` trailer: `N (X -> IDEA-006; Y -> OI-15)`, or `none`. The hook rejects a closing commit without it. This is the one step that cannot be made fully mechanical: the trailer is the artifact, the reread is judgment.

### 5. Commit and push (before the report)

The checkpoint commit is **one logical commit**:

    git add docs/context/SESSION-STATE.md
    git commit -t .gitmessage
    git push
    git status -sb

The `pre-push` hook (`.githooks/pre-push`) runs `scripts/gates.sh` (closing mode) and blocks the push if red. The gate runs at the moment it matters (the push), not twice.

The commit message uses `.gitmessage` as template. Compile subject, optional body, and the **required trailers** (validated by `.githooks/commit-msg`):

- `DPs-Applied: DP-01, DP-03` or `DPs-Applied: none`
- `Rules-Audit: N changes, 0 contradictions, 0 leaks` or `Rules-Audit: skipped, no rule changes` or `Rules-Audit: findings=[...; unresolved: OI-XX]`
- `Verified: C2-green`
- `Deferred: OI-XX (reason)` — optional, only if the OI exists in `SESSION-STATE.md`

`--no-verify` is a protocol violation, not a shortcut. If a push was made with `--no-verify`, the closing report says so explicitly.

`git status -sb` must show `## main...origin/main` with no `[ahead]` or `[behind]`. If it does, the session is not closed.

### 6. Final report to the maintainer

State explicitly:
- What was done (one line per commit)
- What was verified (real tool output, not claims)
- What was **not** done (and why: blocked, deferred, out of scope)
- The state of the repo (aligned, clean, quality gate green) — verified after the push
- The next step for the next session (what `SESSION-STATE.md` says)
- **The Plan table, presented in chat** (not just written in the file). The maintainer must see it before the session ends. If you skip it, say why.

If you have not run one of the checks in this ritual, say so. "I skipped X" is honest; "I ran X" when you did not is a lie.
````
