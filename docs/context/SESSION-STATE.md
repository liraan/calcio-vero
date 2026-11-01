# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. History is in `git log`; this file does not duplicate it.

**Last commit verified:** `7428a6d` — "docs(rules): cut the ritual — bookmark 5→3, closing 6→3, startup 5→3"

**Note on the field above:** it points to the last commit *before* this bookmark update (S1). It will look "stale" compared to HEAD; expected. Do not "fix" it without updating the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md`. Read at startup to know what is not yet decided; do not implement from it.

**Legenda:** 🔵 in corso · 🟡 prossimo · ⬜ aperto · 🔴 urgente

**Analysis reports (vivi):** `docs/context/analysis/2026-10-31-22-28-review.md` (censimento L0, OI-17), `docs/context/analysis/2026-11-01-10-25-review.md` (audit repo, OI-20/21), `docs/context/analysis/2026-11-01-L0-verification-checklist.md` (13 item, OI-13). Archiviati (2026-11-01): `09-50`, `12-30`, `12-40`.

## Open items (what is not decided or verified yet)

| # | Cosa | Prima azione |
|---|------|--------------|
| OI-4 | `README.md:148` cita `pcfm-gen` (non tra i 9 crate) | `grep -rn "pcfm-gen" --include="*.md" --include="*.rs" .` |
| OI-6 | `docs/performance.md` e `docs/save-security.md` mancanti | Decidere se crearli pre-release (con step 13) |
| OI-7 | `ProjectionProfile` senza metodo di calcolo (Gap B) | Decidere quando apre `L2-scouting` |
| OI-8 | `ScoutReport.DeclaredBias` senza formula (Gap C) | Decidere quando apre `L2-scouting` |
| OI-13 | Finding dei 2 report non assegnati a step (riformulato 2026-10-31) | Decidere: promuovere a step, o dichiararli "target design" e chiudere |
| OI-14 | Golden test 10 stagioni non esiste (`CHANGELOG-SIM:42`, `00-foundations` §4) | Decidere forma (L1 o N partite) e priorità |
| OI-15 | A2 esegue `cargo test` una terza volta per sessione | Decidere: saltare A2 in closing, o tenerlo come safety net |
| OI-17 | Censimento L0: 12 divergenze spec↔code (`analysis/2026-10-31-22-28`) | Leggere §5 "Verdict" e decidere quali gap sono obbligatori |
| OI-19 | View model separation: enforcement compile-time deferred | Decidere se vincolo di build o policy documentale |
| OI-21 | Retention policy for analysis artifacts | Definire regola di archiviazione |
| OI-22 | Draft `draft/layered-match-2026-11-01` non compila (15 errori), non riparato | Chiudere quando step 18 apre: ripartire dal tag |

## Plan (in dependency order)

| Pr. | # | Cosa | Dipende da |
|---|---|---|---|
| 🟡 | 2 | Fix ADR-003: `pcfm-demo` importa `GroundTruth` (Plane A). Opzione (a): spostare in `pcfm-fixtures`; richiede autorizzazione `docs/adr/` | — |
| 🟡 | 17b | Fusione Prime azioni nel Piano; aggiornare `gates.sh` B1 e closing Gate B | — |
| 🟡 | 3 | Terminologia (T1+T2, T3 done) | — |
| 🟡 | 11 | GitHub agent Phase 2 | ruleset attivo |
| 🟡 | 21 | Archiviazione review in `analysis/` | — |
| ⬜ | 5 | Review sketch ×3 | L2-tactics, input, L2-staff |
| ⬜ | 7 | Verification cascade | — |
| ⬜ | 8 | ADR-019 deferred 4-9 | — |
| ⬜ | 9 | Ratify ADR-017 | — |
| ⬜ | 10 | `L1-world/01-nation` | — |
| ⬜ | 13 | `cargo-audit`/`cargo-deny` pre-release (o Dependabot) | — |
| ⬜ | 18 | **Motore posizionale (step 18)**: prerequisito L2, non esiste (D3). Fermarsi *prima*, non iniziare in coda a una sessione | L2 |

## Decisioni operative (2026-11-01)

- **Organizzazione futura:** branch per **area di lavoro**, non per livello architetturale. I livelli si esprimono nei crate, non nei branch. Prerequisito del parallelismo: CI + branch protection (OI-20).
- **Board testuale e step 18:** il board di ADR-025 **non è la base** della demo posizionale. Con step 18 gli eventi diventano tracce di uno stato continuo; il board testuale scade, non si estende. La TUI sarà il veicolo del posizionale. **Non investire in abbellimenti del board.**

## Plan — Prime azioni (tabella temporanea, fusa nel Plan da step 17b)

| # | Prima azione |
|---|---|
| 2 | Leggere ADR-003 §Exception: `sed -n '204,235p' docs/adr/003-plane-a-enforcement.md` |
| 3 | Leggere `PLUGIN_DEVELOPER_GUIDE.md` §1: `sed -n '13,40p'` |
| 5 | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| 7 | Decidere da quale file parte la cascade (candidato: `README.md`) |
| 8 | Rileggere ADR-019 §Deferred, decidere quali dei 4-9 chiudere |
| 9 | Leggere ADR-017, decidere: ratificare o rinviare |
| 10 | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| 11 | Decisione del maintainer: attivare o no il ruleset |
| 13 | Installare `cargo-audit` e `cargo-deny` (o verificarne la disponibilità) |
| 17b | Aprire `scripts/gates.sh` blocco B1 e il closing Gate B; decidere la forma |
| 21 | Archiviare la review storica in `analysis/archive/` se non più operativa |
