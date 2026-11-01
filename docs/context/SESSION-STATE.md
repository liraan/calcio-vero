# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. Update at the end of a session, not at the start.

**Last commit verified:** `ef49751` — "docs(ideas): IDEA-TACT-003 — foul severity parked with the positional engine"

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
- `docs/context/analysis/2026-11-01-L0-verification-checklist.md` — 13-item checklist (D1–D13) for L0 draft stability; grounds OI-13. Kept as reference (draft from `draft/layered-match-2026-11-01`, `4613797`).


**Archiviati il 2026-11-01:** `09-50` (superato da `10-25`, stesso audit v1.1→v1.2); `12-30` (protocollo per workstream che stanno per essere cancellati — premessa decaduta); `12-40` (piano preview con `--tui`/`--field`, richiede dati spaziali inesistenti; scope corretto in step 12).

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
| OI-22 | **Draft `draft/layered-match-2026-11-01` (ex `review-l0-branch`) non compila** (15 errori). **Decisione 2026-11-01: non riparare.** Idee estratte in `IDEAS.md` (INFRA-018, INFRA-019, TACT-002) + checklist; il commit `4613797` resta raggiungibile via tag | Il draft è codice mai compilato, su un'architettura che contraddice `00-foundations` §9. Ripararlo produrrebbe codice inerte: non esiste consumatore (D3, step 18 dipende da L2) | Chiudere OI-22 quando step 18 apre: al momento del bisogno, ripartire dal tag `draft/layered-match-2026-11-01` |
| OI-25 | **ADR-026 implementato (fasi 1+2, 2026-11-01).** `ActorId`, `Half`, loop a quattro fasi, punto fisso stoppage, `balance/stoppage_time.json`, boundary events, board `1T`/`2T`. SIM_VERSION 4→6 | Resta la verifica umana: guardare la demo, decidere se i contributi iniziali (goal 30", foul 30", duel 10") sono sensati, e se il board è leggibile. Poi OI-25 si chiude | `cargo run --release -p pcfm-demo -- --seed 42 --board` e `--seed 7`: guardare i boundary, il recupero, la coerenza con il modello a due tempi. Se ok, chiudere OI-25 |
**Da fare (in dependency order):**

| Pr. | #   | Cosa | Dipende da |
|-----|-----|------|------------|
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

**Organizzazione futura (decisione 2026-11-01):**


- **Branch per area di lavoro, non per livello architetturale.** Una feature attraversa i livelli (es. "fuorigioco" tocca engine + spatial + view): con branch di livello servono 3 PR e merge incrociati. Con branch di area (Engine / Data-world / Design-spec / Presentation) la feature tocca 1 area principale e al massimo 1 secondaria.
- **I livelli si esprimono nei crate, non nei branch.** Un branch è un albero di file, non un grafo di dipendenze: non impedisce a `pcfm-presentation` di importare `pcfm-hidden`. Un crate sì.
- **Prerequisito del parallelismo: CI + branch protection (OI-20).** Senza un gate remoto, i branch divergono e nessuno se ne accorge: è esattamente quello che è successo con il draft spaziale (15 errori, mai intercettati). Ordine: CI → consolidamento su `main` → branch per area, quando ci sono le persone.
- **Prova empirica:** i tre `workstream/*` sono vuoti (OI-24) perché non c'era lavoro da metterci che non fosse anche di un altro livello. La struttura ha preceduto il contenuto (D3).


**Board testuale e step 18 (decisione 2026-11-01):** il board di ADR-025 non è la base della demo posizionale. Con step 18 (motore posizionale) gli eventi diventano tracce di uno stato continuo, non una lista: il board testuale **scade**, non si estende. La demo futura richiede riepilogo + vista spaziale + dimensione temporale, e la TUI sarà il suo veicolo naturale (un board testuale non può mostrare un campo). **Non investire in abbellimenti del board** (colori, layout, colonne in previsione): sopravvive solo la validazione dello split renderer/engine, non la forma. Step 18 resta lontano: prerequisito L2, che non esiste (D3).

**Dettagli (solo dove servono):**


**2. Fix ADR-003** — ADR-003 allows Plane A access only to `pcfm-match` and `pcfm-gen`. `pcfm-demo` violates this: `main.rs:20` imports `pcfm_hidden::GroundTruth`, `main.rs:115` and `:170` construct it and pass it to `MatchContext`. It does **not** call `resolve_*` or read latent fields, but the type is in scope in a crate that must not see Plane A. The board (`viewmodel.rs`, `board.rs`) is clean: it reads `MatchResult` only. Options: (a) move the `GroundTruth` construction into a `pcfm-fixtures` helper that returns a ready `MatchContext`; (b) amend ADR-003 to allow `pcfm-demo` as a test harness (weaker, and it widens the boundary). Recommendation: (a). Requires explicit confirmation to touch `docs/adr/`.

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
| 13  | Installare `cargo-audit` e `cargo-deny` (o verificarne la disponibilità) e lanciarli sul workspace |
| 17b | Aprire `scripts/gates.sh` blocco B1 (righe 67-74) e il testo closing Gate B in `NEW-SESSION-PROMPT.md`; decidere la forma della colonna "first action" nel Plan |
| 20  | Creare `.github/workflows/ci.yml`, documentare `git config core.hooksPath .githooks`, e impostare required checks in GitHub |
| 21  | Archiviare la review storica in `docs/context/analysis/archive/` se non più operativa |

**Done (storico):** 1, 2, 3(T3), 4, 6, 6a, 6b, 10bis, 10ter, 10quater, 0ter, 15, 16, 17a; OI-1, OI-2, OI-3, OI-12, OI-16 (misurato 2026-11-01, decisione C), step 14. **Sessione 2026-11-01 (closing `0f86a1b`):** OI-16 misurato, 17a riformatto §F (218→82 righe), B1-bis lesson (`git add` multi-path), ADR-025 `proposed` installato, OI-19 (enforcement view model). **2026-11-01 (consolidamento):** 3 workstream vuoti cancellati (OI-24), `WORKSTREAM-ONBOARDING.md` cancellato, `review-l0-branch` → tag `draft/layered-match-2026-11-01`. **2026-11-01 (estrazione):** OI-22 deciso (non riparare); OI-23 chiuso; 3 idee estratte (INFRA-018, INFRA-019, TACT-002); `L0-verification-checklist.md` copiata. **2026-11-01 (step 12 — demo interna):** ADR-025 ratificato (`accepted`); `viewmodel.rs` (MatchViewModel + test strutturale via `include_str!`), `board.rs` (`render_board` + `print_board`), `--board` in `main.rs` (mutuamente esclusivo con `--json`/`--html`, `parse_args_from` testabile), 12 test nuovi, C2 verde (117 test), demo deterministica (`--seed 42 --board`).

**Files touched this session (2026-11-01, 17a — E3c):** `docs/context/SESSION-STATE.md` (riformatto §F: rimosso "Stato lavoro" — contenuto in `git log`; compresso Dettagli; aggiunto step 17b; revisionato mapping OI-17) |
**Files touched this session (2026-11-01, review of 12-42 + archival):** `docs/context/analysis/2026-10-31-12-42-review.md` (amended: status header, table H and L, §3a H/L split, Verdict, footer; workflow note) |
**Files touched this session (2026-11-01, B1-bis lesson — E3c):** `docs/context/NEW-SESSION-PROMPT.md` (B1-bis: new bullet — `git add` with multiple paths fails entirely if one pathspec is mis-specified) |
**Files touched this session (2026-11-01, ADR-025 installation):** `docs/adr/025-tui-board-renderer.md` (NEW, `proposed`); `STRUCTURE.md` §5 (row 025); `specs/_index.md` (row 025); `docs/context/SESSION-STATE.md` (linked in todo) |
**Files touched this session (2026-11-01, closing — E2 "chiudi"):** `docs/context/SESSION-STATE.md` (Last commit verified → 44cf487, Done storico riscritto con la sessione 2026-11-01, this entry updated) |
**Files touched this session (2026-11-01, repair — E3c):** `docs/context/SESSION-STATE.md` (LCV `44cf487`→`a5a070b` — era sbagliato a monte; step 12 `render.rs`→`--board` in Plan e Prime azioni; Done 2026-11-01 corretto — 17b/20/21 non fatti; sezione "Archiviati il 2026-11-01"; OI-22/23/24 aggiunti; blocco "Organizzazione futura"; this entry); `docs/context/analysis/2026-11-01-09-50-review.md` → `archive/` (superato da `10-25`); `2026-11-01-12-30-branch-test-matrix.md` → `archive/` (premessa decaduta); `2026-11-01-12-40-demo-preview-release.md` → `archive/` (richiede dati spaziali inesistenti).
**Files touched this session (2026-11-01, consolidation — E3c):** `docs/context/WORKSTREAM-ONBOARDING.md` (deleted — third L0-L3 numbering, contradicted `00-foundations` §9); 3 remote branches `workstream/*` deleted (empty, pointed at `9750cde`); `docs/context/SESSION-STATE.md` (OI-24 closed, OI-23 restricted, Done updated, LCV → `a11d5cd`, this entry).
**Files touched this session (2026-11-01, extraction from review-l0-branch — E3c):** `docs/context/IDEAS.md` (3 nuove idee: INFRA-018 tick snapshot, INFRA-019 spatial primitives + duplicate BallState, TACT-002 tactical types); `docs/context/analysis/2026-11-01-L0-verification-checklist.md` (NEW — copiata da `review-l0-branch` `4613797`, 13 item D1–D13); `docs/context/SESSION-STATE.md` (OI-22 deciso non riparare, OI-23 chiuso, Done aggiornato, checklist citata in "Analysis reports (vivi)", LCV → `cd7d5e2`, this entry).
**Files touched this session (2026-11-01, tag+delete del draft — E3c):** tag `draft/layered-match-2026-11-01` → `4613797` (creato e pushato); branch remoto `review-l0-branch` cancellato; `docs/context/IDEAS.md` (riferimenti aggiornati al tag); `docs/context/SESSION-STATE.md` (OI-22 riscritto con azione "ripartire dal tag"; checklist cita il tag; "Organizzazione futura" senza nome branch; LCV → `fe7ceeb`; this entry).
**Files touched this session (2026-11-01, ADR-025 ratification — E3c):** `docs/adr/025-tui-board-renderer.md` (`Status: proposed` → `accepted`); `STRUCTURE.md` §5 (row 025 🟡→✅); `specs/_index.md` (row 025 🟡→✅); `docs/context/SESSION-STATE.md` (step 12 plan + Prime azioni + Detail: ADR-025 accepted, demo interna, LCV → `6c7dc60`, this entry).
**Files touched this session (2026-11-01, step 12 — board demo — E3c):** `docs/adr/025-tui-board-renderer.md` (`proposed` → `accepted`); `STRUCTURE.md` §5, `specs/_index.md` (row 025 ✅); `crates/pcfm-demo/Cargo.toml` (`serde` added); `crates/pcfm-demo/src/viewmodel.rs` (NEW — `MatchViewModel`, `EventRow`, `NarrativeRow`, exhaustive `EventType`→str, structural test); `crates/pcfm-demo/src/board.rs` (NEW — `render_board` + `print_board`, 4 test); `crates/pcfm-demo/src/main.rs` (`--board`, `parse_args_from` testabile, 5 CLI test); `README.md` (badge + 2 righe: 105→117); `Cargo.lock`; `docs/context/SESSION-STATE.md` (step 12 chiuso, LCV → `24174e2`, this entry).
**Files touched this session (2026-11-01, board improvements — E3c):** `crates/pcfm-demo/src/viewmodel.rs` (`Team` enum, `EventRow` with `team`+`number` instead of opaque slot, `is_goal_shot` filter on events only, narrative unfiltered, `try_from` casts, `team_label` const); `crates/pcfm-demo/src/board.rs` (events render `Home 9`/`Away 9`, `GOAL` uppercase, test updated); `README.md` (118 tests); `docs/context/SESSION-STATE.md` (LCV → `b631144`, this entry).
**Files touched this session (2026-11-01, ADR-026 install — E3c):** `docs/adr/026-match-boundaries-stoppage-time.md` (NEW, `proposed`, 145 righe, zero open question); `STRUCTURE.md` §5 (row 026 🟡 proposed); `specs/_index.md` (row 026 🟡 proposed); `docs/context/SESSION-STATE.md` (OI-25 added — ADR-026 ratify+implement; nota "Board testuale e step 18"; LCV → `83c293f`; this entry).
**Files touched this session (2026-11-01, ADR-026 ratification — E3c):** `docs/adr/026-match-boundaries-stoppage-time.md` (`proposed` → `accepted`); `STRUCTURE.md` §5 (row 026 ✅); `specs/_index.md` (row 026 ✅); `docs/context/SESSION-STATE.md` (OI-25: ratify+implement → implement only; LCV → `f195510`; this entry).
**Files touched this session (2026-11-01, ADR-026 phase 2 — E3c):** `crates/pcfm-stats/src/event.rs` (`Half` enum, `MatchEvent.half`); `crates/pcfm-match/src/state.rs` (`half`/`half_start_tick` fields, `begin_half`, `emit_boundary`, timestamp relativo al tempo); `crates/pcfm-match/src/balance.rs` (NEW — `StoppageTime`, `include_str!` del file JSON, `stoppage_for`); `balance/stoppage_time.json` (NEW); `crates/pcfm-match/src/orchestrator.rs` (quattro fasi, punto fisso, guard 45'); `crates/pcfm-demo/src/viewmodel.rs` (`EventRow.half`); `crates/pcfm-demo/src/board.rs` (`format_time`, `1T`/`2T`, `45'+X'`); `crates/pcfm-match/src/save.rs` (`SIM_VERSION` 6); `CHANGELOG-SIM.md` (entry 6, Current); `README.md` (122 test); `docs/context/SESSION-STATE.md` (OI-25 aggiornato, LCV → `efd7f14`, this entry).
**Files touched this session (2026-11-01, ADR-026 calibration — E3c):** `balance/stoppage_time.json` (goal 60, foul 20, duel 10, base 0); `crates/pcfm-match/src/balance.rs` (test aggiornati); `crates/pcfm-match/src/save.rs` (`SIM_VERSION` 7); `CHANGELOG-SIM.md` (entry 7); `docs/context/SESSION-STATE.md` (LCV → `c7ced20`, this entry).
**Files touched this session (2026-11-01, IDEA-TACT-003 — E3c):** `docs/context/IDEAS.md` (IDEA-TACT-003: foul severity parked with step 18); `docs/context/SESSION-STATE.md` (LCV → `fc902c2`, this entry).
**Files touched this session (2026-11-01, nota recupero — E3c):** `docs/context/IDEAS.md` (IDEA-TACT-003: nota "Known consequence", il recupero alto è il motore a 4 eventi, non la formula; niente moltiplicatore per tempo); `docs/context/SESSION-STATE.md` (LCV → `ef49751`, this entry).
