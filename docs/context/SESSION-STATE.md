# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. Update at the end of a session, not at the start.

**Last commit verified:** `7282c4f` — "docs(rules): B1-bis — a git add with multiple paths fails entirely on a missing pathspec"

**Note on the field above:** it points to the last commit *before* this
bookmark update (semantics S1). It will look "stale" compared to HEAD; that
is expected. Do not "fix" it by pointing to a newer commit without updating
the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md` — open and rejected ideas. Read it at session start to see what is not yet decided; do not implement from it.

<!-- Formato Piano: legenda in alto, tabella "Da fare" con emoji di priorità, dettagli sotto, prime azioni per step in tabella separata. Se cambi la forma, aggiorna questa nota nello stesso commit. Non riformattare senza motivo. -->

**Legenda:** ✅ done · 🔵 in corso · ⚫ bloccato · 🔴 urgente · 🟡 prossimo · ⬜ aperto

**Analysis reports (vivi):**

- `docs/context/analysis/2026-10-31-22-28-review.md` — L0 operational census; OI-17. Snapshot at `246f64b`; archive when L0 changes, do not update in place.

**Open items (what is not decided or verified yet):**

| # | Cosa | Perché aperto | Prima azione |
|---|------|---------------|--------------|
| OI-4 | **`README.md:148` cita `pcfm-gen`** come esistente, ma non è tra i 9 crate | Verificare se è nota intenzionale o stale | `grep -rn "pcfm-gen" --include="*.md" --include="*.rs" .` |
| OI-6 | **`docs/performance.md` e `docs/save-security.md`** mancanti | Mai creati, non tracciati altrove | Decidere se e quando crearli (probabilmente pre-release, insieme a step 13) |
| OI-7 | **Gap B: `ProjectionProfile` senza metodo di calcolo** | Mai tracciato altrove | Decidere il metodo quando si apre `L2-scouting` |
| OI-8 | **Gap C: `ScoutReport.DeclaredBias` senza formula** | Mai tracciato altrove | Decidere quando si apre `L2-scouting` |
| OI-13 | **Deferred a L1: finding dei 2 report non assegnati a step** (con `conforms_to v1.1` non-bug, riformulato 2026-10-31) | Nessuno è bloccante per L0; diventano step quando si apre L1 | Decidere: (a) promuovere un finding a step, oppure (b) dichiararli "target design, non current" in `specs/L0-match/00-overview.md` (una riga) e chiudere OI-13 |
| OI-14 | **Golden test 10 stagioni non esiste** (`CHANGELOG-SIM:42` lo richiede; `00-foundations` §4 lo elenca) | Manca scelta della maintainer: forma (i) L1 o (ii) N partite con rose fisse | Decidere (a) dichiararlo "planned, form TBD" in `CHANGELOG-SIM:42`; (b) creare ora; (c) attendere L1 |
| OI-15 | **A2 esegue `cargo test` una terza volta per sessione** (startup step 4, closing C2, closing gates A2) | Mai deciso: tenere come safety net o rimuovere? | Decidere: (a) saltare A2 in closing e dichiarare "A2 = closing step 1"; (b) tenerlo e dichiararlo safety net esplicito |
| OI-17 | **Censimento operativo L0: 12 divergenze spec↔code** (`analysis/2026-10-31-22-28-review.md`) | Fotografia; nessuna decisione su quali gap chiudere prima di L1/L2 | Leggere §5 "Verdict"; decidere quali gap (probabilmente nessuno prima di L1) |
| OI-18 | **Ottimizzazione dei rituali: R1–R6** (startup step 3 rilegge prompt; "unexpected observation" doppia in 2a/step 4; `cargo test` doppio = OI-15; startup step 1 ambiguo sotto S1; startup 2-bis esegue A1/A3 che non cambiano in sessione; step 4 duplica A1) | OI-16 misurato 2026-11-01: `Decisions:` usato (11/20), `Rules-Audit:` verde (10/10), `Deferred:` mai usato (0/20), ~1 commit su 2 tocca il bookmark. **Decisione C:** step 17 prima; semplificazione dei rituali dopo, con dati aggiornati | Step 17b come primo item; poi valutare R1–R6 con i dati OI-16 |
| OI-19 | **View model separation: enforcement compile-time deferred** (ADR-025) | ADR-022 §3 rende normativa la separazione, ma il prototipo in `pcfm-demo` la impone solo via un test strutturale manuale (`grep` sul modulo). D5: il fallback visibile è debole. Diventa enforcement reale solo quando esiste un secondo consumatore (crate boundary) | Quando il view model viene promosso a crate: valutare `pcfm-viewmodel` con dipendenze ristrette, o un test compile-fail; decidere in quell'ADR |

**Da fare (in dependency order):**

| Pr. | #   | Cosa                    | Dipende da                   |
|-----|-----|-------------------------|------------------------------|
| 🔵  | 12  | TUI demo (IDEA-INFRA-006): **board + events, not pitch** (scope 2026-10-31). ADR-025 (proposed) scritto e installato — **ratificare ADR-025**, poi scrivere `viewmodel.rs` + `board.rs` + `--board`. Il pitch con 22 giocatori richiede L0 posizionale (step 18). Leggere prima il censimento (OI-17) | ADR-025 ratificato |
| 🟡  | 2   | Fix ADR-003 (nota rename fatta `a391b85`; restano `demo.rs`, autorizzazione `pcfm-demo`) | — |
| 🟡  | 3   | Terminologia (T1+T2, T3 done) | — |
| 🟡  | 11  | GitHub agent Phase 2    | ruleset attivo               |
| 🟡  | 17b | **Fusione Prime azioni nel Piano**: aggiungere colonna/riga "first action" al Plan, rimuovere la tabella separata, aggiornare B1 in `scripts/gates.sh` e il testo closing Gate B in `NEW-SESSION-PROMPT.md`. Split di DP-03; richiede `Rules-Audit` | — |
| ⬜  | 5   | Review sketch ×3        | L2-tactics, input, L2-staff  |
| ⬜  | 7   | Verification cascade    | —                            |
| ⬜  | 8   | ADR-019 deferred 4-9    | —                            |
| ⬜  | 9   | Ratify ADR-017          | —                            |
| ⬜  | 10  | `L1-world/01-nation`    | —                            |
| ⬜  | 13  | `cargo-audit`/`cargo-deny` pre-release | — |
| ⬜  | 18  | **Ristrutturazione e riscrittura di L0 (posizionale con coordinate)** — non aprire ora: L2 non esiste (D3) | L2 (rating/scouting) come consumatore concreto |

**Dettagli (solo dove servono):**

**12. TUI demo (IDEA-INFRA-006)** — **scope (2026-10-31): board + events, not pitch.** Validate the split renderer/engine (ADR-022): a second renderer reads `MatchResult` without touching the engine. **ADR-025 (proposed, written 2026-11-01) covers this step:** view model in `src/viewmodel.rs` of `pcfm-demo`, board in `src/board.rs`, `--board` flag in `main.rs`; JSON serialisation; no engine change, no `SIM_VERSION` bump. Board content: score, possession, shots, goals, event list (today 4 types), narrative, state hash. The pitch with 22 players requires L0 positional (step 18), which requires L2 (D3). **First step: ratify ADR-025 (`Status: accepted`), then write `viewmodel.rs` + `board.rs` + `--board`; run tests.** Structural enforcement of the view model (no `pcfm-hidden`, no `MatchState`) is manual today, tracked as OI-19.

**2. Fix ADR-003** — ADR-003 still names `pcfm-debug` and `demo.rs`; `pcfm-demo` reads Plane A without authorization. ADR-023 exists (2026-10-31, observed cause traces) and does not cover ADR-003. Options: amend ADR-003, or extend ADR-020. Requires explicit confirmation to touch `docs/adr/`.

**3. Terminologia** — T1 (small): `PLUGIN_DEVELOPER_GUIDE.md` §1 + `GLOSSARY.md`. T2 (small): "core module" → "framework crate" in §2 and §11. T3 (large, one commit): rename `modules/` → `specs/` — **✅ done, commit `5e77d17`**.

**17b. Fusione Prime azioni nel Piano** — Split di DP-03. Il Plan prende una cella "first action" (colonna o riga subito sotto); la tabella "Prime azioni" sparisce; `scripts/gates.sh` B1 controlla la stessa cella nel Plan; il testo closing Gate B (check 1 e 4) aggiornato. Richiede D4 pre-mortem + `Rules-Audit` completo. Blast radius: 3 file (`SESSION-STATE.md`, `scripts/gates.sh`, `NEW-SESSION-PROMPT.md`).

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
| 12  | Ratificare ADR-025 (`docs/adr/025-tui-board-renderer.md`, `proposed`): leggere, decidere, `Status: accepted`. Poi aprire `crates/pcfm-demo/src/` e scrivere `viewmodel.rs` + `board.rs` + `--board` in `main.rs`. |
| 13  | Installare `cargo-audit` e `cargo-deny` (o verificarne la disponibilità) e lanciarli sul workspace |
| 17b | Aprire `scripts/gates.sh` blocco B1 (righe 67-74) e il testo closing Gate B in `NEW-SESSION-PROMPT.md`; decidere la forma della colonna "first action" nel Plan |

**Done (storico):** 1, 2, 3(T3), 4, 6, 6a, 6b, 10bis, 10ter, 10quater, 0ter, 15, 16, 17a; OI-1, OI-2, OI-3, OI-12, OI-16 (misurato 2026-11-01, decisione C), step 14. Report `12-42-review` amended (L reversed) + archived 2026-11-01. Storico dettagliato in `git log --oneline`. Sessione 2026-10-31: A2/E2 exact words, Decisions trailer, S1 on every bookmark update, OI-18, TUI scope, L0 positional (step 18). 105 test, SIM 4.

**Files touched this session (2026-11-01, 17a — E3c):** `docs/context/SESSION-STATE.md` (riformatto §F: rimosso "Stato lavoro" — contenuto in `git log`; compresso Dettagli; aggiunto step 17b; OI-16 chiuso; OI-18 riformulato; aggiunta sezione `Analysis reports (vivi)`; `Last commit verified` aligned to `d504e0e`; this entry).
**Files touched this session (2026-11-01, review of 12-42 + archival):** `docs/context/analysis/2026-10-31-12-42-review.md` (amended: status header, table H and L, §3a H/L split, Verdict, footer; then `git mv` to `analysis/archive/`); `docs/context/SESSION-STATE.md` (rimossa entry dalla sezione "Analysis reports (vivi)", aggiunto in `Done` l'entry di amend+archival, `Last commit verified` aligned to `0d605e0`, this entry).
**Files touched this session (2026-11-01, B1-bis lesson — E3c):** `docs/context/NEW-SESSION-PROMPT.md` (B1-bis: new bullet — `git add` with multiple paths fails entirely if one pathspec is missing; fallback: one path per `git add`, verify with `git status --short`; `Last updated` refreshed); `docs/context/SESSION-STATE.md` (this entry, `Last commit verified` aligned to `b1f6d12`).
**Files touched this session (2026-11-01, ADR-025 installation):** `docs/adr/025-tui-board-renderer.md` (NEW, `proposed`); `STRUCTURE.md` §5 (row 025); `specs/_index.md` (row 025); `docs/context/SESSION-STATE.md` (step 12 → 🔵, Prime azioni 12 → ratify ADR-025, Detail 12 rewritten, OI-19 added, `Last commit verified` → `7282c4f`, this entry).
