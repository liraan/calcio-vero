---
title: "Session facts — 2026-11-07"
version: "1.0"
status: "record"
last_updated: "2026-11-07"
conforms_to: "SESSION-STATE.md §F, E2/E3"
---

# Session facts — 2026-11-07

Facts about the closing of the previous session that `git log` does not
tell, and one meta-finding about the closing itself. Not decisions, not
tasks: facts, so the next session can reconstruct what happened.

## 1. Facts that `git log` does not tell

- **ADR-035 was interrupted mid-way.** `git log` shows `3d0101d` as a
  clean commit; the residual (11 points across 7 files) was reconstructed
  in the previous chat, and gate A1 was RED at the start of that session
  for this reason. The commit message does not say so.
- **Two foci in one session** (ADR-035 doctrine + OI-71) against the
  "one session, one focus" method in
  `2026-11-07-session-notes-log.md`. The note does not record the
  violation.
- **Choice B (full coherence instead of the minimum to make the gate
  pass) was a verbal maintainer decision**, not recorded. A future agent
  does not know the maintainer prefers it. The recorded artefact is this
  paragraph.
- **The D4 pre-mortem for the three B1-bis rules was done in chat**; the
  trailer says `Rules-Audit: 3 changes, 0 contradictions, 0 leaks` but
  the per-rule reasoning is in no file. D5 asks for a visible artefact,
  not the conclusion alone.

## 2. Meta-finding on the closing (E2/E3)

The previous closing took the green-path (E2). The green-path requires
"(e) no finding classified as critical remains open without being
tracked as an OI in the bookmark". Three findings were in chat and in no
file: the `00-foundations.md` §4 wording (not "migration in progress"),
the `01-rng.md` / ADR-029 status headers (stale, not verified this
session), and the un-isolated golden cause. The first two, if confirmed
with `grep`, are D1 in a persistent artefact; the third is a D1 claim in
a commit message that was not verified. **The green-path was taken with
finding (e) in doubt.** Remedy: the three findings must be tracked as
OIs (see Task 2 / OI-78 — they are folded into the reconciliation, not
duplicated as three separate OIs) before the next closing, and the
green-path criterion (e) must be read as "traced in a file, not only
declared in chat".
