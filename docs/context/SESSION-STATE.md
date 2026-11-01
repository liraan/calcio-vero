# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. History is in `git log`; this file does not duplicate it.

**Last commit verified:** `cc81406` — "docs(analysis): archive 10-25, fuse OI-13 into OI-17 (OI-21, OI-13 closed)"

**Note on the field above:** it points to the last commit *before* this bookmark update (S1). It will look "stale" compared to HEAD; expected. Do not "fix" it without updating the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md`. Read at startup to know what is not yet decided; do not implement from it.

**Legenda:** 🔵 in corso · 🟡 prossimo · ⬜ aperto · 🔴 urgente

**Analysis reports (vivi):** `docs/context/analysis/2026-10-31-22-28-review.md` (censimento L0, OI-17 — resta vivo: è la base del censimento), `docs/context/analysis/2026-11-01-L0-verification-checklist.md` (13 item, OI-13 fuso in OI-17). Archiviati (2026-11-01): `09-50`, `12-30`, `12-40`, `10-25` (3/6 finding chiusi, 2 assorbiti da G2/OI-14, 1 era OI-21).

## Open items (what is not decided or verified yet)

| # | Cosa | Prima azione |
|---|------|--------------|
| OI-6 | `docs/performance.md` e `docs/save-security.md` mancanti | Decidere se crearli pre-release (con step 13) |
| OI-7 | `ProjectionProfile` senza metodo di calcolo (Gap B) | Decidere quando apre `L2-scouting` |
| OI-8 | `ScoutReport.DeclaredBias` senza formula (Gap C) | Decidere quando apre `L2-scouting` |
| OI-14 | Golden test 10 stagioni non esiste (`CHANGELOG-SIM:42`, `00-foundations` §4) | Decidere forma (L1 o N partite) e priorità |
| OI-17 | **Censimento L0 (ex OI-13 fuso qui): 12 divergenze spec↔code** (`analysis/2026-10-31-22-28`), più i finding dei report archiviati non assegnati a step. G11 migliorato (4→8 eventi su 31 con ADR-026), non chiuso | Leggere §5 "Verdict" e classificare i 12 G: obbligatorio per L1/L2, o "target design, non current". I secondi si dichiarano in `specs/L0-match/00-overview.md` (una riga) e si chiudono. I primi diventano step |
| OI-19 | View model separation: enforcement compile-time deferred | Decidere se vincolo di build o policy documentale |

## Plan (in dependency order)

| Pr. | # | Cosa | Dipende da | Prima azione |
|---|---|---|---|---|
| 🟡 | 3 | Terminologia (T1+T2, T3 done) | — | Leggere `PLUGIN_DEVELOPER_GUIDE.md` §1: `sed -n '13,40p'` |
| 🟡 | 11 | GitHub agent Phase 2 | ruleset attivo | Decisione del maintainer: attivare o no il ruleset |
| ⬜ | 5 | Review sketch ×3 | L2-tactics, input, L2-staff | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| ⬜ | 7 | Verification cascade | — | Decidere da quale file parte la cascade (candidato: `README.md`) |
| ⬜ | 8 | ADR-019 deferred 4-9 | — | Rileggere ADR-019 §Deferred, decidere quali dei 4-9 chiudere |
| ⬜ | 9 | Ratify ADR-017 | — | Leggere ADR-017, decidere: ratificare o rinviare |
| ⬜ | 10 | `L1-world/01-nation` | — | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| ✅ | 13 | **Sicurezza dipendenze**: Dependabot alerts **attivato dal maintainer il 2026-11-01** (non era attivo di default, contrariamente a quanto avevo assunto). `cargo-deny` (licenze, ban) resta pre-release. | — | Chiuso. Se in futuro serve `cargo-audit` in CI, è un OI nuovo |
| ⬜ | 18 | **Motore posizionale (step 18)**: prerequisito L2, non esiste (D3). Fermarsi *prima*, non iniziare in coda a una sessione | L2 | Leggere `crates/pcfm-match/src/state.rs` e contare gli `EventType` emessi vs risolti |
## Decisioni operative (2026-11-01)

- **Organizzazione futura:** branch per **area di lavoro**, non per livello architetturale. I livelli si esprimono nei crate, non nei branch. Prerequisito del parallelismo: CI + branch protection (OI-20).
- **Board testuale e step 18:** il board di ADR-025 **non è la base** della demo posizionale. Con step 18 gli eventi diventano tracce di uno stato continuo; il board testuale scade, non si estende. La TUI sarà il veicolo del posizionale. **Non investire in abbellimenti del board.**

