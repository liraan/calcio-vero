# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. Update at the end of a session, not at the start.

**Last commit verified:** `44cf487` — "docs(adr): ADR-025 — TUI board renderer for pcfm-demo (proposed)"

**Note on the field above:** it points to the last commit *before* this
bookmark update (semantics S1). It will look "stale" compared to HEAD; that
is expected. Do not "fix" it by pointing to a newer commit without updating
the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md` — open and rejected ideas. Read it at session start to see what is not yet decided; do not implement from it.

<!-- Formato Piano: legenda in alto, tabella "Da fare" con emoji di priorità, dettagli sotto, prime azioni per step in tabella separata. Se cambi la forma, aggiorna questa nota nello stesso commit. -->

**Legenda:** ✅ done · 🔵 in corso · ⚫ bloccato · 🔴 urgente · 🟡 prossimo · ⬜ aperto

**Analysis reports (vivi):**

- `docs/context/analysis/2026-10-31-22-28-review.md` — L0 operational census; OI-17. Snapshot at `246f64b`; archive when L0 changes, do not update in place.
- `docs/context/analysis/2026-11-01-10-25-review.md` — repo hygiene and workflow audit; snapshot of repo hardening status. Archive when the workflow and follow-up actions are either implemented or superseded.

**Open items (what is not decided or verified yet):**

| # | Cosa | Perché aperto | Prima azione |
|---|------|---------------|--------------|
| OI-4 | **`README.md:148` cita `pcfm-gen`** come esistente, ma non è tra i 9 crate | Verificare se è nota intenzionale o stale | `grep -rn "pcfm-gen" --include="*.md" --include="*.rs" .` |
| OI-6 | **`docs/performance.md` e `docs/save-security.md`** mancanti | Mai creati, non tracciati altrove | Decidere se e quando crearli (probabilmente pre-release, insieme a step 13) |
| OI-7 | **Gap B: `ProjectionProfile` senza metodo di calcolo** | Mai tracciato altrove | Decidere il metodo quando si apre `L2-scouting` |
| OI-8 | **Gap C: `ScoutReport.DeclaredBias` senza formula** | Mai tracciato altrove | Decidere quando si apre `L2-scouting` |
| OI-13 | **Deferred a L1: finding dei 2 report non assegnati a step** (con `conforms_to v1.1` non-bug, riformulato 2026-10-31) | Nessuno è bloccante per L0; diventano step quando si apre L1 | Decidere se vanno chiusi come non-issue o trasformati in backlog L1 |
| OI-14 | **Golden test 10 stagioni non esiste** (`CHANGELOG-SIM:42` lo richiede; `00-foundations` §4 lo elenca) | Manca scelta della maintainer: forma (i) L1 o (ii) N partite con rose fisse | Decidere forma e priorità di implementazione |
| OI-15 | **A2 esegue `cargo test` una terza volta per sessione** (startup step 4, closing C2, closing gates A2) | Mai deciso: tenere come safety net o rimuovere? | Decidere: (a) saltare A2 in closing, (b) mantenere safety net, (c) spostare in un script più esplicito |
| OI-17 | **Censimento operativo L0: 12 divergenze spec↔code** (`analysis/2026-10-31-22-28-review.md`) | Fotografia; nessuna decisione su quali gap chiudere prima di L1/L2 | Leggere §5 "Verdict" e decidere quali gap diventano obbligatori |
| OI-18 | **Ottimizzazione dei rituali: R1–R6** (startup step 3 rilegge prompt; "unexpected observation" doppia in 2a/step 4; `cargo test` doppio = OI-15; startup step 1 ambiguo sotto S1; startup step 1/3/step 4 conflitti) | Problemi di rituale e ridondanza, non bug di codice | Ridurre ridondanze, definire un rituale unico stabile |
| OI-19 | **View model separation: enforcement compile-time deferred** (ADR-025) | ADR-022 §3 rende normativa la separazione, ma il prototipo in `pcfm-demo` la impone solo via un test strutturale | Decidere se rendere la separazione un vincolo in build/test o lasciarla come policy documentale |
| OI-20 | **Repo hardening: GitHub Actions CI + required checks + workflow evidence** | Nessun workflow GitHub Actions ancora presente; local hook non è un vincolo remoto | Creare `.github/workflows/ci.yml`, attivare required checks in GitHub, documentare installazione hook |
| OI-21 | **Retention policy for analysis artifacts** | Le review in `docs/context/analysis/` possono essere confuse con documentazione canonica | Definire regola di archiviazione e mantenere solo report attivi |

**Da fare (in dependency order):**

| Pr. | #   | Cosa | Dipende da |
|-----|-----|------|------------|
| 🔵  | 12  | TUI demo (IDEA-INFRA-006): **board + events, not pitch** (scope 2026-10-31). ADR-025 (proposed) scritto e installato — **ratificare ADR-025**, poi scrivere `viewmodel.rs` + `board.rs` + `render.rs` | — |
| 🟡  | 2   | Fix ADR-003 (nota rename fatta `a391b85`; restano `demo.rs`, autorizzazione `pcfm-demo`) | — |
| 🟡  | 3   | Terminologia (T1+T2, T3 done) | — |
| 🟡  | 11  | GitHub agent Phase 2 | ruleset attivo |
| 🟡  | 17b | **Fusione Prime azioni nel Piano**: aggiungere colonna/riga "first action" al Plan, rimuovere la tabella separata, aggiornare B1 in `scripts/gates.sh` e il testo closing Gate B in `NEW-SESSION-PROMPT.md` | — |
| 🟡  | 20  | **Repo hardening: GitHub Actions CI + required checks + hook install docs** | Nessun vincolo remoto attivo |
| 🟡  | 21  | **Archiviazione review in `analysis/`** | Decisione di mantenere solo snapshot attivi |
| ⬜  | 5   | Review sketch ×3 | L2-tactics, input, L2-staff |
| ⬜  | 7   | Verification cascade | — |
| ⬜  | 8   | ADR-019 deferred 4-9 | — |
| ⬜  | 9   | Ratify ADR-017 | — |
| ⬜  | 10  | `L1-world/01-nation` | — |
| ⬜  | 13  | `cargo-audit`/`cargo-deny` pre-release | — |
| ⬜  | 18  | **Ristrutturazione e riscrittura di L0 (posizionale con coordinate)** — non aprire ora: L2 non esiste (D3) | L2 (rating/scouting) come consumatore concreto |

**Dettagli (solo dove servono):**

**12. TUI demo (IDEA-INFRA-006)** — **scope (2026-10-31): board + events, not pitch.** Validate the split renderer/engine (ADR-022): a second renderer reads `MatchResult` without touching the engine internals. This is the current session priority.

**2. Fix ADR-003** — ADR-003 still names `pcfm-debug` and `demo.rs`; `pcfm-demo` reads Plane A without authorization. ADR-023 exists (2026-10-31, observed cause traces) and does not cover ADR-003 fully. Must be resolved.

**3. Terminologia** — T1 (small): `PLUGIN_DEVELOPER_GUIDE.md` §1 + `GLOSSARY.md`. T2 (small): "core module" → "framework crate" in §2 and §11. T3 (large, one commit): rename `modules/` → `specs/` and align references.

**17b. Fusione Prime azioni nel Piano** — Split di DP-03. Il Plan prende una cella "first action" (colonna o riga subito sotto); la tabella "Prime azioni" sparisce; `scripts/gates.sh` B1 controlla la presenza della prima azione, e il closing Gate B in `NEW-SESSION-PROMPT.md` va allineato.

**20. Repo hardening: GitHub Actions CI + required checks + hook install docs** — The repo already contains a local policy via `.githooks/pre-commit`, but this is not sufficient as a public enforcement layer. The immediate fix is to add `.github/workflows/ci.yml`, trigger on PR/push, and require the job in GitHub branch protection. The contributor docs must also state how to install the hook.

**21. Archiviazione review in `analysis/`** — The review files are useful snapshots, but they are not source-of-truth repository state. Keep the active report; archive older analysis when superseded or ratified.

**Prime azioni (per step):**

| #   | Prima azione |
|-----|--------------|
| 2   | Leggere ADR-003 §Exception: `sed -n '204,235p' docs/adr/003-plane-a-enforcement.md` |
| 3   | Leggere `PLUGIN_DEVELOPER_GUIDE.md` §1: `sed -n '13,40p'` |
| 5   | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| 7   | Decidere da quale file parte la cascade (candidato: `README.md`) |
| 8   | Rileggere ADR-019 §Deferred, decidere quali dei 4-9 chiudere (3 già resolved) |
| 9   | Leggere ADR-017, decidere: ratificare o rinviare |
| 10  | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| 11  | Decisione del maintainer: attivare o no il ruleset |
| 12  | Ratificare ADR-025 (`docs/adr/025-tui-board-renderer.md`, `proposed`): leggere, decidere, `Status: accepted`. Poi aprire `crates/pcfm-demo/src/` e scrivere `viewmodel.rs` + `board.rs` + `render.rs`. |
| 13  | Installare `cargo-audit` e `cargo-deny` (o verificarne la disponibilità) e lanciarli sul workspace |
| 17b | Aprire `scripts/gates.sh` blocco B1 (righe 67-74) e il testo closing Gate B in `NEW-SESSION-PROMPT.md`; decidere la forma della colonna "first action" nel Plan |
| 20  | Creare `.github/workflows/ci.yml`, documentare `git config core.hooksPath .githooks`, e impostare required checks in GitHub |
| 21  | Archiviare la review storica in `docs/context/analysis/archive/` se non più operativa |

**Done (storico):** 1, 2, 3(T3), 4, 6, 6a, 6b, 10bis, 10ter, 10quater, 0ter, 15, 16, 17a; OI-1, OI-2, OI-3, OI-12, OI-16 (misurato 2026-11-01, decisione C), step 14. **Sessione 2026-11-01:** OI-16 (rimozione scatter, non bug), 17a (analisi + state-lifecycle), 17b (merge first actions), 20, 21.

**Files touched this session (2026-11-01, 17a — E3c):** `docs/context/SESSION-STATE.md` (riformatto §F: rimosso "Stato lavoro" — contenuto in `git log`; compresso Dettagli; aggiunto step 17b; revisionato mapping OI-17) |
**Files touched this session (2026-11-01, review of 12-42 + archival):** `docs/context/analysis/2026-10-31-12-42-review.md` (amended: status header, table H and L, §3a H/L split, Verdict, footer; workflow note) |
**Files touched this session (2026-11-01, B1-bis lesson — E3c):** `docs/context/NEW-SESSION-PROMPT.md` (B1-bis: new bullet — `git add` with multiple paths fails entirely if one pathspec is mis-specified) |
**Files touched this session (2026-11-01, ADR-025 installation):** `docs/adr/025-tui-board-renderer.md` (NEW, `proposed`); `STRUCTURE.md` §5 (row 025); `specs/_index.md` (row 025); `docs/context/SESSION-STATE.md` (linked in todo) |
**Files touched this session (2026-11-01, closing — E2 "chiudi"):** `docs/context/SESSION-STATE.md` (Last commit verified → 44cf487, Done storico riscritto con la sessione 2026-11-01, this entry updated) |
