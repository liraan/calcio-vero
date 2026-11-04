# L0.11 — Player characteristics (the new engine)

## 0. Purpose

This cell defines **what a player is** for the new phenomenological engine:
the families of characteristics, their names, their dynamics, and where each
one enters the engine. It replaces, for the new engine, the implicit model
that the prototype encoded in `PlayerLatents` (seven uniform `f32`, one per
`resolve_*`).

This is a **model of football**, not a model of the prototype. It describes
what the engine must eventually read, not what the prototype happens to
contain today. The divergence with the prototype is declared in §7.

**Readers.** An agent starting a new session on the new engine should read
this cell before touching `Player`, `PlayerLatents`, any `resolve_*`, or any
movement/duel/save formula. The cell answers three questions: what a
characteristic is, which family it belongs to, and where it enters the
engine.

## 1. Context

The new engine emits typed facts (`Fact`), the narrator classifies them into
events, and the engine reads latents only inside `pcfm-hidden` via
`resolve_*` (ADR-003, ADR-028). A characteristic is not an event and not a
fact: it is an **input** to the engine. This cell specifies the inputs.

The prototype (`specs/L0-match/03-entities-minimal.md` §3.2) already said
that the seven latents are "the shadow of the current functions", that
`injury_proneness` is "a susceptibility, not a skill", and that "adding
fields without changing the shape gives more columns, not more realism".
This cell turns that awareness into a model.

## 2. The four families

A player is not a bag of independent numbers. Characteristics belong to four
families, each with its own dynamic, its own visibility, and its own level
of ownership.

| Family | What it is | Dynamic | Who owns it |
|---|---|---|---|
| **Physical** | The body: dimensions, force, speed, resistance, mobility, reactivity | Biological curve: growth, peak (~25–28), decline | Generated at L3, evolved at L1 (ageing), read at L0 |
| **Technical** | Learned skills: control, pass, shot, dribble, header, tackle, goalkeeping | Learning window (8–18), then maintenance or slow decline | Generated at L3 (as current state), evolved at L2 (training), read at L0 |
| **Psychological** | The head: concentration, confidence, resilience, emotional control, composure | Event-driven: collapses after an error, rebuilds with a positive run | Generated at L3 (traits), evolved at L1/L2 (events, context), read at L0 |
| **Cognitive** | Reading the game: vision, game reading, decision making | Experience-driven: grows with matches played, peaks late | Generated at L3 (as current state), evolved at L2 (experience), read at L0 |

**Why four, not two.** Physical vs technical was the prototype's implicit
split, and it fails on `aerial`: a header is a technical skill, but it
depends on height (physical) and explosive force (physical). The prototype
treats `aerial` as a pure skill and `resolve_duel` never reads `height_cm`.
Cognitive is a fourth family because in a real match the difference between
two players with the same body and the same technique is often **what they
see and how fast they decide** — and the prototype has no latent for it.

**Not a family: fatigue.** Fatigue is a *state*, not a characteristic. It
accumulates during a match and recovers between matches. It lives in
`PhysicalStateMinimal` and is governed by MD-40 (one source, two delays).
This cell does not redefine it.

## 3. The characteristics

Columns: **Persist.** (tratto = trait, stable over months; stato = state,
changes within a match), **Meas.** (M = objectively measurable, S = estimated
by observation, D = to be discovered by playing), **Gen.** (who generates),
**Reads at L0** (which `resolve_*` reads it, if any), **Observable effect**
(which event field carries its consequence, for transparency).

### 3.1 Physical

| Name | Subset | Persist. | Meas. | Gen. | Reads at L0 | Observable effect |
|---|---|---|---|---|---|---|
| Height | Dimensions | trait | M | L3 | movement (stride), header (via technique) | position, trajectory |
| Weight / mass | Dimensions | trait | M | L3 | movement (acceleration), duel (mass) | position, duel outcome |
| Explosive force | Force | trait | M | L3 | `resolve_movement`, `resolve_save`, header, shot | velocity, save distance, shot speed |
| Maximal force | Force | trait | M | L3 | duel (holding position) | duel outcome |
| Acceleration | Speed | trait | M | L3 | `resolve_movement` | velocity, position delta |
| Max speed | Speed | trait | M | L3 | `resolve_movement` | velocity |
| Aerobic resistance | Resistance | trait | M | L3 | movement over 90' (fatigue producer) | fatigue accumulation |
| Anaerobic resistance (RSA) | Resistance | trait | M | L3 | repeated sprints (fatigue recovery) | fatigue between actions |
| Joint mobility | Mobility | trait | M | L3 | movement (direction changes), save (dive) | position, dive reach |
| Reactivity | Reactivity | trait | M | L3 | `resolve_save` (reaction ticks) | save timing |

### 3.2 Technical

| Name | Subset | Persist. | Meas. | Gen. | Reads at L0 | Observable effect |
|---|---|---|---|---|---|---|
| Control / first touch | Base | trait | D | L3 | pass (receiving), shot (setup) | pass complete, shot on target |
| Passing | Base | trait | D | L3 | `resolve_pass` | pass complete |
| Shooting | Base | trait | D | L3 | `resolve_shot` | on_target, shot_type |
| Dribbling | Base | trait | D | L3 | duel (attacker side) | duel outcome |
| Heading | Role | trait | D | L3 | duel (aerial), shot (header) | duel outcome, on_target |
| Tackling | Role | trait | D | L3 | `resolve_duel` (defender side) | duel outcome |
| Goalkeeping | Role | trait | D | L3 | `resolve_save` | save outcome |

### 3.3 Psychological

| Name | Subset | Persist. | Meas. | Gen. | Reads at L0 | Observable effect |
|---|---|---|---|---|---|---|
| Concentration | State | state | D | L3 (trait threshold) | decision (degraded image, MD-40) | pass/shot outcome under pressure |
| Confidence | State | state | D | L3 (trait threshold) | decision (risk taking), shot | shot choice, on_target |
| Resilience | Stable trait | trait | D | L3 | decision (after an error) | action after a mistake |
| Emotional control | Trait + state | trait/state | D | L3 | duel (foul commit), card | foul, card |
| Composure | State | state | D | L3 (trait threshold) | `resolve_shot` (under pressure) | on_target under pressure |

### 3.4 Cognitive

| Name | Subset | Persist. | Meas. | Gen. | Reads at L0 | Observable effect |
|---|---|---|---|---|---|---|
| Vision | Perception | trait | M (eye-tracking) | L3 | perception (cone width?) | targets considered |
| Game reading | Reading | trait | D | L3 | `choose_action` (option quality) | action chosen |
| Decision making | Decision | trait | D | L3 | `choose_action` (speed/quality) | action chosen |

## 4. What enters the engine today, and when

The table below maps each characteristic to the leaf of the Plan where its
consumer becomes real. A characteristic without a leaf is either tracked as
an OI or not implemented (D3).

| Characteristic | Consumer becomes real at |
|---|---|
| Explosive force, acceleration, max speed | F.7 (`resolve_movement`) |
| Height, reactivity, goalkeeping, explosive force | F.7 (save reach and timing) |
| Passing, shooting, tackling, heading | F.3, F.4 (already read by `resolve_pass`/`resolve_shot`/`resolve_duel`) |
| Vision, game reading, decision making | F.6 (off-ball perception) and after |
| Concentration, confidence, composure, emotional control | After F.6 (needs the degraded image) |
| Resilience | L2 (events between matches) |

## 5. Visibility and the manager

**M** characteristics are measurable: the manager can know them, or a close
estimate, without playing. A 190 cm centre-back is visible; a strong leg is
visible from a test. **S** characteristics are estimated from observation
(equilibrium, coordination). **D** characteristics are the game: they are
discovered by playing, and scouting exists because of them.

**Consequence for design:** a characteristic that is `M` does not need a
scouting report; a characteristic that is `D` does. Mixing them (as the
prototype does, with seven `D` latents and `height_cm` public) hides the
distinction that makes scouting a game.

## 6. Where each characteristic lives

- **`Player` (`pcfm-model`)** — observable characteristics. Today:
  `height_cm`, `preferred_foot`. `height_cm` is here because it is seen.
- **`PlayerLatents` (`pcfm-hidden`)** — non-observable characteristics read
  by `resolve_*`. This cell does **not** decide the Rust shape (one struct
  or two, physical vs technical vs cognitive); that is a decision for the
  ADR on the player model. This cell decides **what** belongs to each family
  and **when** it enters the engine.
- **`PhysicalStateMinimal` / `PsychologicalStateMinimal`** — states, not
  characteristics. Fatigue, confidence, morale, pressure_load. They change
  within a match; characteristics do not.

**`strength` (F.7).** It is a physical characteristic, stable, objectively
measurable, non-observable in the sense of ADR-028 (you cannot see it, you
infer it from effects). Its placement in `PlayerLatents` is correct by
ADR-028 (non-observable), but its family is physical, not technical. The
distinction between "non-observable" and "technical" is the point of this
cell: they are not the same axis.

## 7. Divergence with the prototype (declared)

The prototype's `PlayerLatents` has seven `f32`, one per `resolve_*`:
`finishing`, `passing`, `tackling`, `aerial`, `goalkeeping`, `set_pieces`,
`injury_proneness`. This cell diverges in five ways:

1. **`injury_proneness` is not a technical skill.** It is a physical
   susceptibility. Same container today, different family.
2. **`aerial` is not a pure skill.** It is heading (technique) expressed
   through height and explosive force (physical). The prototype's
   `resolve_duel` never reads `height_cm`.
3. **`set_pieces` is a composite**, not a capability. Free kicks, penalties,
   corners, throw-ins are different skills; a single number hides which one
   is being used.
4. **The cognitive family does not exist** in the prototype. Vision, game
   reading, decision making are not modelled. `choose_action` is geometric.
5. **The psychological family is a state, not a characteristic.** Confidence,
   morale, pressure_load live in `PsychologicalStateMinimal` as states. The
   stable traits (resilience, emotional control) are not modelled.

The prototype is not wrong to be simple; it is wrong to be **unaware**. This
cell makes the new engine aware.

## 8. Open decisions (not resolved here)

These are the questions this cell raises and does not answer. Each one
requires a maintainer decision; they are listed so that a new session does
not restart from zero.

1. **Correlations between characteristics.** In a real match, vision and
   passing are correlated; height and explosive force are correlated;
   finishing and composure are correlated. The prototype draws them
   independently, producing impossible players (vision 0.1, passing 0.9) with
   the same probability as coherent ones. How are correlations modelled?
   (Structural equations? Role profiles? A generator at L3 that samples from
   a joint distribution?)
2. **Development window and learning speed.** Technical skills are learned
   mainly between 8 and 18; after 25 a low passing stays low. Without a
   potential scalar (CA/PA is banned, `00-foundations` §5), how is growth
   modelled? What makes two 18-year-olds with the same passing grow at
   different speeds? (A separate learning capability? Context? A hidden
   plasticity trait?)
3. **Consistency, not just the mean.** A player may have a high average
   passing and collapse on easy passes. The mean does not capture
   consistency. Is consistency a characteristic of its own, or noise on top
   of the mean?
4. **Double counting (fatigue × capability).** T3 forbids `resolve_*` from
   receiving `fatigue_milli`, to avoid penalizing the tired player twice.
   But if a characteristic is read under fatigue, how is the double count
   avoided? The degraded image (MD-40) is the perception side; the execution
   side is open.
5. **The bridge from events to estimates (Plane B → Plane C).** Scouting
   estimates `passing` from completed passes, but completed passes depend on
   teammates, opponents, position. Which statistics feed which estimate, and
   with what noise? Without this, L0 emits events and L2 cannot read them.
6. **Age, formation history, and context as inputs.** A player's
   characteristics are not generated in a vacuum: age, training history,
   level of competition shape them. These are inputs to L1/L2, not
   characteristics themselves. Where do they live?
7. **Composite characteristics.** `set_pieces` is the first; `vision` may be
   the second (peripheral vision + scanning + reading). When is a composite
   better than its parts, and when does it hide which part is acting?

## 9. References

- `specs/00-foundations.md` §3 (Three Planes), §5 (banned: no CA/PA)
- `docs/adr/003-plane-a-enforcement.md` (Plane A, the only door)
- `docs/adr/028-observability-plane-criterion.md` (observability as the A/B
  criterion; no model value in a `.rs` without a derivation or `balance/`)
- `specs/L0-match/03-entities-minimal.md` §2.1–2.3 (Player,
  PhysicalStateMinimal, PsychologicalStateMinimal) and §3.2 (the latents are
  the shadow of the prototype functions)
- `specs/L0-match/10-ground-truth.md` (the `resolve_*` contract)
- `docs/context/SESSION-STATE.md` MD-24 (amended), MD-40 (fatigue), MD-53
  (one module, one function), MD-54 (the observability lens)
- OI-53 (keyed latents, extension of OI-44), OI-54 (field audit)
