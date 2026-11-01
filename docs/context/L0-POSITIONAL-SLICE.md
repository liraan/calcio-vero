# L0 positional slice — design note

## 0. Vision (read this first)

**What this project is building:** a realistic simulation of a football
match. Not a video game with a renderer: a **match engine** that
understands space, physics-as-effects, and interactions between agents.
The renderer is downstream; the engine is the product.

**The paradigm: phenomenological simulation.** The engine does not
simulate physics (trajectories, contact forces, biomechanics). It models
the **observable effects** of physical conditions:
- rain does not fall: it makes perception noisy and the pitch slippery;
- wet grass does not have friction coefficients: it raises the probability
  of a slip event;
- a strong press does not compute forces: it forces an error, and the
  error is an event.

The events **emerge** from modelled conditions, tick by tick. They are not
chosen by a global probability roll.

**Why phenomenological:** it is deterministic (integer coordinates, named
RNG streams, no platform-dependent floating point), calibratable (each
model has parameters, in `balance/`), and it produces events that a fan
recognises ("he slipped because it was raining"). A physical engine would
require fixed-point physics, months of calibration, and data that does not
exist publicly. That is a different project, not a later step of this one.

**The success criterion:** the board shows events a fan recognises, and
every event carries the fields to understand *why* it happened. "He
slipped" is not enough; "he slipped, at the 67th minute, on the wet grass,
and lost the ball" is the target.

**The existing work.** The L0 specification (`specs/L0-match/`) is
ratified and describes an agent-first, positional engine
(`04-match-engine.md` §2.2). The code is a probability-per-tick prototype
that does not implement it: `pick_players` (`state.rs:228`) picks an
attacker and a defender **at random from the whole squad, with the same
roll**. `MatchEvent` has 31 variants, the engine emits 8. The gap is
declared, not hidden; this note is the plan to close it.

**Slice A is done** (commit `39c444b`): `crates/pcfm-match/src/spatial.rs`,
`AgentState` (integer position, velocity, heading), `SpatialState`
(`BTreeMap<PlayerId, AgentState>` + ball + carrier), 4-4-2 grid. 6 tests.
Nobody uses it yet: `pick_players` still runs.

**Part 1 of slice B is done** (commit `c231c98`):
`crates/pcfm-match/src/perception.rs` (squared distance, nearest
teammates/opponents, nearest to point, 4 tests) and
`SpatialState.carrier` (who has the ball, if any). Also unused by the
engine yet.

**Part 2 of slice B is next**, and its shape changed after a discussion
about realistic events: see §2 and §6-ter.


(accepted). This note decides the *order* and the *scope* of the
implementation, not the doctrine.

§2.1 and §2.6, `10-ground-truth.md` §2.2, `docs/adr/019-match-engine-contract.md`.

## 1. The gap this note closes

engine. The code is a prototype that does not implement it. This is declared,
not a discovery:

- `04-match-engine.md` §2.2: `for player in all_on_pitch() { perception →
  decision → execution }`. The code does: one global roll, then
  `pick_players` picks an attacker and a defender **at random from the whole
  squad, with the same roll** (`state.rs:228-247`).
- `05-match-events.md` §2.1: `MatchEvent` has `position_snapshot` and
  `attempt_event_id`. The code has neither.
- `05-match-events.md` §2.2: four "attempt" event types
  (`PassAttempted`, `ShotAttempted`, `DuelStarted`, `FoulCommitted`). The code
  emits zero.
- `10-ground-truth.md` §2.2: `DuelContext` has `pitch_position: (f32, f32)`.
  The code's `DuelContext` has no position.
- `09-context-modifiers.md`: `ContextModifiers` exists in the spec. The code
  has no such type.
- `02-event-bus.md`: a deterministic bus. The code has one, the engine does
  not use it.

`04-match-engine.md` carries a maintainer note: *"This module describes the
current prototype. Revision expected after ADR-019."* This note is that
revision, in its first slice.

## 2. What the slices are, and what they are not

The work is split into slices, each one observable. The slices build the
**base** of the phenomenological engine; they are not the engine.

**Slice A — spatial state. DONE (`39c444b`).**
`crates/pcfm-match/src/spatial.rs`: `AgentState` (integer position,
velocity, heading), `SpatialState` (`BTreeMap<PlayerId, AgentState>`, ball,
carrier), `from_squads` with the 4-4-2 grid. 6 tests. Unused by the
engine: `pick_players` still runs.

**Slice B part 1 — perception and carrier. DONE (`c231c98`).**
`crates/pcfm-match/src/perception.rs`: `distance_squared`,
`nearest_teammates`, `nearest_opponents`, `nearest_to_point`. 4 tests.
`SpatialState.carrier: Option<PlayerId>`. Unused by the engine.

**Slice B part 2 — who acts comes from space, and conquest is an event. NEXT.**
Two changes, one bump:
- `pick_players` (random, same roll for attacker and defender) becomes
  `pick_from_space`: the attacker is the agent of the possessing team
  nearest the ball; the defender is the nearest opponent to the attacker.
- The conquest becomes an event: when a duel is won by the defender, emit
  `Tackle` with `fields: { "from": <previous carrier slot>, "to":
  <defender slot> }`. When a pass is incomplete, `Pass` already carries
  `complete: false`; the engine also clears the carrier.
- `SpatialState` is built by `MatchState` (it does not exist outside the
  match engine).
- `SIM_VERSION` 7 → 8. The golden hash is regenerated. The save schema
  does not change.

**Why the conquest event and the space together:** a change in *who acts*
that nobody can see is not a slice, it is a refactor. The event makes the
change observable: the board shows who lost the ball and who took it. This
is the success criterion of §0 applied to the first slice.

**Slice C — the first phenomenological events.** After B part 2, the engine
can decide from space and record a conquest. Slice C adds the first
**modelled conditions**:
- a `Slip` event, driven by `pitch_wear` and `weather` (the parameters
  live in `balance/`, declared arbitrary until calibration);
- a `ControlError` event, driven by the same conditions plus pressure;
- a `Pressure` event, already in the vocabulary, now emitted.

Each event carries the fields to understand why (position, condition,
intensity). The parameters are arbitrary and declared: the model exists,
the calibration is a later step with data.

**What none of the slices is:**
- Not a physics engine. No trajectories, no contact forces, no
  biomechanics. The effects are modelled, not simulated (§0).
- Not a renderer. The board is a consumer; it is not the product.
- Not L1 or L2. No season, no ratings, no market. Those consume the events;
  they do not produce them.
- Not `PositionSnapshot` in events. That is a save-breaking change with no
  consumer yet. It returns when a view needs it (slice D, not planned).
## 3. Decisions


### 3.1 `SpatialState` lives in `pcfm-match`, not `pcfm-model`

The position is **match state**, not player identity. `Player` (in
`pcfm-model`) is shared with L1, where a player has a contract and a squad,
not a position on the pitch. If `position` goes into `Player`, every level
inherits it and overwrites it.

    // pcfm-match/src/spatial.rs (slice A, merged 39c444b)
    pub struct AgentState {
        pub position: (i32, i32),   // centimetres, integer (ADR-019 D1)
        pub velocity: (i16, i16),   // cm per tick, signed
        pub heading: u16,           // 0..=359, degrees, 0 = north
    }

    pub struct SpatialState {
        agents: BTreeMap<PlayerId, AgentState>,
        ball: (i32, i32),
        carrier: Option<PlayerId>,  // slice B: who has the ball, if any
    }

**Why integer coordinates:** ADR-019 D1 (determinism). Floating-point
positions would make the tick loop depend on platform FP behaviour. Integer
centimetres give a 10500 × 6800 grid, which is enough for a prototype and
exact across platforms.

**Why `BTreeMap`:** D2 bans `HashMap` in simulation paths.

**Why `carrier`:** the first draft of §3.4 said "the agent with the ball",
but the ball is a point, not a possession. Without a carrier the decision is
degenerate (the agent nearest the centre always acts). `carrier` is the
minimum: `None` (ball loose), `Some(id)` (the ball follows `id`). A
completed pass or a tackle changes it.

### 3.2 Initial position from the formation

`Squad.formation` exists (`4-4-2` in the fixture) and the engine never reads
it. The first slice places the 11 starters on a 4-4-2 grid:

    GK    (5%, 50%)
    LB    (25%, 15%)   CB (25%, 38%)   CB (25%, 62%)   RB (25%, 85%)
    LM    (50%, 15%)   CM (50%, 38%)   CM (50%, 62%)   RM (50%, 85%)
    ST    (75%, 38%)   ST (75%, 62%)

Coordinates as percentages of pitch length/width, converted to integer
centimetres at kickoff. The away team mirrors on the x axis. Hardcoded for
`4-4-2`; any other formation falls back to a 4-4-2 grid and logs a warning
(the formation parser is a later slice, D3: no content, no structure).

### 3.3 Perception: squared distance, no map

An agent perceives:
- the ball position (always, it is Plane B: the ball is visible);
- the distance to the 3 nearest opponents (squared, integer);
- the distance to the 3 nearest teammates (squared, integer).

It does **not** perceive the full map. This is the Plane B rule: an agent
sees what it sees, not where everyone is. The perception is a local
computation from `SpatialState`, and the result is not stored in the event
(a later slice adds `PositionSnapshot`).

Slice B implements it in `perception.rs`: `distance_squared`,
`nearest_teammates`, `nearest_opponents`, `nearest_to_point`. Four tests.

### 3.4 Who acts: the agent nearest the ball (conservative method)

**The first draft of this section said: "the agent with the ball passes to
the nearest free teammate, or moves toward the goal". That would have
removed shots, goals, duels and fouls: the demo would have shown 0-0 and
no shot events. A regression, not a slice.**

**The slice-B method is conservative.** The action split stays exactly as it
is (`action_probability` for *when*, `action_kind_roll` for *what*: pass,
shot, duel, foul). What changes is **who acts**:

    before:  pick_players = random attacker + random defender (same roll)
    after:   attacker = the agent nearest the ball
             defender = the nearest opponent to the attacker

`SpatialState.carrier` starts at `None` (ball at the centre); the first
action sets it to the attacker. A completed pass keeps it (simplification:
the receiver is not modelled); a failed pass sets it to `None`; a duel won
by the defender sets it to the defender; a foul sets it to `None`.

**Why conservative:** it proves the thesis ("the decision comes from space,
not from a global roll") with zero regression on the demo, the event types,
or `MatchResult`. The conquest event (a duel won by the defender emits
`Tackle`) is added in the same slice, because a change in *who acts* that
nobody can see is not observable. The "pass to the nearest free teammate"
rule and the movement are not in this slice: they are part of slice C,
with the first phenomenological events (§6-ter).
**What stays out:** the goalkeeper, the pressing, the formation-aware
movement, `PositionSnapshot`, attempt/outcome events. Those are §5.

**This is deliberately minimal.** It proves the loop: position → perception →
decision → movement. Everything else is vocabulary.
### 3.5 OI-22 (RNG streams without `match_id`) closes here

The streams today are `"match:actions"`, `"match:action_kind"`,
`"match:player_pick"` (`state.rs:60-64`). The spec (`01-rng.md:66-68`) wants
`match:{id}:...`. The positional slice changes the RNG consumption anyway, so
`SIM_VERSION` bumps regardless. Closing OI-22 now avoids a second bump.

**Action:** `MatchState::new` takes `match_id` and builds
`format!("match:{}:actions", id.slot())`. Same for the other streams.

### 3.6 OI-24 (getters on `GroundTruth`) — deferred, with a recommendation

`world_seed()` and `match_id()` are public getters. ADR-003 says "no public
getters on latents". These are not latents (they are identifiers), but the
contract is strict. Recommendation: **amend ADR-003** to declare them
non-latent, rather than remove them (they are useful for the save
reconstruction test). This is a decision for the maintainer, not for the
slice. **Not blocking.**

### 3.7 `duration_minutes` duplication — not in this slice

`MatchRules.duration_minutes` and `MatchConfig.duration_minutes` both exist.
The engine uses `MatchConfig`. The duplication is real but harmless today.
Recorded here so it is not lost; fixed when a config file
(`balance/match_engine.toml`) exists.

## 4. Files the slice touches

    crates/pcfm-match/src/spatial.rs      NEW — AgentState, SpatialState
    crates/pcfm-match/src/perception.rs   NEW — what an agent sees
    crates/pcfm-match/src/state.rs        SpatialState field, pick from space
    crates/pcfm-match/src/orchestrator.rs  no change (the tick loop stays)
    crates/pcfm-match/src/context.rs       no change
    crates/pcfm-match/src/lib.rs           export the new modules
    crates/pcfm-match/tests/golden_l0.rs   hash regenerated (SIM_VERSION 8)
    CHANGELOG-SIM.md                       SIM_VERSION 7 -> 8

Not touched: `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-demo`.
The board keeps working: `MatchResult` does not change.

## 5. What is out, and when it comes back

| Piece | Why out | When |
|---|---|---|
| `PositionSnapshot` in events | Breaking change on the save; no consumer yet | Slice 2, when the board or a debug view needs it |
| Attempt/outcome two-phase events | Requires the rules plugin contract (ADR-019 D11) | Slice 3 |
| `ContextModifiers` | Requires L2 producers | With L2-psyche |
| Event bus in the tick loop | The engine emits directly today; the bus is for subscribers | When a second subscriber exists |
| Referee as perceptual agent | ADR-021; independent of space | Separate slice |
| Formation parser | D3: no content, no structure. 4-4-2 hardcoded | When L2-tactics exists |
| `MatchResult` changes | Not needed for the slice | Never, if the slice works |

## 6. Open points, with a hypothesis

1. **Does the ball have a position?** Yes: `SpatialState.ball`. It moves when
   a pass completes (from passer to receiver), stays with the carrier
   otherwise. Hypothesis: a simple interpolation over 3 ticks, integer.
   If it is too much for slice 1, the ball can teleport; the slice still
   proves the decision loop.
2. **What is "significant action" after the slice?** Today it is
   `action_probability` (0.0005 per tick). After the slice, the same roll
   selects **which agent** acts (the one nearest the ball), not **whether**
   an action happens. Hypothesis: keep the probability for now; the space
   decides *who*, the probability decides *when*.
3. **How is possession represented?** Today `Possession::Home | Away`. After
   the slice, the ball position plus the nearest agent gives the carrier.
   Hypothesis: keep `Possession` for the event stream (compatibility), add
   `carrier: Option<PlayerId>` in `SpatialState`.
4. **Does the slice change the score distribution?** Almost certainly yes:
   the decision is now spatial, and the number of passes/shots will differ.
   This is why `SIM_VERSION` bumps and the golden hash is regenerated. The
   new distribution is not calibrated; it is the first spatial one.

## 6-bis. State at 2026-11-01, end of session

**Slice A is done** (commit `39c444b`). What exists now:

- `crates/pcfm-match/src/spatial.rs` (204 lines): `AgentState`
  (`position: (i32, i32)` cm, `velocity: (i16, i16)`, `heading: u16`),
  `SpatialState` (`BTreeMap<PlayerId, AgentState>` + `ball: (i32, i32)`),
  `from_squads(pitch, home, away)` with the 4-4-2 grid, away mirrored, ball
  at the centre.
- 6 tests, all green: 22 agents, determinism (same input → same output),
  ball at the centre, goalkeepers at the extremes, every position inside
  the pitch, away mirrored on x.
- Exported from `pcfm-match`: `pub use spatial::{AgentState, SpatialState};`
- 129 tests total (was 123). Badge and the four docs updated. A4 green.

**What has NOT changed** (important for the next agent):

- `state.rs` still has `pick_players` (random pick, no space).
- `SpatialState` exists but **nobody uses it yet**. The engine does not
  build it, does not read it. It is dead code until slice B.
- `SIM_VERSION` is still 7. The golden hash is unchanged.
- `MatchEvent` has no `position_snapshot`. `MatchResult` is unchanged.
- The board still works: `cargo run -p pcfm-demo -- --seed 42 --board`.

**The next concrete step (slice B part 2):** in `state.rs`,
`MatchState` builds (or receives) the `SpatialState`; `pick_players` is
replaced by `pick_from_space`: the attacker is the agent of the
possessing team nearest the ball, the defender the nearest opponent to
the attacker. The conquest becomes an event: a duel won by the defender
emits `Tackle` with `fields: { "from": ..., "to": ... }`; an incomplete
pass clears the carrier. `SIM_VERSION` 7 → 8, golden hash regenerated.
See §2 and §7.
**Reading order for the next agent, before writing any code:**

1. `docs/context/SESSION-STATE.md` (the bookmark: where we are)
2. this note, §1 (the gap), §3 (the decisions), §6-bis (what exists)
3. `crates/pcfm-match/src/spatial.rs` (what slice A produced)
4. `crates/pcfm-match/src/state.rs`, `pick_players` at `:228` (what slice B
   replaces)
5. `specs/L0-match/04-match-engine.md` §2.2 (the target tick loop)
## 6-ter. The phenomenological event vocabulary (the map)

The slices above produce **structure** (space, perception, conquest). This
section is the **map of the events** the engine should eventually produce,
so the next slices have a target. It is a map, not a commitment: each event
is implemented when its model exists, not before (D3).

### The principle

An event is worth adding when:
1. a fan recognises it ("he slipped", "he lost control", "he was pressed");
2. it has a **cause** the engine can model (a condition, a decision, an
   interaction), not a random draw;
3. it carries the **fields** to understand why (position, condition,
   intensity, outcome);
4. it has a **consumer** today (the board, the narrative) or a declared
   one in the next slice.

An event that fails any of the four is an idea, not a design (D3).

### The candidates, with their fields

| Event | Cause (modelled) | Fields | Consumer |
|---|---|---|---|
| `Tackle` | defender wins a duel | from, to, position | board, narrative |
| `Interception` | defender anticipates a pass | from, to, position | board, narrative |
| `Pass` (already) | decision | complete, from, to | board, narrative |
| `Slip` | `pitch_wear` × `weather` × velocity | position, condition, lost_ball | board, narrative |
| `ControlError` | `pitch_wear` × pressure × skill (via `resolve_*`) | position, pressure, lost_ball | board, narrative |
| `Pressure` (in enum) | opponent within R, intent | position, intensity | board, narrative |
| `Push` | contact, intensity, body part | position, intensity, outcome | board (needs data: deferred) |

`Push` is listed but **deferred**: "which part of the body" has no public
dataset, and inventing it violates `00-foundations` §5. It returns when a
calibration source exists.

### The parameters

Every model has parameters (how much `pitch_wear` raises the slip
probability, how much rain adds to perception noise). They live in
`balance/`, they are **arbitrary and declared** until calibration. The
`CHANGELOG-SIM` entry records them as "prototype, not calibrated".

The calibration itself is a later step, with data. It does not block the
model: a model with arbitrary parameters is still a model, and the board
shows the events it produces.

### What is not in the map

- Physics (trajectories, forces, biomechanics): §0.
- "Which part of the body": no data, deferred.
- Events with no cause the engine can model: a random draw is not a cause.
- Events with no consumer: an idea, not a design.

## 7. Where we are, and the next sessions

**Done:**
- Slice A (`39c444b`): `spatial.rs`, `AgentState`, `SpatialState`, 4-4-2
  grid, 6 tests.
- Slice B part 1 (`c231c98`): `perception.rs` (4 tests),
  `SpatialState.carrier`.
- 133 tests, C2 green, CI green on `main`, gates green.

**The next session (slice B part 2).** Two changes, one `SIM_VERSION` bump:
1. `MatchState` owns a `SpatialState` (built from `ctx.match_` in
   `MatchState::new`, or in `simulate_match` and passed in — the second is
   simpler, decide in the session).
2. `pick_players` becomes `pick_from_space`: attacker = the agent of the
   possessing team nearest the ball; defender = the nearest opponent to
   the attacker.
3. The conquest is an event: a duel won by the defender emits `Tackle`
   with `fields: { "from": ..., "to": ... }`; an incomplete pass clears
   the carrier (`Pass` already carries `complete: false`).
4. `SIM_VERSION` 7 → 8, golden hash regenerated, `CHANGELOG-SIM`.

Tests: the attacker is the nearest to the ball; the defender is the
nearest opponent to the attacker; a won duel emits `Tackle` with both
slots; determinism (same seed → same hash).

**The session after (slice C — first phenomenological events).** `Slip`
and `ControlError`, driven by `pitch_wear` and `weather` from
`balance/` (parameters arbitrary, declared). `Pressure` emitted. See
§6-ter for the map.

**Open, not for the next session:**
- OI-22 (stream names with `match_id`) — closes in slice B part 2, because
  the RNG consumption changes anyway.
- OI-24 (getters on `GroundTruth`) — a maintainer decision (remove or
  amend ADR-003). Not blocking.
- `PositionSnapshot` in events — slice D, not planned; needs a consumer.
- `ContextModifiers` — needs L2 producers.
## 8. How to verify the slice worked

Not "the match looks better". Concrete:
- `pick_players` no longer exists; `grep -rn "pick_players" crates/` is empty.
- The agent that acts is the one nearest the ball: a test with two fixed
  positions asserts it.
- The 4-4-2 grid is deterministic: two `SpatialState::from_squad` calls with
  the same squad produce the same map.
- `SIM_VERSION` is 8, the golden test passes with the new hash.
- The board still renders: `cargo run -p pcfm-demo -- --seed 42 --board`.

## 9. What the next agent must not do

- Do not add `PositionSnapshot` to `MatchEvent`. It is a save-breaking
  change, and there is no consumer. It returns when a view needs it.
- Do not touch `pcfm-hidden`. Perception reads `SpatialState`, not latents.
  If a decision needs a latent (finishing, passing), it goes through
  `resolve_*` as today.
- Do not introduce `f32` coordinates. Integer, per ADR-019 D1.
- Do not add `ContextModifiers`. It has no producer (L2 does not exist).
- Do not add a formation parser. 4-4-2 hardcoded, warning for the rest.
- Do not add an event that has no consumer and no cause the engine can
  model. §6-ter lists the candidates; an event outside the map is an idea,
  not a design.
- Do not calibrate. The parameters in `balance/` are arbitrary and
  declared. Calibration is a later step with data.
