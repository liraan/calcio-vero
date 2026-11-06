---
title: "Repo description and external pages"
version: "1.0"
status: "reference"
last_updated: "2026-11-07"
conforms_to: "README.md (the source), OI-69 (donate), the 2026-11-07 session"
---

# Repo description and external pages

**What this is.** The texts that live *outside* the repository — GitHub
"About", Ko-fi bio, Ko-fi frontpage — kept here so the next session does
not rewrite them from scratch. The README is the source: if a text here
diverges from the README, the README wins (D1).

**Not a public page.** GitHub and Ko-fi render their own copies. This
file is the artefact, not the source.

---

## GitHub "About" (description, variant b)

> Calcio Vero — an open, offline-first football management game. Deterministic match engine, a community alternative to commercial football manager games. Human-directed, AI-assisted: the design, the doctrine and every decision are human; the AI proposes and executes under explicit rules, written in the repo.

**Tags (10):** `football` · `football-manager` · `football-simulation` · `football-game` · `simulation` · `game-engine` · `rust` · `deterministic` · `offline-first` · `open-source`

---

## Ko-fi bio

> Calcio Vero — an open, offline-first football management game. Deterministic match engine, community alternative to commercial FM games. Human-directed, AI-assisted. Donations fund development time; they do not buy influence over the roadmap.

---

## Ko-fi frontpage (chosen: version 1, no Three Planes)

**Calcio Vero — Deep Simulator**

An open, offline-first football management game. Transparent by design. Deterministic by construction. Yours by default.

Not a Football Manager clone: where commercial games optimise for licensed fidelity, Calcio Vero optimises for explainability and ownership.

No telemetry. No closed data feeds. No hidden CA/PA scalars. Every formula is public, every decision is in an ADR, every save is reproducible from its seed.

Code and doctrine: https://github.com/liraan/calciovero

---

## What was rejected, and why

- **Ko-fi frontpage v2** (Three Planes instead of the no/yes list): more elegant, but a cold reader learns less. The Three Planes answer "how does it work inside", which belongs in the README, not on a landing page.
- **Ko-fi frontpage v3** (the full README): too long, Ko-fi cuts around 700 characters.
- **"free forever"** in any text: no artefact backs it. `LICENSE-MIT` / `LICENSE-APACHE` exist, `"free and open"` is descriptive; "forever" is a promise.
