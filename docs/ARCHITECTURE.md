## 6. Layered match model (L0/L1/L2/L3)

The project is deliberately structured as nested layers. A layer does not replace the previous one: it expands it.

- L0 = match engine / causal state / deterministic simulation
- L1 = world-space / player + ball / positional state
- L2 = tactical policy / live team behaviour / role influence
- L3 = presentation / 2D / 3D / HTML / text rendering

This is the core rule:

- the renderer cannot invent the semantics of the match
- the simulation is the source of truth
- 2D/3D views are projections of the authoritative state

The layered rule is also the correct answer to the question of “what is a real match?”
At the match level, what matters is not a single screenshot, but the state at each tick and the causal chain behind it.

This is documented more fully in:

- `docs/context/LAYERED-MATCH-ARCHITECTURE.md`
- `docs/context/TICK-TEMPORAL-STATE.md`
- `docs/context/TODO-LAYERED-MATCH-ARCHITECTURE.md`

This design is intended to keep the engine trustworthy, deterministic, and extensible while still allowing later 2D and 3D reconstruction layers.
