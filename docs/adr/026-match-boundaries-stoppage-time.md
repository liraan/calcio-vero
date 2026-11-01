# ADR-026 — Match boundaries and stoppage time

- **Status:** accepted
- **Date:** 2026-11-01
- **Supersedes:** —
- **Superseded by:** —
- **Blocks:** L0 match readability (the board cannot show kick-off, half-time, second-half, full-time, or stoppage time until these events exist)
- **Depends on:** ADR-019 (match engine contract), ADR-025 (board renderer)

## Context

The L0 engine emits 4 of the 27 `EventType` variants (census 2026-10-31-22-28, G11): Shot, Goal, Duel, Foul. It never emits `HalfTime` or `FullTime`, which exist in the enum. It has no `KickOff`, no `SecondHalf`, and no concept of stoppage time.

The board (ADR-025) shows what the engine emits. It cannot show a match boundary that does not exist, and it must not invent one. Today a match ends at 89'46" with no full-time marker.

## Decision

### 1. Continuous clock (decided 2026-11-01)

Timestamps do not stop at 45' and 90'. The clock is continuous:

- `KickOff` at 0'00"
- first half runs to `45 + rec1`
- `HalfTime` at `45 + rec1`
- `SecondHalf` at the same timestamp as `HalfTime`
- second half runs to `90 + rec1 + rec2`
- `FullTime` at `90 + rec1 + rec2`

`rec1` and `rec2` are the total stoppage time of the first and second half.

Why: an event at 93'12" is at 93'12", not "90' plus a field". The timestamp is the observation; stoppage time is derived from it. Because the timestamps change with `rec1`/`rec2`, `chain_hash` covers them: two matches with different stoppage times have different hashes. A separate `stoppage` field would not be covered by the hash.

**Two display contexts, both correct:**
- **During the match**, the board shows `45'+X'` and `90'+Y'`: the reader expects regulation + excess.
- **After the match**, the board shows the real total (`94'20"`): the match lasted that long.

These are two views of the same timestamp, not two models. The first subtracts the regulation boundary; the second does not.

### 2. Stoppage time: additive, per half, two passes (decided 2026-11-01)

    rec_base(half) = base + Σ (contribution[type] × count[type] in [0, 45])
    rec_base(half) = min(rec_base(half), cap_base)

    rec_extra(half) = Σ (contribution[type] × count[type] in (45, 45 + rec_base])
    rec_extra(half) = min(rec_extra(half), cap_extra)

    rec_total(half) = rec_base(half) + rec_extra(half)

Two independent computations, one per half. The second half does not inherit the first half's stoppage time.

**Why two passes, not a fixpoint.** Events inside stoppage time can extend stoppage time. A fixpoint (`simulate → count → simulate → count → …`) risks non-termination and consumes RNG differently per iteration, making calibration fragile. Two passes are exact and terminate:

1. Simulate 90', compute `rec_base`.
2. Simulate `90 + rec_base`, look only at events in `(45, 45 + rec_base]`, compute `rec_extra` (cap 2').
3. `rec_total = rec_base + rec_extra`.

Each pass restarts from the same seed, so the RNG sequence is reproducible. The event set of pass 2 contains pass 1 plus the stoppage-time events; the extra contributions are exactly those, capped.

**Caps are fixed, not dynamic.** `cap_base = 5'`, `cap_extra = 2'`, both halves. A dynamic cap (fatigue, late-game slowing) is a *simulation* property: it belongs to an engine that models fatigue. Today it does not (`action_probability` is constant). A dynamic cap now would encode a behaviour the simulation does not have. When fatigue exists, a dedicated ADR changes the cap.

**Extensibility.** Adding a contributing event type is one row in the balance table plus a `SIM_VERSION` bump. The formula does not change.

### 3. `ActorId` (decided 2026-11-01)

`MatchEvent` currently has `player_id: PlayerId`, mandatory. Match-boundary events (`KickOff`, `HalfTime`, `SecondHalf`, `FullTime`) do not belong to a player.

**Decision: `MatchEvent.actor: Option<ActorId>`**, where:

    ActorId = Player(PlayerId) | Referee(RefereeId)

- A player action: `Some(ActorId::Player(id))`.
- A referee action (future: cards, VAR): `Some(ActorId::Referee(id))`.
- A match-boundary event: `None`.

**Documented for contributors:** a new actor (VAR, fourth official, assistant referee) extends `ActorId` with a new variant. It does **not** use `RefereeId` for a non-referee actor, and it does not add a sentinel `PlayerId`. The type is the contract.

**Cost, accepted deliberately:** this is a breaking change on every event, on `chain_hash`, and on the save format. It is bundled with the `SIM_VERSION` bump this ADR already requires. Doing it now, once, is cheaper than doing it later on top of more consumers.

### 4. Two new `EventType` variants, two emitted

- `KickOff` — new. At 0'00", `actor = None`.
- `SecondHalf` — new. At `45 + rec1`, `actor = None`.
- `HalfTime` — exists, now emitted. At `45 + rec1`, `actor = None`.
- `FullTime` — exists, now emitted. At `90 + rec1 + rec2`, `actor = None`.

### 5. The balance table lives in `balance/` (decided 2026-11-01)

The contributions are balance values, not code. Doctrine §5: "No hardcoded rules, names, colours, or balance values. Everything lives in files." They live in `balance/stoppage_time.toml`.

Initial table (proposed, modify freely):

| Event type | Contribution | Applies to |
|---|---|---|
| goal | 30 s | rec_base, rec_extra |
| foul | 30 s | rec_base, rec_extra |
| duel | 10 s | rec_base, rec_extra |
| base | 0 s | rec_base only |
| cap_base | 300 s (5 min) | rec_base |
| cap_extra | 120 s (2 min) | rec_extra |

`balance/` does not exist today (census G8). This ADR creates it for this one file. D3 says a structure is added when concrete content requires it: the content is this table, the requirement is the extensibility chosen on 2026-11-01.

### 6. `SIM_VERSION` 4 → 5

Four new events, a changed match end time, and `player_id: PlayerId` → `actor: Option<ActorId>` are all simulation-visible changes. The five-step procedure in `CHANGELOG-SIM.md` applies.

## Consequences

### Easier
- The board shows kick-off, half-time, second-half, full-time, and stoppage time without inventing anything.
- `HalfTime` and `FullTime` are emitted: G11 moves from 4 to 8 event types emitted.
- The stoppage-time table is extensible without touching the formula.
- `ActorId` gives future actors (VAR, fourth official) a home that is not `RefereeId` and not a sentinel.

### Harder
- `SIM_VERSION` 5. Saves from version 4 are not compatible (per the bump procedure).
- Every consumer of `MatchEvent.player_id` changes to `MatchEvent.actor`.
- `balance/` exists for one file. The next balance value should go there, not in code.
- Two-pass simulation: pass 2 re-simulates from the same seed. Cost is roughly double, not exponential.

### Locked in
- Continuous clock. No "regulation time + separate stoppage field".
- Two-pass stoppage time, not a fixpoint.
- Fixed caps (5' base, 2' extra), not dynamic.
- `actor: Option<ActorId>`, not a sentinel and not `RefereeId` for players.

## Implementation notes

1. `pcfm-core`: define `ActorId` (`Player` | `Referee`), `Serialize`/`Deserialize`.
2. `pcfm-stats::MatchEvent`: `player_id: PlayerId` → `actor: Option<ActorId>`.
3. `pcfm-stats::EventType`: add `KickOff`, `SecondHalf`.
4. `balance/stoppage_time.toml` + a parser in `pcfm-match` (or a `pcfm-balance` crate when a second balance file appears; D3: not now).
5. `orchestrator.rs`: pass 1 simulates 90', computes `rec_base`; pass 2 simulates `90 + rec_base`, computes `rec_extra`; final simulation runs `90 + rec_base + rec_extra` and emits the boundaries.
6. `save.rs`: `SIM_VERSION` 4 → 5; the save schema carries `actor`.
7. `viewmodel.rs`: `event_kind` match gains `KickOff` and `SecondHalf` (exhaustive match forces it).
8. `board.rs`: boundaries render as `kick-off`, `half-time`, `second-half`, `full-time`; during the match `45'+X'` / `90'+Y'`; after the match the real total.
9. Specs: `specs/L0-match/05-match-events.md` §2.2, `00-overview.md` (DoD), `CHANGELOG-SIM.md`.
10. Tests: determinism (same seed, same timestamps across both passes); `rec_extra` capped at 2'; `FullTime` at `90 + rec1 + rec2`; boundary events have `actor = None`; board renders all boundaries in both display contexts.

## References

- Census G8 (`balance/` absent) and G11 (4 of 27 events): `docs/context/analysis/2026-10-31-22-28-review.md`
- ADR-019 — match engine contract
- ADR-025 — board renderer
- `specs/00-foundations.md` §4 (determinism), §5 (no hardcoded balance values)
