# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. History is in `git log`; this file does not duplicate it.

**Last commit verified:** `96e8120` — "docs(session-state): closing final — LCV to 1e58ad8"

**Note on the field above:** it points to the last commit *before* this bookmark update (S1). It will look "stale" compared to HEAD; expected. Do not "fix" it without updating the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md`. Read at startup to know what is not yet decided; do not implement from it.
**Decision map:** `docs/context/DECISION-MAP.md` — the map of *decisions*, not steps. Read before choosing a step that touches the engine model. The Plan executes; the map shows the forks the Plan cannot represent as a sequence. Status: sketch (2026-11-02), not ratified.

**Legenda:** 🔵 in corso · 🟡 prossimo · ⬜ aperto · 🔴 urgente

**Analysis reports (vivi):** `docs/context/analysis/2026-11-01-L0-verification-checklist.md` (13 item; G2/G10 ora OI-23/24; G1/OI-22 chiuso in slice B2). Archiviati (2026-11-01): `09-50`, `12-30`, `12-40`, `10-25`, `22-28` (censimento invecchiato: L0 cambiato 3 volte il 2026-11-01, i G ancora veri sono OI individuali).

**Vision (2026-11-01, read before the Plan):** the project builds a
**phenomenological** simulation of a football match, not a physics engine
and not a renderer. Rain makes perception noisy, wet grass raises the
probability of a slip, a strong press forces an error: the effects are
**modelled**, deterministic (integer coordinates, named RNG streams),
calibratable (parameters in `balance/`, declared arbitrary), and every
event carries the fields to understand why. The full note is
`docs/context/L0-POSITIONAL-SLICE.md` — §0 is the vision, §6-ter is the
map of the events, §7 is where we are. **Read it before the Plan.** A
physical engine is not a later step of this project: it is a different
paradigm, not planned.

**Gate: reading before touching the engine.** `state.rs`, `spatial.rs`,
`perception.rs`, `orchestrator.rs`, `save.rs`, `golden_l0.rs` may only be
modified by a commit that carries `Note-Read: L0-POSITIONAL-SLICE §0,
§6-ter, §7`. The trailer is self-declared but visible in `git log`; the
hook check is deferred to a dedicated session (D5: an artefact today, a
gate tomorrow). Touching those files without the note is how the wrong
engine gets built.


## Open items (what is not decided or verified yet)

| # | Cosa | Prima azione |
|---|------|--------------|
| OI-6 | `docs/performance.md` e `docs/save-security.md` mancanti | Decidere se crearli pre-release (con step 13) |
| OI-7 | `ProjectionProfile` senza metodo di calcolo (Gap B) | Decidere quando apre `L2-scouting` |
| OI-8 | `ScoutReport.DeclaredBias` senza formula (Gap C) | Decidere quando apre `L2-scouting` |
| OI-23 | **G2 — panic nel bus non isolato**: un handler che panica propaga e perde gli eventi in coda (`event_bus.rs:80-85`, documentato come "known limitation"). ADR-004 prevede `catch_unwind` al boundary | Decidere: `catch_unwind` al boundary o lasciare la limitazione. Tocca D2 (panic nelle path di simulazione). Sessione dedicata |
| OI-24 | **G10 — getter pubblici su `GroundTruth`**: `world_seed()` e `match_id()` (`pcfm-hidden/src/lib.rs:81,87`). Non sono latenti (identificatori), ma ADR-003 dice "no public getters". Contratto dichiarato vs API reale | Decidere: rimuovere i getter, o emendare ADR-003 per dichiararli non-latenti. Sessione dedicata |
| OI-25 | **Modello di movimento tra gli eventi.** Letti §0, §6-ter, §7; non §1–§6-bis: cosa muove gli agenti tra un tick e l'altro non è verificato. Slice C (`Slip` con velocity) lo presuppone | Leggere §1–§6-bis della nota, `04-match-engine.md` §2.2, `orchestrator.rs` |
| OI-26 | **Granularità del tick e densità degli eventi.** Cosa succede nei tick senza evento: non letto. Se "niente", il paradigma è testabile solo evento-per-evento | Stessa lettura di OI-25; poi decidere se serve `Pressure` continuo o stato persistente |
| OI-27 | **Hook L0–L2 sulla forma collettiva (#5).** Contratto inesistente (`INFRA-014`, `INFRA-016` sketch). Non blocca slice C, ma decide se le tattiche saranno bolt-on | Leggere §2 e §6-bis della nota, poi `INFRA-014`/`INFRA-016` in `IDEAS.md` |
| OI-28 | **Contraddizione §3.4 vs §6-ter della nota L0.** §3.4 dice "movement is part of slice C"; §6-ter/§7/Plan elencano C = Slip+ControlError+Pressure, senza movimento | Decidere quale delle due fonti vale, aggiornare l'altra nello stesso commit |
| OI-29 | **`AgentState.velocity` è `(0, 0)` hardcoded e mai aggiornata** (`spatial.rs:76,87`). `Slip` (sezione §6-ter) la usa come input: oggi leggerebbe zero costante | Decidere: modello di movimento, riformulare `Slip`, o rimandarlo (dipende da OI-25) |
| OI-30 | **`balance/` contiene solo `stoppage_time.json`.** `pitch_wear` e `weather` (citati per slice C) non esistono come parametri | Creare i JSON quando il modello li consuma, non prima (D3) |
| OI-31 | **Drift spec↔codice su `EventType`.** `05-match-events.md` §2.2 non elenca `KickOff`/`SecondHalf`; il codice li ha (`pcfm-stats/src/event.rs`). La spec dice anche "The set is closed at L0" ma `Slip`/`ControlError` non ci sono | Decidere se allineare lo spec al codice, o viceversa, e se aggiungere `Slip`/`ControlError` richiede un ADR |
| OI-32 | **`crates/pcfm-match/src/narrative.rs.tmp` orfano** su disco (non tracciato, non in `git status`: probabile `.gitignore`). Residuo di una sed/awk precedente | Verificare con `git ls-files` e `git check-ignore`, poi rimuovere |
| OI-33 | **Il motif è a stadi (L0 = causalità locale, L2 = causale globale).** Il pillar 5 ("perché ho perso?") non è rispondibile a L0 senza tattiche e psicologia. Va dichiarato esplicitamente nel README/manifesto, o il prodotto promette più di quanto L0 possa dare | Decidere se dichiarare la promessa a stadi in `README.md` o in un ADR |
| OI-34 | **Tensione D2 nel codice esistente.** `state.rs:320` (`expect("spatial state contains every starter")`) e `state.rs:325` (`unwrap_or(max)`) sono in path di simulazione. D2 dice "no `unwrap`/`expect`/`panic!` in simulation paths". Documentati unreachable, ma letteralmente in violazione | Valutare sostituzione con `Option` + gestione esplicita, in una sessione dedicata (non ora: non blocca slice C) |

## Plan (in dependency order)

| Pr. | # | Cosa | Dipende da | Prima azione |
|---|---|---|---|---|
| ⬜ | 11 | **GitHub agent Phase 2 — deferred (2026-11-02).** Ruleset **disabled**. Reactivate when one holds: (a) a second reviewer exists; (b) the project ships; (c) a write token is granted to an agent. `Evaluate` is not available (personal repo, non-Enterprise). Until then, Phase 1: direct push by the maintainer. See `AGENT-POLICY.md` §6 | — | Rileggere `AGENT-POLICY.md` §6 quando una delle tre condizioni si presenta |
| ⬜ | 5 | Review sketch ×3 | L2-tactics, input, L2-staff | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| ⬜ | 7 | Verification cascade | — | Decidere da quale file parte la cascade (candidato: `README.md`) |
| ⬜ | 8 | **ADR-019 deferred 4-9: verificati 2026-11-01, nessuno chiudibile.** 4 (UX) e 5 (resolution per piattaforma) richiedono UI/mobile reali; 6 (adaptive) richiede un ADR dedicato; 7 (LOD override) un emendamento ad ADR-013; 8 (zona di influenza) `docs/calibration/` vuota; 9 (role→tactic) `L2-tactics` non esiste. Non è un rinvio: è D3, non c'è contenuto concreto | — | Riaprire quando la dipendenza esiste. Nessuna azione ora |
| ⬜ | 10 | `L1-world/01-nation` | — | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| ⬜ | 19 | **Narrativa: legge i `fields` degli eventi.** Oggi `narrative.rs` ignora `fields` per ogni evento: `Tackle` non dice chi ha perso la palla (`from`/`to`), `Shot` non dice se è gol (`goal`), `Duel` non dice contro chi (`winner_slot`). Viola il criterio di §0 ("every event carries the fields to understand why"). Fix: `narrate_event` legge `fields`; test sul `Tackle` con `from`/`to`. Non tocca i sei file del motore (niente `Note-Read`), non cambia `SIM_VERSION` (la narrativa non è nel `chain_hash`) | — | Leggere `crates/pcfm-match/src/narrative.rs` (`render_text` e `narrate_event`), poi `fields` in `05-match-events.md` §2.1 |
| ✅ | 13 | **Sicurezza dipendenze**: Dependabot alerts **attivato dal maintainer il 2026-11-01** (non era attivo di default, contrariamente a quanto avevo assunto). `cargo-deny` (licenze, ban) resta pre-release. | — | Chiuso. Se in futuro serve `cargo-audit` in CI, è un OI nuovo |
| ✅ | A | **Slice A — fondamenta spaziali**: `spatial.rs` (`AgentState`, `SpatialState`, griglia 4-4-2, 6 test). Fatto `39c444b` | — | Fatto |
| ✅ | B1 | **Slice B parte 1 — percezione + carrier**: `perception.rs` (4 test), `SpatialState.carrier`. Fatto `c231c98`. Non usati dal motore | A | Fatto |
| ✅ | B2 | **Slice B2 — chi agisce dallo spazio + conquista come evento.** `MatchState` possiede `SpatialState`; `pick_players` → `pick_from_space`; duello vinto dal difensore emette `Tackle` con `from`/`to`; pass incompleto/fallo azzerano il carrier; `SIM_VERSION` 8, golden rigenerato, OI-22. Fatto. 133 test | B1 | Fatto. Vedi slice C |
| ⬜ | T | **Step 19 — la narrativa legge i `fields` (nodo trasversale, vale in ogni strada).** `narrate_event` legge `fields`: `Tackle` con `from`/`to`, `Shot` con `goal`, `Duel` con `winner_slot`. Test sul `Tackle`. Non tocca i sei file del motore (niente `Note-Read`), non cambia `SIM_VERSION` | — | Leggere `narrative.rs` (`render_text`, `narrate_event`) e `05-match-events.md` §2.1 |
| ⬜ | C | **Slice C — primi eventi fenomenologici.** `Slip` e `ControlError` da `pitch_wear` × `weather` (parametri in `balance/`, arbitrari dichiarati); `Pressure` emesso. Vedi §6-ter per la mappa | B2 | Leggere §0, §6-ter, §7 (trailer `Note-Read`), poi aprire `balance/` |
## Decisioni operative (2026-11-01)

- **Finding (2026-11-01, closing):** the DP archival check is no longer
  executable. `DPs-Applied` became optional with the ritual cut, so the
  `git log` check cannot tell which DPs were used. Not urgent (3 active,
  max 10). If the DPs grow, reintroduce the trailer or an alternative
  check.

- **Organizzazione futura:** branch per **area di lavoro**, non per livello architetturale. I livelli si esprimono nei crate, non nei branch. Prerequisito del parallelismo: CI + branch protection (OI-20).
- **Board testuale e step 18:** il board di ADR-025 **non è la base** della demo posizionale. Con step 18 gli eventi diventano tracce di uno stato continuo; il board testuale scade, non si estende. La TUI sarà il veicolo del posizionale. **Non investire in abbellimenti del board.**

