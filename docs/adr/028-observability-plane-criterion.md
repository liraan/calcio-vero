# ADR-028 — Observability as the Plane A/B criterion

- **Status:** accepted
- **Date:** 2026-11-05
- **Supersedes:** —
- **Superseded by:** —
- **Blocks:** every field added to `pcfm-model`, `pcfm-hidden`, or any future crate; the placement of `strength` (F.7); the removal of `GK_REACH_CM` (`state.rs`, F.3-b3); the audit tracked as OI-54
- **Depends on:** ADR-003 (Plane A enforcement), `specs/00-foundations.md` §3 (Three Planes), MD-24 (amended 2026-11-05), MD-53 (added 2026-11-05)

## Context

`specs/00-foundations.md` §3 separates Plane A (Ground Truth, never exposed) from Plane B (Observation, computed from observable outcomes). It states *what* each plane contains, not *how* to classify a field that could belong to either. Two cases in the 2026-11-05 session (F.3-b3 closure, F.7 start) exposed the gap:

1. **`GK_REACH_CM` in `state.rs`.** F.3-b3 introduced `const GK_REACH_CM: i32 = 200` as the goalkeeper's reach. It is neither derived from a fact (no derivation in the comment) nor declared in `balance/` (no JSON, no `_note`). MD-24 lists three families — derived, arbitrary declared, model choices — and none of them covered it. The value was hand-written because `state.rs` cannot read `goalkeeping` (Plane A, ADR-003), so the check was approximated by a constant. The approximation was not declared as a simplification; it was a leak.

2. **`strength` in `PlayerLatents` (F.7).** `strength` (force in the legs, driving acceleration and top speed) is a physical characteristic, like `height_cm`. But `height_cm` is public in `pcfm-model::Player` and `strength` is latent in `pcfm-hidden::PlayerLatents`. If the criterion were "physical vs technical", one of the two placements would be wrong by construction. The criterion that explains both is **observability**: `height_cm` is seen (a scout sees a 190 cm centre-back), `strength` is not (it is inferred from who wins the physical duel, who accelerates faster).

The existing doctrine (`00-foundations` §3) is correct in its boundaries but silent on the classification rule. Without that rule, the next field is a coin flip.

## Decision

### 1. The criterion is observability

A field is classified by **what can be seen in a real match**, not by its physical or technical nature.

- **Plane B (observable):** the field can be seen, measured, or inferred from the observable outcomes of a match. It lives in `pcfm-model` (identity, height, name, fatigue) or is produced by `resolve_*` (velocity, event fields).
- **Plane A (latent, not observable):** the field cannot be seen directly; it is inferred from effects, and the inference is the whole point of the game. It lives in `pcfm-hidden` and is read only inside `resolve_*`.

The question that settles it, asked of every field, in plain language:

> *In a real match, can a scout see this? Can a fan see it? Can the player see it? If not, it is latent.*

`height_cm` passes: it is seen. `strength` does not: it is inferred. Both are physical; only one is observable.

### 2. A field that cannot answer the question is not designed yet

If the question has no answer, the field is not ready to be added. It goes back to `IDEAS.md` with the question as a prerequisite. This is D3 applied to fields: no field without a place in Plane A or Plane B.

### 3. The corollary: no model value in a `.rs` without a derivation or a `balance/` include

MD-24 lists three families — derived, arbitrary declared, model choices — and the third one was open ("where do its parameters live?"). This ADR closes it:

- **Derived:** the value follows from a fact of the domain, and the derivation is written in the comment above the constant. Example: `BALL_GRAVITY = 10` (cm/tick²) from g ≈ 9.8 m/s² at 10 ticks/s (`spatial.rs`).
- **Arbitrary declared:** the value is a prototype choice, lives in `balance/*.json` with `_note: prototype, not calibrated`, and the leaf that replaces it is tracked in the Plan or in an OI. Example: `balance/influence.json` (weights for `resolve_influence`).
- **Anything else:** a hand-written value in a `.rs` without one of the two above is a violation. Not a simplification: a violation.

`GK_REACH_CM` is the example of what this ADR forbids. It will be removed in F.7; the reach becomes a derivation from `height_cm` (public) and `strength` (latent, read inside `pcfm-hidden`).

### 4. The lens is applied when a file is touched, not as a big-bang audit

The classification is a judgment (D5: the criterion is not mechanizable in full). What is mechanizable is the artifact: each field has a comment above it that answers the question. The audit of the fields already in the codebase is **OI-54**, with the first action stated there. Every session that touches `pcfm-model`, `pcfm-hidden`, or any future crate pays its share. A big-bang audit is rejected (D3: structure without content).

## Consequences

- **F.7 keeps `strength` latent and `resolve_movement` in `pcfm-hidden`.** The engine receives `MovementStep` (a velocity, Plane B), not the latent. The choice was already made (D2, ADR-003); this ADR gives it the criterion it was missing.
- **`GK_REACH_CM` is removed in F.7.** The reach is derived from `height_cm` and `strength`. No model value lives in `state.rs`.
- **`PlayerLatents` has two families with different dynamics.** Physical non-observable (`strength`) and technical (`finishing`, `passing`, `tackling`, `aerial`, `goalkeeping`, `set_pieces`, `injury_proneness`). Today they are eight independent uniform `f32` from a single sequence. The role profile that makes a tall, strong centre-back more likely than a tall, strong playmaker is not modelled. Tracked as part of OI-54 (field audit) and as a future design session; not implemented here (D3).
- **MD-54 in `SESSION-STATE.md` becomes a pointer to this ADR**, not the source of the rule. The hierarchy is: `00-foundations` §3 (as amended by this ADR) > MD-54 > MD-24 (amended) > MD-53. If MD-54 ever contradicts the doctrine, the doctrine wins.
- **Every future field answers the question.** The first to use it after this ADR are the reach derivation (F.7) and the tick-record fields `PositionChange`/`HeadingChange` (T1-bis amendment, 2026-11-05): they are Plane B because the 2D/3D sees them as background, and they are not events because an event requires a cause a fan recognises (D6).

## Alternatives considered

- **Physical vs technical criterion.** Rejected: it would make `height_cm` and `strength` belong to the same class, and one of the two placements would be wrong. The 2026-11-05 session started from this wrong criterion and produced `GK_REACH_CM`, a hand-written value that violated MD-24.
- **Observable vs non-observable, but limited to new fields.** Rejected: it would leave the existing fields unclassified and create a silent exception for exactly the fields already written. The lens applies to every field; the audit is tracked (OI-54), not ignored.
- **Big-bang audit of the existing fields.** Rejected (D3). The lens is applied when a file is touched.
- **Promote the criterion to a new pillar in `00-foundations` §2.** Rejected: the pillars say *what the project is*; the Three Planes in §3 say *how it is organized*. The criterion belongs in §3, as an extension of the existing separation, not as an eleventh pillar.

## References

- `specs/00-foundations.md` §3 (Three Planes), §5 (banned things), §2.5 (radical transparency)
- ADR-003 (Plane A enforcement: `GroundTruth` is not `Serialize`, no public getters, only `resolve_*` as door)
- MD-24 (three parameter families, amended 2026-11-05 to close the third)
- MD-53 (one module, one function: `pcfm-hidden` reads latents, the narrator classifies facts)
- MD-54 (the lens as an operational rule, pointing here)
- OI-53 (keyed latents, extension of OI-44: the field classification does not fix the RNG sequence; the two are distinct)
- OI-54 (audit of the existing fields: `heading`, `fatigue`, `condition` are the ambiguous cases)
- `docs/context/analysis/2026-11-04-match-stats-and-traits.md` §3 (the report that first named the shot schema; the F.3-b3 closure is what exposed `GK_REACH_CM`)
