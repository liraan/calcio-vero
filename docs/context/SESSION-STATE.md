# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. History is in `git log`; this file does not duplicate it.

**Last commit verified:** `ecd24fa` — "docs(guide): step 3 — module/crate terminology"

**Note on the field above:** it points to the last commit *before* this bookmark update (S1). It will look "stale" compared to HEAD; expected. Do not "fix" it without updating the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md`. Read at startup to know what is not yet decided; do not implement from it.

**Legenda:** 🔵 in corso · 🟡 prossimo · ⬜ aperto · 🔴 urgente

**Analysis reports (vivi):** `docs/context/analysis/2026-11-01-L0-verification-checklist.md` (13 item; G1/G2/G10 ora OI-22/23/24). Archiviati (2026-11-01): `09-50`, `12-30`, `12-40`, `10-25`, `22-28` (censimento invecchiato: L0 cambiato 3 volte il 2026-11-01, i G ancora veri sono OI individuali).

## Open items (what is not decided or verified yet)

| # | Cosa | Prima azione |
|---|------|--------------|
| OI-6 | `docs/performance.md` e `docs/save-security.md` mancanti | Decidere se crearli pre-release (con step 13) |
| OI-7 | `ProjectionProfile` senza metodo di calcolo (Gap B) | Decidere quando apre `L2-scouting` |
| OI-8 | `ScoutReport.DeclaredBias` senza formula (Gap C) | Decidere quando apre `L2-scouting` |
| OI-22 | **G1 — stream RNG senza `match_id`**: `"match:actions"`, `"match:action_kind"`, `"match:player_pick"` (`state.rs:60-64`). Due partite con lo stesso seed producono stream identici. Il census `22-28` lo segnala come "deterministico ma non per-match" | Prefissare gli stream con `match_.id` in `MatchState::new`. Richiede `SIM_VERSION` bump (cambia la sequenza RNG). Sessione dedicata |
| OI-23 | **G2 — panic nel bus non isolato**: un handler che panica propaga e perde gli eventi in coda (`event_bus.rs:80-85`, documentato come "known limitation"). ADR-004 prevede `catch_unwind` al boundary | Decidere: `catch_unwind` al boundary o lasciare la limitazione. Tocca D2 (panic nelle path di simulazione). Sessione dedicata |
| OI-24 | **G10 — getter pubblici su `GroundTruth`**: `world_seed()` e `match_id()` (`pcfm-hidden/src/lib.rs:81,87`). Non sono latenti (identificatori), ma ADR-003 dice "no public getters". Contratto dichiarato vs API reale | Decidere: rimuovere i getter, o emendare ADR-003 per dichiararli non-latenti. Sessione dedicata |

## Plan (in dependency order)

| Pr. | # | Cosa | Dipende da | Prima azione |
|---|---|---|---|---|
| 🟡 | 3 | Terminologia (T1+T2, T3 done) | — | Leggere `PLUGIN_DEVELOPER_GUIDE.md` §1: `sed -n '13,40p'` |
| 🟡 | 11 | GitHub agent Phase 2 | ruleset attivo | Decisione del maintainer: attivare o no il ruleset |
| ⬜ | 5 | Review sketch ×3 | L2-tactics, input, L2-staff | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| ⬜ | 7 | Verification cascade | — | Decidere da quale file parte la cascade (candidato: `README.md`) |
| ⬜ | 8 | **ADR-019 deferred 4-9: verificati 2026-11-01, nessuno chiudibile.** 4 (UX) e 5 (resolution per piattaforma) richiedono UI/mobile reali; 6 (adaptive) richiede un ADR dedicato; 7 (LOD override) un emendamento ad ADR-013; 8 (zona di influenza) `docs/calibration/` vuota; 9 (role→tactic) `L2-tactics` non esiste. Non è un rinvio: è D3, non c'è contenuto concreto | — | Riaprire quando la dipendenza esiste. Nessuna azione ora |
| ⬜ | 10 | `L1-world/01-nation` | — | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| ✅ | 13 | **Sicurezza dipendenze**: Dependabot alerts **attivato dal maintainer il 2026-11-01** (non era attivo di default, contrariamente a quanto avevo assunto). `cargo-deny` (licenze, ban) resta pre-release. | — | Chiuso. Se in futuro serve `cargo-audit` in CI, è un OI nuovo |
| ⬜ | 18 | **Motore posizionale (step 18)**: prerequisito L2, non esiste (D3). Fermarsi *prima*, non iniziare in coda a una sessione | L2 | Leggere `crates/pcfm-match/src/state.rs` e contare gli `EventType` emessi vs risolti |
## Decisioni operative (2026-11-01)

- **Organizzazione futura:** branch per **area di lavoro**, non per livello architetturale. I livelli si esprimono nei crate, non nei branch. Prerequisito del parallelismo: CI + branch protection (OI-20).
- **Board testuale e step 18:** il board di ADR-025 **non è la base** della demo posizionale. Con step 18 gli eventi diventano tracce di uno stato continuo; il board testuale scade, non si estende. La TUI sarà il veicolo del posizionale. **Non investire in abbellimenti del board.**

