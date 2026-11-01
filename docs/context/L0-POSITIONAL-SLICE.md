# L0 positional slice — design note

**Status:** draft, written 2026-11-01. Not an ADR: the contract is ADR-019
(accepted). This note decides the *order* and the *scope* of the
implementation, not the doctrine.

**Read with:** `specs/L0-match/04-match-engine.md` §2.2, `05-match-events.md`
§2.1 and §2.6, `10-ground-truth.md` §2.2, `docs/adr/019-match-engine-contract.md`.

## 1. The gap this note closes

The L0 specification is ratified and describes a positional, agent-first
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

## 2. What the first slice is, and what it is not

**Is:** an agent has a position, a velocity, and a heading. It perceives the
ball and the nearest opponents (squared distance, no map). It takes **one**
decision per significant action (move toward the ball, or pass to the nearest
free teammate). The position is updated every tick. The match still produces
the same observable output (score, events, narrative) through the same
`MatchResult`.

**Is not:** 27 event types, attempt/outcome two-phase events,
`PositionSnapshot` in events, `ContextModifiers`, the event bus in the tick
loop, the referee as a perceptual agent. Those are later slices, listed in §5.

**Why this slice:** it proves that the engine can decide *from space* instead
of from a global probability roll. Once that is true, the rest of ADR-019 is
adding vocabulary, not changing the architecture.

## 3. Decisions

### 3.1 `SpatialState` lives in `pcfm-match`, not `pcfm-model`

The position is **match state**, not player identity. `Player` (in
`pcfm-model`) is shared with L1, where a player has a contract and a squad,
not a position on the pitch. If `position` goes into `Player`, every level
inherits it and overwrites it.

    // pcfm-match/src/spatial.rs (new)
    pub struct AgentState {
        pub position: (i32, i32),   // centimetres, integer (ADR-019 D1)
        pub velocity: (i16, i16),   // cm per tick, signed
        pub heading: u16,           // 0..=359, degrees, 0 = north
    }

    pub struct SpatialState {
        agents: BTreeMap<PlayerId, AgentState>,
        ball: (i32, i32),
    }

**Why integer coordinates:** ADR-019 D1 (determinism). Floating-point
positions would make the tick loop depend on platform FP behaviour. Integer
centimetres give a 10500 × 6800 grid, which is enough for a prototype and
exact across platforms.

**Why `BTreeMap`:** D2 bans `HashMap` in simulation paths.

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

### 3.4 One decision, one rule

When a significant action triggers (the existing `action_probability` roll
stays, for now), the agent with the ball:

1. computes the distance to the nearest free teammate (free = no opponent
   within 3 metres);
2. if a free teammate exists within 20 metres, passes (calls
   `resolve_pass`, emits `Pass` as today);
3. otherwise, moves toward the goal (updates velocity, no event).

The goalkeeper, the defenders, the pressing: not in this slice. The other 21
agents move toward their formation slot plus a small offset toward the ball
(this is "shape", not "decision"). No event is emitted for movement.

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

## 7. The three sessions, concretely

**Session A (morning, design + foundations).**
Read this note, read `04-match-engine.md` §2.2 and `10-ground-truth.md` §2.2
again. Write `spatial.rs` (`AgentState`, `SpatialState`, the 4-4-2 grid).
Tests: the grid is deterministic, the away team mirrors, the 11 starters are
placed. No change to `state.rs` yet. C2 green.

**Session B (afternoon, even partial — perception + decision).**
`perception.rs`: squared distance, 3 nearest. `state.rs`: `pick_players`
replaced by "the agent nearest the ball acts". One decision: pass to the
nearest free teammate, or move toward the goal. Tests: the agent nearest the
ball is the one that acts; the pass target is the nearest free teammate.
C2 green.

**Session C (evening — movement, bump, close).**
Movement every tick (velocity applied to position, integer). OI-22 (stream
names with `match_id`). `SIM_VERSION` 7 → 8, golden hash regenerated,
`CHANGELOG-SIM` entry. C2 green, commit, push. If the movement is not done,
the slice still closes: position + perception + decision is the slice;
movement is the first item of the next one.

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

- Do not add `PositionSnapshot` to `MatchEvent` in this slice. It is a save
  breaking change, and there is no consumer. It is slice 2.
- Do not touch `pcfm-hidden`. The perception reads `SpatialState`, not
  latents. If a decision needs a latent (finishing, passing), it goes through
  `resolve_*` as today.
- Do not introduce `f32` coordinates. Integer, per ADR-019 D1.
- Do not add `ContextModifiers`. It has no producer (L2 does not exist).
- Do not add a formation parser. 4-4-2 hardcoded, warning for the rest.

