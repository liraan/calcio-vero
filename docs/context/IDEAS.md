# Ideas — PCFM

**Purpose:** a curated backlog of ideas that are not yet decisions. Unlike `PROJECT-IDEA-LOG.md` (archived historical record), this file holds ideas in motion: sketches, open questions, trade-offs.

**Admission rule:** an idea enters only if (a) it has been discussed, (b) it is not already decided or placed elsewhere (ADR, module, `WORLD-MODEL.md`), (c) it is worth tracking. Rejected ideas are kept in a separate section, with motivation, to avoid re-discussing them.

**Promotion rule:** when an idea becomes a decision, mark it `promoted → ADR-NNN` (or `promoted → module`) and keep it here for traceability. Do not delete.

**Categories:** Player model, Match engine, Career (L2), World, UX / Presentation, Infrastructure.

---

## Open ideas

### IDEA-PM-001 — Dynamic attributes over a season
- **Status:** sketch
- **Category:** Player model (L2 career)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** attributes evolve over a season: physical decline with age, technical improvement possible, confidence modulates match performance (over/under-performance).
- **Open questions:** age curves; training and game-time effect; confidence source (wins? form? role?); determinism (Plane A vs B); interaction with ADR-016 psyche axes.
- **Critical note:** "losing improves technique" is a weak heuristic — it can create a perverse incentive (losing is good). Better: the loss is the context, professionalism (an ADR-016 axis) is the mechanism. Without professionalism, losing teaches nothing.

### IDEA-PM-002 — Multi-attribute success range per action
- **Status:** sketch
- **Category:** Player model (match resolution)
- **Source:** conversation 2026-10-27 (user, proposal 2)
- **Idea:** every action (shot, pass, duel) is resolved by a *combination* of attributes — technical, physical, psychological — that together define a **success range**. Example: a shot depends on shooting, composure, finishing, stamina, balance, concentration, aggression, teamwork. No single attribute decides the outcome.
- **Open questions:** the formula (weighted sum? thresholds? probability distribution?); which attributes per action; how the range is computed deterministically (integers / fixed-point); how it interacts with the zone of influence (ADR-019 D8) and with the two-phase events (D11).
- **Critical note:** the formula is deferred to `docs/calibration/`. Without calibration, the weights are arbitrary. The principle (multi-attribute, not single-attribute) is the decision; the numbers are not.

### IDEA-PM-003 — Confidence as a psychological axis or modulator
- **Status:** sketch
- **Category:** Player model (L2 psyche)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** ADR-016 has three active axes (professionalism, ambition, pressure). "Confidence" (fiducia) is not one of them. It could be a fourth axis, or a modulator that depends on form/wins and affects performance.
- **Open questions:** is confidence a stable axis (like professionalism) or a dynamic state (like form)? How does it interact with pressure? Does it belong to ADR-016 (content) or to the match engine (parameter)?
- **Critical note:** ADR-016 fixed three axes as a binding decision. Adding a fourth requires an amendment (or a new ADR). Do not treat this as a minor addition.

### IDEA-PM-004 — Concentration as a consumable cognitive resource
- **Status:** sketch (deferred — depends on prerequisites not yet in place)
- **Category:** Player model (L2 psyche) / match engine (L0 consumption)
- **Source:** conversation 2026-10-28 (user)
- **Idea:** a player has a **concentration** resource that is consumed during
  a match. An action that is not instinctive (a function not fully learned)
  costs more concentration, and more physical fatigue. Familiarity with a
  function changes the player's effective attributes; a function not fully
  learned forces the player to "think" each action instead of executing it
  instinctively.
- **Relationship with existing decisions:**
  - **Not pressure.** ADR-016 D1 defines `pressure` as the axis that drives
    performance in high-stakes matches. Concentration, as described here,
    is a *consumable resource*, not a stable trait. The observable effect
    must be **distinct** from pressure (e.g. choice errors, lost time,
    positional mistakes), otherwise pillar 5 (radical transparency) is
    violated: "he underperformed" could not be attributed to pressure or
    to concentration. This is the same reason `consistency` was rejected
    in ADR-016 D1.
  - **ADR-016 is binding.** It fixes three active axes
    (professionalism, ambition, pressure) and three inactive
    (loyalty, consistency, temperament, with `active: false`).
    Concentration is not in the six candidates: adding it as a seventh
    axis, or replacing one, requires an amendment or a new ADR.
  - **Not confidence.** `IDEA-PM-003` covers *confidence* (a modulator
    driven by form and wins). Concentration is a different concept:
    a resource consumed by cognitive load. Do not merge the two.
- **Prerequisites (why this is deferred):**
  1. **Dynamic physical fatigue in L0.** The idea says concentration is
     consumed *together with* physical fatigue. If L0 has no fatigue that
     decays during the tick, concentration would be the second floor of
     a house without the first. Verify whether `PhysicalStateMinimal`
     decays during a match or is static.
  2. **ADR-017 ratified** (roles are labels, not modifiers). The idea
     "positioning changes functions, not attributes" rests on it; today
     ADR-017 is `proposed`.
  3. **Attribute taxonomy (L2-psyche)**: a written list of attributes,
     each classified across the Three Planes (latent / observable / event),
     each with a distinct observable effect. Concentration is one entry in
     that taxonomy, not an isolated addition.
  4. **L0 vs L2 decided.** L2 defines *what* concentration is (an
     attribute, with familiarity and learning). L0 defines *how* it is
     consumed during a match. These are two ADRs, in that order. L0
     consumption requires an L0 hook that does not exist today.
- **Open questions:**
  1. Is concentration a **latent** (Plane A, via `resolve_*`), an
     **observable** (Plane C, via `IndexSnapshot`/`ProjectionProfile`),
     or an **event** ("lost concentration at minute X")? The three have
     different contracts.
  2. Does it share a pool with physical fatigue, or is it a separate pool?
     If they interact, how?
  3. Is the consumption deterministic and reproducible from the seed?
     (It must be: `MatchState::tick` is in the simulation path.)
  4. Does "familiarity with a function" grow over time (L1/L2 learning),
     or is it static per player? If it grows, it is not an L0 concern.
- **Critical note:** touching `MatchState::tick` or `resolve_*` requires a
  `SIM_VERSION` bump (see `CHANGELOG-SIM.md`) and a performance
  measurement, which today does not exist (`PROJECT-STATUS.md`:
  "Not yet measured"). Do not implement before the prerequisites above.

### IDEA-PM-005 — Player footedness (strong foot / weak foot)
- **Status:** sketch
- **Category:** Player model (attributes) / Match engine (L0 action modifier)
- **Source:** conversation 2026-10-29 (user)
- **Idea:** each player has a footedness: a preferred (strong) foot and a
  weak foot. Footedness modifies the quality/cost of actions resolved with
  the weak foot (first touch, pass, shot) and can influence trajectory
  preferences (e.g. a winger cutting inside to shoot with the strong foot).
  Real-world datasets model this simply (preferred_foot: left/right/both);
  FM-style games use two separate ratings, one per foot.
- **Three Planes (proposal, not decided):** footedness would likely live in
  Plane A as a stable identity trait, and be highly observable (Plane B/C):
  unlike hidden traits, estimation error is low. A candidate test case for
  the observation doctrine, subject to ADR.
- **Open questions:**
  1. Model shape: two separate per-foot ratings (FM-style) vs a single
     categorical field (left/right/both) plus one weak-foot rating — the
     latter is more aligned with the minimalist philosophy of PCFM.
  2. Which L0 actions read footedness (first touch, pass, shot, dribble
     direction?) and how it modifies them: action cost, success range (see
     IDEA-PM-002), or both.
  3. Is "both" a real value (ambidextrous players) or just the top of a
     weak-foot scale?
- **Critical note:** L0 is complete and ratified; adding footedness is a
  contract change — it requires a new module or a revision, and an ADR if it
  touches the match engine contract (ADR-019). Do not implement from this
  sketch.

### IDEA-RULE-001 — Contract for rule modules
- **Status:** discussing
- **Category:** Match engine (L0 rules)
- **Source:** conversation 2026-10-27 (ADR-019 discussion)
- **Idea:** refereeing rules (offside, fouls, cards) are behaviour, extensible by modules (ADR-012). The core provides primitives (positions, ball state, temporal events); rule modules read them and produce outcomes. The contract for rule modules is deferred to a dedicated ADR.
- **Open questions:** what a rule module may change (thresholds? priorities? the state machine?); how rule modules interact (conflicts, priority); how they are recorded in the save manifest (ADR-013); performance cost of per-event rule evaluation.
- **Critical note:** "behaviour open" must not become "anything goes": a rule module that rewrites the engine would violate ADR-012. The contract must define what is open and what is closed.

### IDEA-INFRA-001 — Session-start tooling
- **Status:** sketch
- **Category:** Infrastructure (process)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** a small script (or `make` target) that prints `SESSION-STATE.md` + `git log --oneline -5` + `git status --short` in a single block, to paste at the start of a new session. Reduces the number of manual commands.
- **Open questions:** script language (Bash, no Python); where it lives (`tools/`); whether it is versioned; how it handles encoding on Windows (Git Bash).
- **Critical note:** the script is convenience, not a replacement for the checkpoint rule. The bookmark must be updated regardless.

### IDEA-INFRA-002 — Agent tooling for Git
- **Status:** discussing
- **Category:** Infrastructure (agent)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** options for giving an agent (Warp, DeepSeek-TUI) controlled access to Git: MCP server (e.g. `mcp-server-git`), `gh` CLI wrapper, or DeepSeek-TUI with a write token. Phase 2 of `AGENT-POLICY.md`.
- **Open questions:** which option is mature enough; how to enforce the policy (branch, PR, no force-push) at the tool level; token scope and revocation; interaction with the ruleset on `main`.
- **Critical note:** no agent gets a write token until the ruleset on `main` is reactivated. The policy is already written (`AGENT-POLICY.md` §2–§4).

### IDEA-INFRA-004 — `pcfm-test-runner` covers installed modules and plugins
- **Status:** discussing
- **Category:** Infrastructure (testing / tooling)
- **Source:** conversation 2026-10-27 (user, next-session goal)
- **Idea:** extend `pcfm-test-runner` so that `--check` / `--list` / `--coverage`
  also account for scenarios and features contributed by installed modules
  and plugins, not only the 6 built-in L0 scenarios.
- **What exists today (verified in code, 2026-10-27):**
  - `pcfm-testkit::Feature` is `#[non_exhaustive]` and already has a
    `Plugin(String)` variant alongside the 26 built-in features
    (`crates/pcfm-testkit/src/feature.rs:13,87,118`).
  - `TestRegistry::register()` accepts any `impl Scenario`
    (`crates/pcfm-testkit/src/registry.rs:26`); the contract is open.
  - `pcfm-test-runner` hardcodes the discovery path: `check.rs:47-49`,
    `commands.rs:16-17`, `commands.rs:49-50` all call
    `features::register_all(&mut registry)`, and `features/mod.rs:15`
    documents that this is "the only entry point the rest of the binary
    uses".
  - `--coverage` denominator is `Feature::all_builtin()` only
    (`commands.rs:54`); `Plugin(_)` features are never counted.
- **What is missing (the actual gap):**
  1. A discovery step for installed plugin scenarios ("what is installed?").
  2. A way to feed them into the same `TestRegistry` that `pcfm-test-runner` builds.
  3. A report format that distinguishes built-in from plugin scenarios
     (`Feature::Plugin(_)` exists but is not surfaced).
  4. A decision on the coverage denominator: built-in only, or union.
- **Ambiguity to resolve before designing:** "cover installed plugins" can
  mean (a) run plugin-contributed test scenarios alongside L0's, or (b)
  verify that a plugin is loaded and functional (smoke test of the plugin
  system itself). They are different features. Pick one.
- **Constraints from existing decisions:**
  - ADR-012 §1: the plugin sheet is mandatory and validated. Discovery
    should read the sheet, not scan arbitrary directories.
  - ADR-013 §3: the manifest is part of the save and validated. The save is
    a natural discovery source, but only for an in-progress save; a fresh
    workspace has none.
  - "`pcfm-testkit` has no `pcfm-*` dependencies" (codebase rule). Any
    metadata type added to the testkit must respect this. `Feature::Plugin(String)`
    is already a minimal representation that does.
  - The plugin system itself is not implemented yet (no loader, no
    `pcfm-pack`). This idea can be **designed** now but **implemented** only
    after ADR-012's runtime contract exists.
- **Open questions:**
  1. Discovery: `sheet.toml` scan, save manifest, or explicit list passed to
     `pcfm-test-runner`? Each has different determinism and UX implications.
  2. Aggregation in the report: per-plugin rows, per-feature rows, or both?
     What is the stable machine-readable format?
  3. Coverage denominator: built-in only, union, or two separate lines?
  4. Does `Feature::Plugin(String)` suffice, or does the testkit need a
     plugin metadata struct (name + version + feature)?
  5. Does `--check` fail (non-zero exit) when a plugin scenario fails, or
     does it warn?
- **Critical note:** before designing, read (a) the 6 scenarios in
  `crates/pcfm-test-runner/src/features/l0_match.rs` to see what a good `Scenario`
  looks like, (b) `docs/adr/012-plugins.md` §1–§3 for the sheet contract,
  (c) `docs/adr/013-worlds.md` §3 for the manifest. A stale
  snapshot is not sufficient.

### IDEA-INFRA-005 — Separate diagnostics and benchmark from the test runner
- **Status:** discussing
- **Category:** Infrastructure (tooling / observability)
- **Source:** conversation 2026-10-28 (user)
- **Idea:** the test runner (`pcfm-test-runner`, `--check`, `--list`,
  `--coverage`, `--seed`) is now correctly named after ADR-020. Originally
  the crate was `pcfm-debug`, whose name suggested a debugger. Remaining
  scope after ADR-020 — two distinct directions:
  1. **Diagnostics** (log / trace of startup and runtime). Name must
     describe the function; candidates: `pcfm-log`, `pcfm-trace`,
     `pcfm-diagnostics` (plural, coherent with `pcfm-stats`). Final name
     decided in the ADR, after local analysis.
  2. **Benchmark** (performance measurement on user hardware, target
     `00-foundations.md` §6: 60 fps UI, season < 60 s). **Deferred** — the
     targets are not measurable today (no UI, no L1). Revisit when the
     targets become measurable.
- **What is missing before any code:** a local reading of
  `crates/pcfm-test-runner/src/main.rs` and `crates/pcfm-testkit/src/lib.rs` to
  establish (a) what `pcfm-test-runner` actually does today beyond `--check`,
  `--list`, `--coverage`, (b) what the `Scenario` trait exposes, (c) what
  surfaces (if any) should move to a separate diagnostics crate.
- **Relationship with IDEA-INFRA-004:** that idea is about the **runner**
  (covering installed plugins and scenarios). It stays valid regardless of
  the runner crate name. The rename to `pcfm-test-runner` has already
  happened (ADR-020, 2026-10-28), and IDEA-INFRA-004 has been updated to
  match.
- **Open questions:**
  1. Does the diagnostics crate share the `Scenario` contract with the
     runner, or does it need a different one?
  2. Does `pcfm-test-runner` become a wrapper CLI with subcommands (`run`,
     `list`, `coverage`, `replay`, `inspect`), or stay a single-purpose
     binary? The `pcfm-demo` split is already done (ADR-020).
  3. Hooks: how many, for which concrete use case? (Discussion 2026-10-28:
     minimal, no generic hooks "for the future".)
- **Critical note:** an ADR is required before any crate is created or
  renamed, because the change touches the doctrine (dependencies,
  determinism, `pcfm-testkit` as contract). The doctrine wins. No code
  until the ADR is accepted.

### IDEA-INFRA-006 — TUI renderer (event-driven ASCII) for `pcfm-demo`
- **Status:** sketch
- **Category:** Infrastructure (rendering / validation)
- **Source:** conversation 2026-10-28 (user)
- **Idea:** a terminal UI that renders a match as ASCII: pitch, 22 players,
  ball, goals, score, updated on match events. A **validation renderer**,
  not L3-ui: it proves that the simulation/rendering split works with a
  second renderer, and it makes the match visible without leaving the
  terminal.
- **Where it lives:** `pcfm-demo` (name = function, per ADR-020). No new
  crate. `ratatui` + `crossterm`, or raw ANSI if we want zero dependencies.
- **What the data allows today:** the event chain has ~19 events per match,
  each with a minute (not a second). `PositionSnapshot` exists only on
  *attempt* events (ADR-019 D11), not every tick. So the TUI would show
  **discrete frames**, not smooth motion. The ball would jump, not roll.
  This is honest but limited. **Do not fake interpolation**: it would show
  movement the engine did not simulate, against pillar 5 (radical
  transparency).
- **Decision deferred to a short ADR:** whether the coarse granularity is
  acceptable for a prototype (likely yes), and — only if smooth motion is
  wanted — whether `MatchState::tick` should expose per-tick positions
  (which requires `SIM_VERSION` bump, a dedicated ADR, and a performance
  measurement, none of which exist today).
- **Open questions:**
  1. `ratatui`/`crossterm` or raw ANSI? (Dependency vs control.)
  2. Frame cadence: one frame per event, or a fixed tick?
  3. What is shown: pitch only, or pitch + score + event log?
  4. Is it a one-off prototype, or a maintained tool?
- **Critical note:** an ADR is required before code, because a second
  renderer touches the renderer contract
  (`simulation_is_shared_and_renderer_agnostic`). The ADR must say
  explicitly that this is a **validation prototype in `pcfm-demo`**, not
  the L3-ui (ADR-005 chose Qt for that), and that no engine change is
  planned.

### IDEA-TACT-001 — Tactics as per-state functions

- **Status:** sketch
- **Category:** Tactics
- **Source:** conversation 2026-10-29 (user, discussed)
- **Idea:** replace the classic zone-grid tactics model with a model where
  each player is assigned **functions** in two states (in possession, out of
  possession), plus a team-level **transition system** (not a third state with
  per-player functions). The map becomes **output** (derived), not input. The
  player flags functions; the system shows the resulting shape.
- **Two states + one system:**
  - **In possession:** per-player functions (ceiling 6-8, to be decided)
  - **Out of possession:** per-player functions (same ceiling)
  - **Transition:** team-level rule (counter-press / drop back / maintain
    shape / tactical foul), not per-player
- **Defaults:** each role provides a default function profile per state. The
  player starts from the profile and modifies it. Role remains an anchor, not
  a fixed prescription.
- **Validation:**
  - Hard errors: structural (11 players, no double assignment where
    functions conflict)
  - Soft warnings: competence estimates, incompatible combinations with
    explanation
- **Map:** output, derived from functions. The player does not place players
  on a grid; the system shows the resulting shape.
- **Relationship with IDEA-PRES-001:** this idea **revises** the tactics
  section of IDEA-PRES-001 ("three phases, each with its own zone
  assignment"). The rest of IDEA-PRES-001 (view model discipline, screens,
  order) is unchanged.
- **Pre-requisites:**
  - `L2-tactics` module (not yet written; ADR-019 deferred 9:
    "role -> tactic -> parameter chain")
  - ADR-019 D3: tactical instructions passed as named parameters. The
    function model must become parameters the engine understands.
  - Possibly a new ADR if the function model changes the parameter vocabulary.
- **Open questions:**
  1. How many functions per state? (Recommendation: 6-8, mutually exclusive
     or clearly compatible)
  2. How do functions relate to existing roles? (Anchor + deviation, or
     replace?)
  3. Familiarity/cohesion over time (attributable to tactic, not state) — in
     scope or deferred?
  4. What is the function vocabulary? (Closed enum, like ADR-019 D2, or open?)
  5. Is the derived map rendered by the TUI, or is a list enough for v1?
- **Critical note:** this is a **sketch**, not a decision. Do not implement
  until `L2-tactics` exists and a dedicated ADR (if needed) is accepted.

### IDEA-INFRA-007 — In-match intervention (manual pause, engine hints)

- **Status:** sketch
- **Category:** Infrastructure (interaction)
- **Source:** conversation 2026-10-29 (user, discussed)
- **Idea:** the manager can **manually pause** the match at any time, change
  the transition rule (or other pre-defined instructions), and resume. The
  engine **suggests** notable moments via hints (visible, not blocking), but
  never pauses on its own. The manager decides.
- **Separation from IDEA-INFRA-006:** this idea is about **interaction**; the
  renderer (IDEA-INFRA-006) is about **visibility**. The renderer must exist
  for the manager to see what to react to. They are separate concerns.
- **What the engine must provide:**
  - A **runtime input channel** that accepts commands from the renderer
  - **Recording of inputs as events in the save** (determinism: replay reads
    the same commands at the same ticks)
  - **Notable-transition events** (hints) at the engine's discretion —
    non-blocking, no automatic pause. Suggested triggers: possession loss in
    own third, recovery in opponent third, numerical superiority in
    transition. Whether these are hardcoded or plugin-provided is open.
- **What the renderer must provide:**
  - A visible, pausable view of the match (IDEA-INFRA-006 dependency)
  - A UI for issuing commands while paused
  - Display of hints as non-blocking notifications
- **Determinism:** every input is recorded in the save as an event. Replay
  is identical. The engine does not "know" whether the match is visible; it
  receives inputs as events like any other.
- **Pre-requisites (none exist today):**
  1. Runtime input channel (new ADR)
  2. Hook system for input (new ADR — ADR-012 mentions `MatchPlugin` but the
     hook shape is open; rule plugins in ADR-019 D11 produce outcomes from
     snapshots, not commands)
  3. Event granularity fine enough to make hints meaningful (~19 events/match
     today is too coarse — see IDEA-INFRA-006)
  4. Renderer that shows more than 19 frames per match (IDEA-INFRA-006)
  5. `SIM_VERSION` bump + performance measurement, if per-tick positions are
     exposed (IDEA-INFRA-006 decision, ADR required)
- **Open questions:**
  1. How many interventions per match? (No auto-budget; the manager is free.
     But does the engine warn after N? Does it record a counter?)
  2. Format of the hint / command UI (novice vs expert — expandable per the
     2026-10-29 discussion)
  3. How is the hint trigger vocabulary defined? (Closed enum, like D2, or
     open via plugins?)
  4. Does the assistant (IDEA-STAFF-001) also emit commands through this
     channel? If yes, ordering matters (manager vs assistant priority).
- **Critical note:** this is a **sketch**, not a decision. It depends on
  four prerequisites that do not exist today. Do not implement until they
  exist and dedicated ADRs are accepted.

### IDEA-INFRA-008 — Explanation/narration as a plugin, not in the engine

- **Status:** sketch
- **Category:** Presentation / Infrastructure / Narrative
- **Source:** conversation 2026-10-31 (user suggestion), during the ADR-023 discussion
- **Idea:** the engine does not produce a textual explanation of events. It produces `MatchEvent` with `event_type`, `fields`, and `contributing_factors` (observed). A **plugin** (L3, ADR-012) reads the match — today the event stream, tomorrow the 2D positional state (ADR-019) — and generates the explanation/narrative text. The explanation is presentation (Plane C/UI), not data (Plane B), and belongs to the consumer that has the context to produce it.
- **Why this matters:** at L0 a cause trace built by the engine is tautological ("the shot is a shot"). The real value ("the player shot from outside because the defender was out of position") requires the full match context, which the engine at L0 does not have and cannot have. A plugin at L3 can. Moving the explanation to the plugin keeps `MatchEvent` minimal and durable, and eliminates the temptation to put presentational strings (like `explanation_template`) on the event.
- **Relationship with ADR-023:** ADR-023 (option 2) has the engine build an observed cause trace. IDEA-INFRA-008 is the longer-term alternative: no cause trace in the engine at all, the plugin does it. If adopted, ADR-023 becomes a stepping stone (engine produces the minimal observed factors; the plugin enriches the explanation) or is superseded (the engine produces no cause; the plugin reads `MatchEvent` and builds its own).
- **Open questions:**
  1. Does the plugin import `pcfm-model::CauseTrace` (shared type) or define its own explanation type?
  2. How does the plugin access the match state? Today: event stream (`MatchEvent`). Tomorrow: 2D positional state (ADR-019) — is that exposed to plugins, and how?
  3. Localisation: the plugin ships localised templates. How does that interact with ADR-007 (English canonical) and the future `locales/it/`?
  4. Determinism: the plugin's output must not affect the simulation (Plane C → Plane A forbidden). Is the explanation purely presentational, or can it feed back into decisions (e.g. the assistant manager reading the narrative)? If it feeds back, it becomes Plane B and must be deterministic.
  5. Performance: reading the full event stream per match is cheap at L0; at L1/L2 with seasons, it must be scoped.
- **Critical note:** this is a **sketch**, not a decision. It depends on ADR-012 (plugin contract) and ADR-019 (positional engine), neither of which is implemented. Do not implement before those exist. ADR-024 (remove CauseTrace at L0, supersedes ADR-023) is the current L0 decision and is not blocked by this sketch.

### IDEA-INFRA-009 — CauseTrace at L1 (reintroduction with content)

- **Status:** sketch
- **Category:** Infra
- **Source:** ADR-024 (2026-10-31), which removed `CauseTrace` from L0.
- **Why:** at L0 the resolver has no observable features (no distance, no pressure, no body part), so any cause trace would be empty. ADR-023 kept an empty type; ADR-024 removed it as ceremony.
- **When:** at L1, when the engine has observable features from which a cause can be *derived* (not read from a latent). Prerequisite: L1 positional model.
- **What:** a `CauseTrace` with a non-empty `contributing_factors`, each factor an observed metric with a declared source. Reintroduced via a new ADR, not by reviving the old one.
- **Anti-pattern to avoid:** do not reintroduce `CauseTrace` as a placeholder field. It comes back only when it can hold content (D3).

### IDEA-INFRA-010 — CI workflow + ruleset (phase 2)

- **Status:** sketch
- **Category:** Infra
- **Source:** session 2026-10-31 (pre-push hook discussion).
- **Why:** the `pre-push` hook is bypassable with `--no-verify`. Server-side enforcement is not. In phase 1 (ruleset disabled, no CI, single maintainer) the hook is the best available defence; in phase 2 it must be replaced by CI + ruleset.
- **When:** the commit that activates the ruleset (phase 2, `AGENT-POLICY` §1).
- **What:**
  - `.github/workflows/ci.yml`: runs C2 and `scripts/gates.sh` on every push and PR. The script is the single source of truth: the CI job calls it, it does not duplicate the logic.
  - Ruleset on `main`: branch + PR + 1 approval, required status check = the CI job. Direct push blocked server-side.
- **Prerequisite:** phase 2 start, a second reviewer (or a bot with a write token), and the ruleset activated per `AGENT-POLICY` §1.
- **Anti-pattern:** do not create an inactive workflow file now (D3). It is created when it can run.

### IDEA-STAFF-001 — Assistant manager and skip-match mode

- **Status:** sketch
- **Category:** Staff
- **Source:** conversation 2026-10-29 (user, discussed)
- **Idea:** a new kind of staff member, the **assistant manager**, with
  characteristics. When the player chooses to **skip a match**, the assistant
  manages the team for that match, making changes based on their
  characteristics. The player delegates and inherits the outcome.
- **Model B (opaque competence):**
  - **Observable (Plane B):** professional training, experience (matches
    managed, contexts).
  - **Latent (Plane A):** actual competence at in-match decisions. Lives in
    `pcfm-hidden`, never exposed.
  - **Estimated (Plane C):** reputation — a noisy signal of competence,
    correlated with but not identical to the latent. Influenced also by
    non-competence factors (visibility, team results, media sympathy).
- **Design constraints:**
  - Reputation must carry **significant noise**: the manager cannot estimate
    the latent with arbitrary precision by accumulating observations.
  - Reputation must not be a **doppelganger** of training + experience; it
    must add a partially independent signal.
  - The skip-match outcome is **uncertain by design**: the manager takes a
    risk by delegating.
- **What the assistant does (scope of skip-match):**
  - Manages the team using the pre-match tactic (set by the manager)
  - Makes in-match decisions (substitutions, transition rule changes) via the
    runtime input channel (IDEA-INFRA-007)
  - Deterministic: same characteristics + same context + same seed = same
    decisions
- **Relationship with the manager:**
  - The manager can be present and let the assistant act (delegation)
  - The manager can pause and override the assistant (IDEA-INFRA-007)
  - Priority: manager over assistant, per-tick
- **Relationship with ADR-019:**
  - If Model B, the assistant is a **new agent type** (not player, not
    referee, not peripheral-referee). It has a `Position`? No — it is not
    in-pitch. It is an **external agent** that emits commands via the input
    channel.
  - This is a type of agent that ADR-019 does not currently describe. It
    likely requires an ADR amendment or a new ADR.
- **Pre-requisites:**
  1. Plane A (ADR-003) for the latent competence
  2. Runtime input channel (IDEA-INFRA-007)
  3. New agent type or input-only agent, ADR amendment/extension
  4. `L2-staff` module (not yet written)
  5. Model B decision (if Model A: simpler, no latents, no ADR-003 impact)
- **Open questions:**
  1. How much signal vs noise in reputation? (Parameter to calibrate)
  2. Is there a hard cap on how precisely the latent can be estimated? (E.g.
     reputation update is slow, or noisy with a floor)
  3. Skip-match: is the assistant visible to the player, or is the match
     shown as if the manager were there? (UX question)
  4. Can the assistant disagree with the manager's pre-match tactic? (If yes,
     how is the disagreement communicated?)
  5. What happens to the assistant's reputation after a skip-match loss? (The
     manager should be able to blame the assistant — but can they?)
  6. How is the assistant hired? (Scouting, word of mouth, former players,
     etc.)
- **Critical note:** this is a **sketch**, not a decision. Do not implement
  until `L2-staff` exists, the runtime input channel exists (IDEA-INFRA-007),
  and the assistant agent type is decided (ADR-019 amendment or new ADR).
  Model A (transparent) is a simpler alternative if Model B turns out too
  costly.

---

### IDEA-PRES-001 — TUI-first presentation and view model
- **Status:** sketch
- **Category:** UX / Presentation
- **Source:** conversation 2026-10-28/29 (user)
- **Idea:** a terminal UI (TUI) as the primary form of presentation for PCFM,
  not as a fallback to a future GUI. The TUI reads a **presentation-level
  view model**: a stable, serializable projection of Plane C (estimation)
  and Plane B (observation), never Plane A (`pcfm-hidden`). The prototype
  runs around `pcfm-demo`, with deterministic mock data where L1/L2 does
  not yet exist.
- **Screens (prototype scope, 6 ceiling — 5 if finances is cut):**
  homepage (dashboard), squad, calendar, tactics, finances, staff
  (staff with two tabs: first team / academy, both filtered views on a
  single `StaffMember` collection with a `reparto` field).
- **Order of construction (by pressure on the view model, not by ease):**
  1. **Squad** — densest table, forces the choice of which fields exist
     and which are derived.
  2. **Tactics** — the only writable screen; introduces commands,
     validation, dirty state, undo/redo, spatial interaction.
  3. **Calendar** — introduces time as a dimension (past/future, state,
     next fixture).
  4. **Finances (or staff)** — numeric aggregation, formatting, locale.
  5. **Homepage** — **last**, because it aggregates all the others:
     building it first means inventing the data twice.
- **Tactics model (decisions from this discussion):**
  - **Three phases** (with ball, without ball, transition), each with its
    own zone assignment. Same 11 individuals across phases; positions may
    vary within role-compatible zones. Not a single tactic with views:
    three assignments on one shared zone grid.
  - **Zone + function**, not free positioning. Discrete grid (recommend
    5×3, named zones in UI, IDs in the model). The user does not move
    players: assigns players to primary zones and gives each a function
    per phase.
  - **Competence is an estimate (Plane C), not the latent (Plane A).**
    Each `(player, zone, phase)` has an estimated level + reliability +
    source. Own-team players are always "analysed" (high reliability but
    not perfect); other teams start unknown until a scout report. Colours
    and markers on the map are estimates, never facts.
  - **Heatmap:** observed (Plane B, from played matches) and estimated
    (Plane C, staff/scout). Both in the player screen, side by side, with
    a legend. Empty observed heatmap is a valid state (young player never
    fielded) and is displayed explicitly, not hidden.
  - **Hysteresis with decay:** competence rises slowly with practice,
    falls slower without it, with a non-zero floor (an ex-keeper remains
    a decent keeper). Shape of the curve is a model decision, mocked for
    the prototype.
  - **Validation:** hard errors (structural: 11 players, one zone per
    player, no double assignment) vs soft warnings (competence estimates).
    Warnings aggregated in the status bar, detailed per-zone.
- **View model rules (non-negotiable, the only binding part of this
  sketch):**
  - The UI reads a view model; it never reads `MatchState`, `Player`, or
    any `pcfm-hidden` type directly.
  - The view model projects Plane C and Plane B only. Plane A is never
    exposed.
  - The view model is serializable, so snapshot tests are deterministic.
  - The UI does not compute competence, ratings, or derived stats. If the
    UI needs a value, the view model must expose it.
  - Mocks live in a dedicated module, seeded deterministically, and are
    declared "mock" (not normative) in this sketch.
- **Responsive:** declared minimum (recommend 100×30 for the app, 120×35
  for tactics at full layout). Below the minimum, refuse with a clear
  message; do not adapt. Two or three breakpoints, each with "what appears
  here". Panels are not added at higher resolutions: they are revealed.
- **Social feed:** deferred. News of the world covers the "what happened"
  layer; social reactions require systems (fan sentiment, board pressure,
  player popularity) that do not exist yet. News = institutional facts
  derived from world state; social = reactions on those facts. Do not
  build the second without the first.
- **What this idea does not cover:** the match renderer
  (`IDEA-INFRA-006`), L3-ui (ADR-005), simulation changes
  (`SIM_VERSION` untouched), L1/L2 systems (finances, staff, youth are
  mocked).
- **Relationship with existing decisions:**
  - **`IDEA-INFRA-006`** is the match renderer inside `pcfm-demo`; this
    idea is the broader presentation layer that contains it. Neither
    replaces the other.
  - **ADR-005 chose Qt for L3-ui.** This idea proposes TUI-first as the
    primary presentation, which is in tension with that choice.
    Resolution requires re-reading ADR-005 and deciding: revise it, or
    reject this idea. **No code until ADR-005 is revisited.**
  - **ADR-019 D11** (`PositionSnapshot` on attempt events only) constrains
    the match renderer, not the rest of the presentation.
  - **If ADR-005 is revised:** a documentation pass is required on
    `README.md`, `PROJECT-STATUS.md`, `STRUCTURE.md`, `docs/ARCHITECTURE.md`,
    `specs/00-foundations.md`. Do not update them before the ADR decision.
- **Open questions:**
  1. Competence estimate: per phase, or aggregated? (Per phase is more
     informative; aggregated is simpler.)
  2. Heatmap in the tactics screen, or only in the player screen?
  3. Grid: 5×3 named zones, or different?
  4. Minimum terminal size and breakpoints: which values, chosen against
     which content?
  5. Is the prototype a throwaway (delete after validation) or a seed
     for the future UI crate?
- **Critical note:** this is a **sketch**, not a decision. The only
  binding item is the **view model separation** (UI reads a projection,
  never the simulation). Everything else — layout, screens, mocks, order
  — is up for revision when L1/L2 exist and when ADR-005 is revisited.
  Do not implement from this idea until ADR-005 is resolved.

---

### IDEA-DOC-001 — Maturity statement in README
- **Status:** sketch
- **Category:** Documentation
- **Source:** audit report 2026-10-28-09-42, finding #6 (third-pass review)
- **Idea:** add a short "current maturity" block to `README.md` and `PROJECT-STATUS.md`, separating four status dimensions that today are mixed in prose: doctrine status, implementation status, ADR status, current test status. Goal: prevent a reader (human or AI) from inferring a higher maturity than the repo actually supports.
- **Open questions:** where exactly (top of README, or a dedicated `docs/MATURITY.md`); whether it is generated or hand-maintained; how to keep it from going stale (it is exactly the failure mode it is meant to prevent).
- **Critical note:** this is a documentation-hygiene idea, not a code change. Do not implement without an ADR only if it changes the doctrine; otherwise it is a doc task to schedule in `SESSION-STATE.md`.

### IDEA-DOC-002 — ASCII/ANSI style in documentation files
- **Status:** sketch (with critical note — likely reject)
- **Category:** Documentation / presentation
- **Source:** conversation 2026-10-31 (maintainer suggestion)
- **Idea:** since the project is TUI-first (ADR-022), replace emoji and other Unicode decoration in documentation files (README badges, `SESSION-STATE` status icons, `IDEAS.md` markers) with ASCII/ANSI equivalents, so that the docs visually reflect the TUI choice.
- **Open questions:** does any consumer read the `.md` files in a terminal? Is the emoji in `SESSION-STATE` operational readability (for the maintainer) rather than presentation of the game? What is the actual scale (how many files, how many occurrences)?
- **Critical note:** this is probably over-engineering today. ADR-022 concerns the game renderer (`pcfm-demo` TUI), not the project documentation, which is read on GitHub web where emoji render correctly. Replacing operational status icons (✅🔴🟡) with text labels would be a readability downgrade, not an alignment. D3: no concrete content today requires ASCII docs. Do not implement without a concrete consumer.

### IDEA-DOC-003 — Screenshot or GIF of the TUI in the README
- **Status:** sketch
- **Category:** Documentation / presentation
- **Source:** conversation 2026-10-31 (maintainer suggestion)
- **Idea:** add to `README.md` a screenshot, animated GIF, or ASCII capture showing the TUI renderer in action, so that the TUI-first choice is *shown* rather than *declared*. More effective than rewriting 60 documentation files.
- **Open questions:** static screenshot vs GIF vs terminal cast; where to host the asset (repo binary vs release artifact); how to keep it up to date as the TUI evolves.
- **Critical note:** depends on step 12 (TUI demo, IDEA-INFRA-006). Today there is nothing to capture yet. Do not implement before the TUI exists.

---

## Rejected ideas (for traceability)

### IDEA-REJ-001 — Drones over the pitch
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** they clutter the view of the pitch; no gameplay value.

### IDEA-REJ-002 — Sensor in the ball
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** unnecessary. The moment of contact is already the event timestamp. A sensor as a separate entity would be a complication without benefit.

### IDEA-REJ-003 — Player portraits in the TUI
- **Source:** conversation 2026-10-29 (user, then discarded)
- **Why rejected:** the TUI is the primary presentation, not a fallback. Portraits (procedural or asset-based) would move attention to a channel that carries no information, and would look like "almost a photo, but grainy" — the worst of both worlds. An icon of state (`[GK]`, `[DF]`, …) is fine and different. Revisit only if the presentation changes (GUI L3-ui).

### IDEA-REJ-004 — Real player names (including "initial + surname")
- **Source:** conversation 2026-10-29 (user, then discarded)
- **Why rejected:** legal risk (image rights include initials; a surname in context identifies a person) and doctrinal inconsistency (`00-foundations.md` §5: no invented real data, no bundling commercial data). The safe path is procedural, plausible names from `world_seed` + `player_id`. Real names only via an optional user-supplied pack, declared unofficial and never committed to the repo.
