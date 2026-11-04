# L0.11 — Player characteristics (the new engine)

## 0. Purpose

This cell defines **what a player is** for the new phenomenological engine:
the families of characteristics, the names, the dynamics, where each one
enters the engine, and how the engine reads it. It replaces, for the new
engine, the implicit model the prototype encoded in `PlayerLatents` (seven
uniform `f32`, one per `resolve_*`).

This is a **model of football**, not a model of the prototype. It describes
what the engine must eventually read, not what the prototype happens to
contain today. The divergence with the prototype is declared in §10.

**Readers.** An agent starting a new session on the new engine reads this
cell before touching `Player`, `PlayerLatents`, any `resolve_*`, any
`balance/*.json` formula, or any movement/duel/save code. The cell answers
four questions: what a characteristic is, which family it belongs to, where
it lives, and how the engine reads it.

## 1. Context

The new engine emits typed facts (`Fact`), the narrator classifies them
into events, and the engine reads latents only inside `pcfm-hidden` via
`resolve_*` (ADR-003, ADR-028). A characteristic is not an event and not a
fact: it is an **input** to the engine. This cell specifies the inputs.

The prototype (`03-entities-minimal.md` §3.2) already said that the seven
latents are "the shadow of the current functions", that
`injury_proneness` is "a susceptibility, not a skill", and that "adding
fields without changing the shape gives more columns, not more realism".
This cell turns that awareness into a model.

**Design principle (MD-23).** The model is chosen for fidelity to a real
match; performance is a measured constraint, not the reason for a
decision. A characteristic that a fan recognises and the engine can
explain is kept even if it costs; the cost is measured (OI-51 has the
threshold and the method), and if it is too high the simplification is
declared and tracked, not used as a silent veto.

## 2. The one criterion: what is a characteristic

This section gives the single criterion that decides, for every datum, what
it is and where it lives. It is stated first, because every later table
follows from it. It assumes no knowledge of the project: every term is
defined the first time it appears.

### 2.1 The criterion, in one sentence

> **A characteristic is a hidden datum that is stable over months and is a
> performance, not a dimension. Everything else is a dimension, a state, a
> behaviour, a composite, or out of scope — and none of those is a
> characteristic.**

The criterion has two halves, and both must hold:

1. **Hidden.** The value is not visible without an instrument: you cannot
   read it by watching, you infer it from effects (a dynamometer for
   strength, many matches for passing). Visible-and-measured data is a
   *dimension*, not a characteristic.
2. **Stable and a performance.** The value changes over months or years
   (age, training, experience), never within a match. A value that changes
   within a match is a *state*. A dimension (height, weight) is a property
   of the body, not a performance: it does not change with effort and it
   is not something the player *does*.

### 2.2 The five kinds of datum

| Kind | What it is | Changes | Example | Lives in |
|---|---|---|---|---|
| **Characteristic** | Hidden, stable, a performance | Over months or years | passing, strength, vision | `pcfm-hidden`, read through the custodian |
| **Dimension** | Visible, measured, a property of the body | Over years (growth), not with effort | height, weight, preferred foot | `Player` (`pcfm-model`), read directly |
| **State** | A condition of the moment | Within a match, or between matches | fatigue, confidence, morale | `PhysicalStateMinimal`, `PsychologicalStateMinimal` |
| **Behaviour** | Something the player *does* | Every action or tick | scanning, positioning, pressing | The decision code, never a saved datum |
| **Composite** | A name that hides several things | — | set pieces (free kicks, penalties, corners) | Not a datum: it is split or declared a label |

**A composite is not a sixth kind of datum: it is a warning.** `set pieces`
looks like one capability but hides four different skills. `instinct` /
`anticipation` is reading + experience + concentration. A composite is
either split into its parts or declared a label that describes an effect,
never a stored value.

### 2.3 Why it matters

A characteristic is saved once and evolves slowly; a state is saved or
reconstructed every match; a behaviour is never saved; a dimension is
measured once. If the kinds are mixed, three things break: the save does
not know what to keep, the model does not know what is a trait and what is
a condition, and the manager is shown a value he should have discovered.

**Example (the boundary that matters).** Scanning — turning the head
before receiving — is a **behaviour**, not a characteristic: it is what
the player does with his vision. Vision is the characteristic; scanning is
the behaviour that uses it. Putting scanning in the characteristic list
would be a category error: it is not something the player *is*, it is
something he *does*.

**Example (the boundary that used to be fuzzy).** Fatigue and confidence
are both conditions of the moment, so both are **states**, not
characteristics. But they differ in how the manager knows them: fatigue is
seen directly (the player slows, the engine holds a counter), confidence
is not — it is inferred from behaviour. The next section explains why this
difference does not change the kind of datum: it changes the *estimate*.

### 2.4 The manager never sees a true value

This is the principle that makes the game a game.

The manager — the human player of Calcio Vero — **never reads a true
value**. He reads an **estimate**, always, for every datum: a measurement
for a dimension (height with a ruler, weight with a scale), a test for a
physical performance (a dynamometer for strength), many matches for a
technical skill (passing). The estimate has a confidence, and the
confidence depends on the method, not on the datum.

This is why scouting exists. If the manager saw `passing = 0.73`, there
would be nothing to discover and the game would lose its core. He sees
"this player passes well, but I have seen him only five times, and twice
against weak opponents": an estimate with low confidence.

**Consequence for the model.** There is no "the manager reads Plane B
directly". Plane B is where the *engine* reads a value directly (a
dimension, a state it holds). The manager reads an estimate in Plane C,
for every kind of datum, with a confidence that depends on the method.
The distinction between "observable" and "estimable" is not a difference
between datums: it is a difference between *who reads*, and with what
confidence.

**The three methods of estimation** (a lens for analysis, not a
classification of datums; it becomes concrete only when the scouting crate
exists):

| Method | What it means | Example |
|---|---|---|
| **Instrumental** | A test produces a number, low error, repeatable | dynamometer for strength, sprint for acceleration, eye-tracking for peripheral vision |
| **Statistical-contextual** | Match statistics plus a model, high error, depends on teammates and opponents | completed passes for passing, shots for finishing |
| **Qualitative** | Expert judgement only, no reliable number | composure under pressure, reading of the game |

The same characteristic can have more than one method (strength is
instrumental *and* observable in duels); the methods describe how the
estimate is formed, not where the datum lives.

## 3. The four families

A player is not a bag of independent numbers. Characteristics belong to
four families, each with its own dynamic, its own visibility, and its own
level of ownership.

| Family | What it is | Dynamic | Who owns it |
|---|---|---|---|
| **Physical** | The body: dimensions, force, speed, resistance, mobility, reactivity | Biological curve: growth, peak (~25–28), decline | Generated at L3, evolved at L1 (ageing), read at L0 |
| **Technical** | Learned skills: control, pass, shot, dribble, header, tackle, goalkeeping | Learning window (8–18), then maintenance or slow decline | Generated at L3 (as current state), evolved at L2 (training), read at L0 |
| **Psychological** | The head: concentration, confidence, resilience, emotional control, composure | Event-driven: collapses after an error, rebuilds with a positive run | Generated at L3 (traits), evolved at L1/L2 (events, context), read at L0 |
| **Cognitive** | Reading the game: vision, game reading, decision making | Experience-driven: grows with matches played, peaks late | Generated at L3 (as current state), evolved at L2 (experience), read at L0 |

**Why four, not two.** Physical vs technical was the prototype's implicit
split, and it fails on `aerial`: a header is a technical skill, but it
depends on height (physical) and explosive force (physical). The prototype
treats `aerial` as a pure skill and `resolve_duel` never reads
`height_cm`. Cognitive is a fourth family because in a real match the
difference between two players with the same body and the same technique
is often **what they see and how fast they decide** — and the prototype
has no latent for it.

**Not a family: fatigue.** Fatigue is a *state*, not a characteristic.
It accumulates during a match and recovers between matches. It lives in
`PhysicalStateMinimal` and is governed by MD-40 (one source, two delays).

## 4. Three components: custodian, calculator, orchestrator

This section explains how a player's characteristics become an outcome on
the pitch. It assumes no knowledge of the rest of the project: every term
is defined the first time it appears.

### 4.1 The problem, in plain words

A player has characteristics that the manager cannot see directly:
passing, finishing, strength, vision, reading of the game. They are stored
hidden. When an action happens — a shot, a pass, a duel, a movement — the
engine must combine those hidden characteristics with the situation (where
the player is, how tired he is, who is near him) and produce an outcome
(on target or not, complete or not, won or lost).

The question this section answers is: **who does what, and who is allowed
to see what?**

### 4.2 Three components, three responsibilities

| Component | Where it lives | What it holds | What it does | Sees the hidden values |
|---|---|---|---|---|
| **Custodian** | `pcfm-hidden` | The hidden characteristics | Answers "what is the value of this characteristic for this player?" | yes |
| **Calculator** | a new crate | The formulas from `balance/*.json` | Applies a formula to a situation and a player, returns the outcome | yes, in input |
| **Orchestrator** | `resolve_*` | Which formula, which situation, when | Calls the calculator, returns the outcome to the engine | no |

The **custodian** is the only place where the true values live. It does not
calculate, does not decide, does not expose. It answers one question at a
time.

The **calculator** is the only place where the formulas are applied. It
receives the values from the custodian and the situation from the
orchestrator, applies the formula, returns the outcome. It does not know
who asked or why.

The **orchestrator** is the door the engine uses. It decides when to call
the calculator, with which situation, for which formula. It does not
calculate.

The **engine** (`pcfm-match`) calls `resolve_*` and nothing else. It never
sees a hidden value, never sees a formula.

### 4.3 The three APIs, and who may call them

Each component exposes one API, and only one component may call it. The
visibility is a compile-time boundary, not a convention: if the engine
tried to call the custodian, it would not compile.

| API | Callable by | Returns |
|---|---|---|
| Custodian: `value(player, characteristic)` | only the calculator | the true value of that characteristic for that player |
| Calculator: `apply(formula, situation, player)` | only the orchestrator | the outcome (observable, not hidden) |
| Orchestrator: `resolve_*` | the engine | the outcome (observable, not hidden) |

The custodian has **no** "give me all the values" method. The caller must
know what to ask for, one characteristic at a time. The engine does not
know the custodian or the calculator exist; it knows only `resolve_*`.

**Why the boundary matters.** The hidden values are the secret the manager
must discover by watching matches. If the engine could read them directly,
the secret would leak into every part of the game. The three-API boundary
makes the leak impossible at compile time, not by promise.

### 4.4 The calculator does not choose

The calculator has no freedom of choice. The **formula** declares which
characteristics and which situation data it uses, and the calculator asks
the custodian for exactly those, no more:

1. The orchestrator chooses **which formula** (from the action: shot, pass,
   duel, movement).
2. The formula in `balance/*.json` **declares its terms**: which
   characteristic, which situation datum, with which weight.
3. The calculator reads the formula, and for every term that is a
   characteristic, asks the custodian for **that one**, for that player.
4. The calculator combines the values with the situation data and returns
   the outcome.

**"Which characteristics does the shot use" is a maintainer decision,
written in the formula.** It is not the calculator's decision (it
executes), not the orchestrator's (it chooses the formula, not its terms),
not the engine's (it sees nothing). It is data, and changing it is a
declared version bump.

**Mechanical verification.** A mock custodian records every call; the test
verifies that the calculator asked for exactly the formula's terms, no
more and no less. If it asks for something not declared, the test fails.
The rule "the calculator reads more than it needs" becomes a red test, not
a promise.

### 4.5 The shape of a formula

A formula is a weighted sum of known terms, not a free expression language.
The terms are enumerated; the weights live in `balance/`; the arithmetic
is integer or fixed-point, never floating point on the simulation path.

The shot formula, illustrative (the weights are prototype values, not
calibrated):

    outcome_score = w_finishing * finishing
                  + w_composure * composure
                  + w_distance  * distance_term(situation)
                  + w_pressure  * pressure_term(situation)
                  + w_fatigue   * fatigue_term(situation)
                  - w_opponent  * opponent_term(situation)

The terms fall into two kinds: **characteristics** (read from the
custodian, hidden) and **situation data** (position, distance, pressure,
fatigue — visible to anyone watching). The formula combines the two; it
never reads a hidden value directly, only through the custodian's API.

**Validation at load time, not at run time.** The calculator validates a
formula against the catalogue of available terms — the custodian's
characteristics plus the fields of that situation — and rejects a formula
with unknown terms before applying it. A formula that names a
characteristic the custodian does not have, or a situation field the
situation does not carry, fails at load time with a clear error, not in
the middle of a match.

### 4.6 Why three components, not one

With a single component (the prototype's approach), adding a formula
touches the function, adding a characteristic touches the function,
changing the orchestration touches the function: three reasons to change,
all in the same place.

With three components:

- **Adding a characteristic** is a value in the custodian and a term in the
  formulas that use it. No function changes.
- **Adding a formula** is a new file in `balance/`. Neither the orchestrator
  nor the custodian changes.
- **Changing the orchestration** (when, in which situation) touches only
  the orchestrator. The formulas and the custodian do not change.

Three components, three reasons to change, separated.

### 4.7 What it forbids

- **A free expression language.** The terms are enumerated; a formula
  cannot invent a new operator.
- **Floating point on the simulation path.** Fixed-point or integers,
  deterministic.
- **The engine reading the custodian or the calculator.** It calls
  `resolve_*` and nothing else.
- **A "give me all the values" door.** The custodian answers one
  characteristic per call.

### 4.8 Prerequisite: hidden values must be independent of generation order

This structure is sustainable only if adding a characteristic does not
change the hidden values of the other players.

Today the hidden values are generated in sequence: the first player
generated consumes the first random draws, the second player consumes the
next ones, and so on. Adding a characteristic consumes one more draw for
every player, so every player generated after the first ends up with
different values. Every match changes.

The fix, tracked as OI-53, is to derive each value from a key instead of a
sequence: `hash(world_seed, match_id, player_id, characteristic)`. A value
then depends on the player and the characteristic, not on how many players
were generated before him. This is the same defect already found for event
noise (OI-44), and the fix is the same.

**This is a prerequisite, not an option.** Until OI-53 is closed, every
added characteristic shifts the whole match and is a version bump that
changes every result. With OI-53 closed, adding a characteristic is a
local change: the other players are untouched.

## 5. Contexts, one per action family

The context is not one per characteristic. It is one per **action family**:
the data a family of actions needs, regardless of how many characteristics
the formula will read.

| Context | Action family | Variable data carried |
|---|---|---|
| `ShotContext` | Shooting | shooter, from, to, difficulty (pressure), fatigue at the moment |
| `PassContext` | Passing | passer, from, to, fatigue at the moment |
| `DuelContext` | Contesting | attacker, defender, duel type, positions |
| `SaveContext` | Goalkeeping | goalkeeper, crossing point, height_cm, time available |
| `MovementContext` | Moving | agents, targets, positions, current velocities |
| `InfluenceContext` | Influence (already exists) | point, agents, skill, fatigue |

**A characteristic enters a context when a formula uses it**, not when the
characteristic exists. `composure` enters `ShotContext` because the shot
formula reads it; it does not enter `DuelContext` because the duel formula
does not.

**The context carries observable data, not latents.** The calculator
reads the hidden value through the custodian API (one characteristic
per call); the context carries position, distance, pressure, fatigue —
everything a scout in the stand could see.

## 6. What enters the engine, and when (map of fidelity)

This is a **map of when a phenomenon becomes observable in the engine**,
not a filter to save work. A characteristic that a fan recognises and a
formula can read is in the model; the leaf where its consumer becomes
real is declared here so a future session does not restart from zero.

| Characteristic | Formula / consumer becomes real at |
|---|---|
| Explosive force, acceleration, max speed | F.7 (`balance/movement.json`, `resolve_movement`) |
| Height, reactivity, goalkeeping, explosive force | F.7 (save reach and timing) |
| Passing, shooting, tackling, heading | F.3, F.4 (already read by `resolve_pass`/`resolve_shot`/`resolve_duel`) |
| Vision, game reading, decision making | F.6 (off-ball perception) and after |
| Concentration, confidence, composure, emotional control | After F.6 (needs the degraded image) |
| Resilience | L2 (events between matches) |
| Set pieces | OI-48 (the rules module) |

## 7. The characteristics, by plane

Every row follows the criterion of §2: a characteristic is hidden, stable,
and a performance. It lives in **one plane, and one only**. The column
**Plane** says which: **A** (hidden, read by the engine through the
custodian) or **B** (visible, read by the engine directly). There is no
"A+B": a datum that looks like it belongs to two planes is not one datum,
it is a characteristic plus a state (see §7.6).

Columns: **Plane** (A or B), **Persist.** (trait = stable over months;
state = changes within a match), **Est.** (how the manager estimates it:
**I** instrumental test, **S** statistical-contextual, **Q** qualitative
judgement — a lens for analysis, not a classification, §2.4),
**Consumed by** (which formula or decision, or "none yet"), **Observable
effect** (which event field carries the consequence).

### 7.1 Physical (Plane A)

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Explosive force | A | trait | I | movement, save, header, shot | velocity, save distance, shot speed |
| Maximal force | A | trait | I | duel (holding position) | duel outcome |
| Acceleration | A | trait | I | movement | velocity, position delta |
| Max speed | A | trait | I | movement | velocity |
| Resistance (aerobic + anaerobic) | A | trait | I | movement over 90' | fatigue accumulation and recovery |
| Joint mobility | A | trait | I | movement (direction changes), save (dive) | position, dive reach |
| Reactivity | A | trait | I | save (reaction ticks) | save timing |
| Control of the body (equilibrium + coordination) | A | trait | Q | duel, shot under pressure | duel outcome, on-target |

**Height and weight are not here.** They are dimensions, not
performances: properties of the body that exist whether the player moves
or not. They live in `Player` (Plane B), they are measured, and they enter
the formulas as inputs. See §7.5.

**Declared merges:**

- **Aerobic + anaerobic resistance -> Resistance.** In a real match the
  phenomenon is one ("who collapses", "who recovers"). The engine reads it
  in two moments (duration and recovery), but it is one characteristic.
- **Equilibrium + coordination -> Control of the body.** Seen together in
  a real match ("he moves well"), used in the same place by the engine
  (duel, gesture under pressure).
- **Acceleration and max speed stay two parameters** of the same model,
  not two independent characteristics: distinct in a real match (short
  burst vs long run) but correlated by nature.

### 7.2 Technical (Plane A)

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Control / first touch | A | trait | Q | pass (receiving), shot (setup) | pass complete, shot on target |
| Passing | A | trait | S | pass formula | pass complete |
| Shooting | A | trait | S | shot formula | on_target, shot_type |
| Dribbling | A | trait | S | duel (attacker side) | duel outcome |
| Heading | A | trait | S | duel (aerial), shot (header) | duel outcome, on_target |
| Tackling | A | trait | S | duel formula (defender side) | duel outcome |
| Goalkeeping | A | trait | S | save formula | save outcome |

**Heading is not aerial.** Heading is the technique; it is *expressed
through* height and explosive force. The prototype's `aerial` collapsed
the three into one number and never read `height_cm`. In the new model,
`height_cm` (Plane B) and `explosive force` (Plane A) are terms of the
same formula, together with `heading` (Plane A).

**Set pieces is not a characteristic** (it is a composite, §2.2): free
kicks, penalties, corners, throw-ins are different skills. It waits for
the rules module (OI-48) and will be split, not stored as one number.

### 7.3 Psychological traits (Plane A)

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Resilience | A | trait | Q | none yet (L2) | action after a mistake |
| Emotional control | A | trait | Q | duel (foul commit), card | foul, card |
| Aggressiveness | A | trait | Q | duel (commitment) | duel intensity |
| Courage | A | trait | Q | duel (going for the ball) | duel outcome |

These are **traits**: stable thresholds, hidden. They are not the state of
the moment: that is a separate datum, in Plane B, computed from the trait
plus fatigue plus events (§7.6).

### 7.4 Cognitive (Plane A)

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Vision | A | trait | I | perception (cone width) | targets considered |
| Game reading | A | trait | Q | choice of action (option quality) | action chosen |
| Decision making | A | trait | Q | choice of action (speed/quality) | action chosen |
| Risk management | A | trait | Q | choice of action (risk) | action chosen |

**The cognitive family does not exist in the prototype.** Today the
choice of action is geometric: it reads positions and picks. With this
family, the choice is no longer pure geometry. The leaf is F.6.

**Vision is the exception in this family for the estimate:** a test
(eye-tracking) measures it, so the manager estimates it instrumentally,
not qualitatively. The datum is still hidden (the engine reads it through
the custodian); the estimate is just better than for reading or decision
making.

### 7.5 Dimensions (Plane B, in `Player`)

These are **not characteristics**: they are properties of the body, they
are measured, and they are visible enough that the manager does not need
to discover them (the estimate is a measurement).

| Name | Plane | Persist. | Est. | Consumed by | Observable effect |
|---|---|---|---|---|---|
| Height | B | property | measurement | movement (stride), header (via technique), save (reach) | position, trajectory, save distance |
| Weight | B | property | measurement | movement (mass vs force), duel (mass) | velocity, duel outcome |
| Preferred foot | B | property | observation | pass, shot (which foot) | pass/shot direction |

**Why dimensions enter the formulas.** A player of 100 kg and 150 cm with
average strength accelerates less than one of 75 kg and 180 cm with the
same strength: acceleration depends on force *and* mass. If the movement
formula read only `strength`, the model would produce a world where mass
does not exist — which is false. So `movement.json` reads `strength`
(Plane A, through the custodian), `height` and `weight` (Plane B, from
`Player`), and the situation. One formula, inputs from two planes, each
datum in its own plane. The formula crosses the planes; it does not merge
them.

**Weight is measured, not derived.** A scale measures it. The prototype
of this cell derived it from height and maximal force; that was wrong:
a player can be heavy and weak, or light and strong.

### 7.6 States (Plane B, not characteristics)

A state is a condition of the moment. It changes within a match (or
between matches) and it is not a characteristic. States are **computed**:
the trait (Plane A, through the custodian) plus fatigue plus events,
inside the calculator, produce the state. The engine holds the state; the
manager estimates it (often with low confidence).

| Name | Plane | Persist. | Est. | Computed from | Observable effect |
|---|---|---|---|---|---|
| Fatigue | B | state | observation | effort over the match (MD-40) | slower movement, later arrival |
| Condition | B | state | observation | recent load, recovery | baseline of the day |
| Confidence | B | state | Q | trait threshold + events (error, goal) | risk taking, shot choice |
| Morale | B | state | Q | results, context | commitment |
| Pressure load | B | state | Q | accumulated pressure events | composure under pressure |
| Concentration (level) | B | state | Q | trait threshold + fatigue + events | errors, distraction |
| Composure (level) | B | state | Q | trait threshold + pressure load | on-target under pressure |

**The three "double" cases are resolved here.** Concentration,
composure and emotional control looked like they belonged to two planes.
They do not: **the trait is a characteristic in Plane A; the level of the
moment is a state in Plane B, computed from the trait plus context.**
Two datums, two planes, one criterion. The trait is what the player *is*
(hidden, stable); the state is what he *is like today* (visible in its
effects, computed).

**How the state is computed, and where.** The calculator reads the trait
(through the custodian), combines it with fatigue and the events, and
produces the state. The state is an **output** of the calculator, stored
in the engine's state (`MatchState`), not a latent and not a field of
`Player`. This is the piece that the prototype did not have: it stored
psychological traits as states (in `PsychologicalStateMinimal`) without a
trait behind them.

### 7.7 Behaviours (not datums)

| Behaviour | Uses which characteristics | Where it lives |
|---|---|---|
| Scanning (turning the head before receiving) | Vision, concentration | Decision code (F.6) |
| Positioning (where to stand) | Game reading, role | Decision code (F.6) |
| Pressing (when to close down) | Aggressiveness, game reading | Decision code (F.6, later) |

A behaviour is never a latent, never a state, never a saved datum: it is
recomputed from the characteristics and the situation.

### 7.8 Out of scope at L0 (L2)

| Datum | Why out of scope | When |
|---|---|---|
| Leadership | Needs the dressing room, not the match | L2 |
| Motivation | Needs season objectives, contracts | L2 |
| Communication / organisation | Needs the unit that talks, the line that manages | L2 |

They are real, they are football, and they are not L0. They are listed so
that a future session does not restart from zero, and they are not
generated at L0 (no consumer, no context).

### 7.9 Composites (to be split, not stored)

| Composite | Parts | Status |
|---|---|---|
| Set pieces | Free kicks, penalties, corners, throw-ins | Split when OI-48 (rules module) exists |
| Instinct / anticipation | Reading + experience + concentration | Not a datum: a label for an effect |

A composite is never a stored value. Either it is split into its parts, or
it is declared a label that describes an effect of other data.

### 7.10 Summary of the planes

| Plane | Contains | Count |
|---|---|---|
| **A** | Hidden performances: physical, technical, psychological traits, cognitive | 8 + 7 + 4 + 4 = 23 |
| **B** | Dimensions (3) and states (7) | 10 |
| **C** | Estimates, not datums: the manager's view, with confidence | — (future scouting crate) |
| Out | L2 (3), composites (2), behaviours (3) | 8 |

**The plane decides where the datum lives, not how the manager knows it.**
The manager estimates every datum in C, with a confidence that depends on
the method (§2.4): a measurement for a dimension, a test for a physical
performance, many matches for a technical skill. The plane is about who
reads the true value; the estimate is about what the manager sees. Two
questions, two answers, no double home.

## 8. What the manager sees, and why scouting is a game

The manager never reads a true value (§2.4). He reads an estimate, in
Plane C, for every datum: a measurement for a dimension (height with a
ruler), a test for a physical performance (a dynamometer for strength),
many matches for a technical skill (passing), expert judgement for what
has no instrument (composure, reading).

**The confidence of the estimate is what makes scouting a game.** Two
characteristics can both be hidden and both be performances, and still be
known very differently:

- **Strength** is estimated with a test: thirty minutes, low error. The
  manager who has a laboratory knows it well; the one who does not
  estimates it from duels. The asymmetry is resources, not time.
- **Passing** is estimated from matches: months, high error, and the
  error depends on teammates, opponents, position. The asymmetry is time
  and observation, and it is never fully resolved.

This is why the game has scouting: the manager must decide whom to watch,
for how long, and with what method. A world where every characteristic
had an instrumental test would have no scouting; a world where none had
one would have no measurement. The mix is the design.

**Consequence for the model.** The model must not collapse the methods
into one. A characteristic with an instrumental test and a characteristic
with only qualitative judgement are both hidden performances, but they
are known in different ways, and the game is built on that difference.
The three methods — instrumental, statistical-contextual, qualitative
(§2.4) — are a lens for analysis, not a classification of datums. They
become concrete only when the scouting crate exists; today they explain
why the estimate, not the datum, is what the manager sees.

## 9. Where each characteristic lives

- **`Player` (`pcfm-model`)** — the visible data: dimensions and anatomical
  facts. `height_cm`, `weight_kg`, `preferred_foot`. They are measured with
  a simple instrument, they do not change during a match, and they are not
  performances: they are properties of the body. The engine reads them
  directly; the custodian does not hold them.
- **`PlayerLatents` (`pcfm-hidden`, the custodian)** — the hidden
  characteristics: body performances and learned skills. Read only through
  the custodian API, one characteristic per call.
- **`PhysicalStateMinimal` / `PsychologicalStateMinimal`** — states, not
  characteristics. They change within a match or between matches; the
  characteristics do not. This cell does not decide the Rust shape (one
  struct or several); that is a decision for the ADR on the player model.
- **Nothing is "derived and hidden".** A value is either measured and
  visible (`Player`), or hidden and read through the custodian
  (`PlayerLatents`), or a state. A derived value that the engine needs is
  computed by the calculator inside a formula, never stored in a third
  place.

**`strength` (F.7).** It is a hidden physical characteristic: a
performance, not a dimension. You cannot measure it with a ruler; a test
(dynamometer, jump, sprint) estimates it, and the estimate has error. Its
placement in `PlayerLatents` is correct because it is a performance the
engine reads through the custodian, not a visible property like height.
Its family is physical, and its access is instrumental (a test, low noise)
— which is different from a technical skill like passing, which has no
instrument and is discovered by watching matches (high noise). The
distinction between "instrumental" and "observational" access is the point
of this cell.

## 10. Divergence with the prototype (declared)

The prototype's `PlayerLatents` has seven `f32`, one per `resolve_*`:
`finishing`, `passing`, `tackling`, `aerial`, `goalkeeping`, `set_pieces`,
`injury_proneness`. This cell diverges in six ways:

1. **`injury_proneness` is not a technical skill.** It is a physical
   susceptibility. Same container today, different family.
2. **`aerial` is not a pure skill.** It is heading (technique) expressed
   through height and explosive force (physical). The prototype's
   `resolve_duel` never reads `height_cm`.
3. **`set_pieces` is a composite**, not a capability.
4. **The cognitive family does not exist** in the prototype.
5. **The psychological family is a state, not a characteristic** in the
   prototype. The stable traits are not modelled.
6. **The formula is not a contract.** The prototype hard-codes which
   characteristic each `resolve_*` reads. This cell moves the choice into
   `balance/*.json`.

The prototype is not wrong to be simple; it is wrong to be **unaware**.
This cell makes the new engine aware.

## 11. Open decisions (not resolved here)

Each requires a maintainer decision; listed so a new session does not
restart from zero.

1. **Trait by state: how do they combine?** The calculator reads the state;
   the trait sets the baseline. But how exactly? Does fatigue degrade the
   capability (a tired player passes worse) or the body (a tired player
   arrives later)? The choice determines whether T3's "no double
   counting" is respected. **This is the first decision to take**, because
   it touches every formula.

2. **Correlations between characteristics.** In a real match, vision and
   passing are correlated; height and explosive force are correlated;
   finishing and composure are correlated. The prototype draws them
   independently, producing impossible players (vision 0.1, passing 0.9)
   with the same probability as coherent ones. How are correlations
   modelled? (Structural equations? Role profiles? A generator at L3 that
   samples from a joint distribution?)

3. **Development window and learning speed.** Technical skills are learned
   mainly between 8 and 18; after 25 a low passing stays low. Without a
   potential scalar (CA/PA is banned, `00-foundations` §5), how is growth
   modelled? What makes two 18-year-olds with the same passing grow at
   different speeds?

4. **Consistency, not just the mean.** A player may have a high average
   passing and collapse on easy passes. The mean does not capture
   consistency. Is consistency a characteristic of its own, or noise on top
   of the mean?

5. **The bridge from events to estimates (Plane B -> Plane C).** Scouting
   estimates `passing` from completed passes, but completed passes depend
   on teammates, opponents, position. Which statistics feed which
   estimate, and with what noise? Without this, L0 emits events and L2
   cannot read them.

6. **Age, formation history, and context as inputs.** A player's
   characteristics are not generated in a vacuum. These are inputs to
   L1/L2, not characteristics themselves. Where do they live?

7. **Composite characteristics.** `set_pieces` is the first; `vision` may
   be the second (peripheral vision + scanning + reading). When is a
   composite better than its parts, and when does it hide which part is
   acting?

8. **When is a characteristic "dormant" vs "not modelled"?** A
   characteristic with a declared consumer but no formula yet is dormant;
   a characteristic with no consumer and no leaf is an idea, and belongs in
   `IDEAS.md`, not here. The boundary is the leaf in §6.

## 12. References

- `specs/00-foundations.md` §3 (Three Planes), §5 (banned: no CA/PA)
- `docs/adr/003-plane-a-enforcement.md` (Plane A, the only door)
- `docs/adr/028-observability-plane-criterion.md` — **to be rewritten.** The
  criterion "observable in a match" is insufficient: height is not seen
  with precision without a ruler, weight needs a scale, and a dynamometer
  measures strength in half an hour. The criterion that holds is
  **dimension vs performance**: a dimension is a property of the body
  (height, weight), a performance is something the player does (strength,
  passing). Dimensions live in `Player`; performances live hidden and are
  read through the custodian. The three-component structure (§4) is the
  other half of the rewrite.
  Old line: "observability as the A/B criterion; no model value in a
  `.rs` without a derivation or `balance/`" — the second half stays valid.
- `specs/L0-match/03-entities-minimal.md` §2.1–2.3 (Player,
  PhysicalStateMinimal, PsychologicalStateMinimal) and §3.2 (the latents
  are the shadow of the prototype functions)
- `specs/L0-match/10-ground-truth.md` — the `resolve_*` contract. **Note:**
  it describes the prototype contract (the `resolve_*` that reads the
  latent and applies a hardcoded formula). The new contract has three
  components (§4); `10-ground-truth.md` will need the same rewrite.
- `docs/context/SESSION-STATE.md` MD-23 (fidelity first, cost as
  constraint), MD-24 (amended), MD-40 (fatigue), MD-53 (one module, one
  function), MD-54 (the observability lens)
- OI-44 (event-keyed noise), OI-53 (keyed latents, extension of OI-44),
  OI-54 (field audit), OI-51 (performance threshold), OI-48 (rules module)
