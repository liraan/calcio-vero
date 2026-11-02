# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. History is in `git log`; this file does not duplicate it.

**Last commit verified:** `96317c2` — "feat(balance): influence weights loader (C.1 step 1)"

**Note on the field above:** it points to the last commit *before* this bookmark update (S1). It will look "stale" compared to HEAD; expected. Do not "fix" it without updating the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md`. Read at startup to know what is not yet decided; do not implement from it.

**Legenda:** [wip] in corso · [next] prossimo · [open] aperto · [urgent] urgente

**Analysis reports (vivi):** `docs/context/analysis/2026-11-01-L0-verification-checklist.md` (13 item; G2/G10 ora OI-23/24; G1/OI-22 chiuso in slice B2) e `docs/context/analysis/2026-11-02-engine-gaps.md` (fatti misurati + decisioni 2026-11-02, da annotare sui contratti). Archiviati (2026-11-01): `09-50`, `12-30`, `12-40`, `10-25`, `22-28` (censimento invecchiato: L0 cambiato 3 volte il 2026-11-01, i G ancora veri sono OI individuali).

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
| OI-28 | **Contraddizione §3.4 vs §6-ter della nota L0.** §3.4 dice "movement is part of slice C"; §6-ter/§7/Plan elencano C = Slip+ControlError+Pressure, senza movimento. **Aggravato il 2026-11-02:** la decisione palla (posizione intermedia in `InFlight`) richiede movimento, quindi §3.4 non è più un dettaglio: è la condizione della slice. Da chiudere **prima** della slice palla | Decidere quale delle due fonti vale (raccomandazione: §3.4 aggiornata, movimento nella positional slice), aggiornare l'altra nello stesso commit |
| OI-29 | **`AgentState.velocity` è `(0, 0)` hardcoded e mai aggiornata** (`spatial.rs:76,87`). `Slip` (sezione §6-ter) la usa come input: oggi leggerebbe zero costante | Decidere: modello di movimento, riformulare `Slip`, o rimandarlo (dipende da OI-25) |
| OI-30 | **`balance/` contiene solo `stoppage_time.json`.** `pitch_wear` e `weather` (citati per slice C) non esistono come parametri | Creare i JSON quando il modello li consuma, non prima (D3) |
| OI-31 | **Drift spec↔codice su `EventType`.** `05-match-events.md` §2.2 non elenca `KickOff`/`SecondHalf`; il codice li ha (`pcfm-stats/src/event.rs`). La spec dice anche "The set is closed at L0" ma `Slip`/`ControlError` non ci sono | Decidere se allineare lo spec al codice, o viceversa, e se aggiungere `Slip`/`ControlError` richiede un ADR |
| OI-32 | **`crates/pcfm-match/src/narrative.rs.tmp` orfano** su disco (non tracciato, non in `git status`: probabile `.gitignore`). Residuo di una sed/awk precedente | Verificare con `git ls-files` e `git check-ignore`, poi rimuovere |
| OI-33 | **Il motif è a stadi (L0 = causalità locale, L2 = causale globale).** Il pillar 5 ("perché ho perso?") non è rispondibile a L0 senza tattiche e psicologia. Va dichiarato esplicitamente nel README/manifesto, o il prodotto promette più di quanto L0 possa dare | Decidere se dichiarare la promessa a stadi in `README.md` o in un ADR |
| OI-34 | **Tensione D2 nel codice esistente.** `state.rs:320` (`expect("spatial state contains every starter")`) e `state.rs:325` (`unwrap_or(max)`) sono in path di simulazione. D2 dice "no `unwrap`/`expect`/`panic!` in simulation paths". Documentati unreachable, ma letteralmente in violazione | Valutare sostituzione con `Option` + gestione esplicita, in una sessione dedicata (non ora: non blocca slice C) |
| OI-35 | **README — tabelle superate da sostituire.** Level/Code (riga ~89), Axis/Status (~100), Phase/Target (~113), FM vs PCFM (~124) hanno numeri e stato che invecchiano. La nuova sezione "The match engine in one glance" è un primo passo; le vecchie tabelle vanno riscritte o rimosse quando il motore L0 è completo | Riscrivere le tre tabelle con numeri e stato reali (o rimuoverle se ridondanti) quando il debito ADR-019 è chiuso |
| OI-36 | **ADR-025 cita "step 18" (vecchia numerazione).** Il Plan è ora un albero (T/C/C.1/C.2), "L0 posizionale" si chiama positional slice. ADR è record storico, non si modifica; ma il lettore che lo apre trova un riferimento che non esiste più. README e IDEAS corretti (2026-11-02) | Aggiungere una nota glossario o un emendamento leggero ad ADR-025 quando lo si tocca per altro |
| OI-37 | **Palla: sequenza invertita — per-agent prima (2026-11-02).** Decisione: **no all'intercettazione statica** (la palla richiede intercettazione dinamica → per-agent). L'intercettazione va scritta come **query spaziale** (chi è più vicino alla traiettoria tra le posizioni correnti), non come caso del defender scelto. **Sequenza:** misura per-agent (fatta: 18 ms, limite inferiore) → per-agent/movimento → palla → 2D. Il 2D debug arriva **dopo** il movimento. **RNG:** con l'influenza non c'è roll per la selezione (dissolto). **Aperti:** (d) `Pitch.condition` riservato (chiuso: nota in `03` e `WORLD-MODEL`); (e) tick record derivato (replay rigenera) + live streaming da progettare; (f) 2D = consumatore di tick record. **Nota (2026-11-02):** `carrier` è **solo `Possessed`**; `InFlight` non è un carrier speciale (oggi `carrier = None` significa sia "libera" sia "in volo"); `Possession` (Home/Away) è una **proiezione** di `carrier`, non stato — il `flip_possession()` manuale va sostituito quando `ball_state` esiste. **Prerequisito:** annotare i 5 contratti | Prima azione: annotare i 5 contratti; poi progettare il live streaming; poi aprire il piano di slice (`Note-Read`, `SIM_VERSION` bump) |
| OI-38 | **Ruleset: tre decisioni differite (2026-11-02).** (1) Il ruleset è per-partita o per-stagione? (2) Chi lo sceglie: la configurazione a L0, o la competizione a L1? (3) I recuperi sono opachi (come in una partita vera, nessuno sa perché 4 minuti e non 3) o trasparenti (il motore spiega il calcolo, pillar 5)? Il ruleset è un id versionato nel save (conseguenza di ADR-013); un cambio di regole post-release costa un modulo, non un motore. `resolve_stoppage` core, plugin solo per la taratura | Decidere (1) e (2) quando la prima regola (offside, ADR-019 D11) viene implementata; (3) quando il referee model esiste. Nessuna azione ora |
| OI-39 | **Stream RNG unico per tutti i `resolve_*` (`hidden:{id}:resolve`).** Aggiungere un resolve nuovo sposta gli esiti di **tutti** i resolve successivi in ogni partita: non si può isolare l'effetto di una modifica. Con influence, stoppage, rules nel tick loop, il costo composto diventa strutturale. Verificato 2026-11-02: il motore non usa `event_bus` e non chiama `drain_observations`. | Verificare chi consuma lo stream `hidden` oggi; decidere stream per scopo (`:duel`, `:pass`, `:influence`) vs unico. Alta priorità |
| OI-40 | **`event_bus` esiste, il motore non lo usa; `observations` accumulate e mai drenate.** Il primo subscriber del bus è già deciso nel contratto: la fase 2 delle regole (ADR-019 D11, attempt → outcome). `drain_observations` è chiamato solo dai test (`observation.rs` lo diceva, corretto 2026-11-02). Con le regole, o si usa il bus o le regole finiscono hardcoded nel tick loop | Decidere quando le regole arrivano: il bus è il loro meccanismo. Per `drain_observations`, decidere se è un canale Plane A→B o un residuo (IDEA-INFRA-015) |

## Plan (tree: leaves are executable steps, forks expose a decision; see IDEA-DESIGN-001)

| Pr. | # | Cosa | Dipende da | Prima azione |
|---|---|---|---|---|
| [open] | 11 | **GitHub agent Phase 2 — deferred (2026-11-02).** Ruleset **disabled**. Reactivate when one holds: (a) a second reviewer exists; (b) the project ships; (c) a write token is granted to an agent. `Evaluate` is not available (personal repo, non-Enterprise). Until then, Phase 1: direct push by the maintainer. See `AGENT-POLICY.md` §6 | — | Rileggere `AGENT-POLICY.md` §6 quando una delle tre condizioni si presenta |
| [open] | 5 | Review sketch ×3 | L2-tactics, input, L2-staff | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| [open] | 7 | Verification cascade | — | Decidere da quale file parte la cascade (candidato: `README.md`) |
| [open] | 8 | **ADR-019 deferred 4-9: verificati 2026-11-01, nessuno chiudibile.** 4 (UX) e 5 (resolution per piattaforma) richiedono UI/mobile reali; 6 (adaptive) richiede un ADR dedicato; 7 (LOD override) un emendamento ad ADR-013; 8 (zona di influenza) `docs/calibration/` vuota; 9 (role→tactic) `L2-tactics` non esiste. Non è un rinvio: è D3, non c'è contenuto concreto | — | Riaprire quando la dipendenza esiste. Nessuna azione ora |
| [open] | 10 | `L1-world/01-nation` | — | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| [done] | 13 | **Sicurezza dipendenze**: Dependabot alerts **attivato dal maintainer il 2026-11-01** (non era attivo di default, contrariamente a quanto avevo assunto). `cargo-deny` (licenze, ban) resta pre-release. | — | Chiuso. Se in futuro serve `cargo-audit` in CI, è un OI nuovo |
| [done] | A | **Slice A — fondamenta spaziali**: `spatial.rs` (`AgentState`, `SpatialState`, griglia 4-4-2, 6 test). Fatto `39c444b` | — | Fatto |
| [done] | B1 | **Slice B parte 1 — percezione + carrier**: `perception.rs` (4 test), `SpatialState.carrier`. Fatto `c231c98`. Non usati dal motore | A | Fatto |
| [done] | B2 | **Slice B2 — chi agisce dallo spazio + conquista come evento.** `MatchState` possiede `SpatialState`; `pick_players` → `pick_from_space`; duello vinto dal difensore emette `Tackle` con `from`/`to`; pass incompleto/fallo azzerano il carrier; `SIM_VERSION` 8, golden rigenerato, OI-22. Fatto. 133 test | B1 | Fatto. Vedi slice C |
| [done] | T | **Step 19 — la narrativa legge i `fields` (nodo trasversale, vale in ogni strada).** `narrate_event` legge `fields`: `Tackle` con `from`/`to`, `Shot` con `goal`, `Duel` con `winner_slot`. Test sul `Tackle`. Non tocca i sei file del motore (niente `Note-Read`), non cambia `SIM_VERSION` | — | Leggere `narrative.rs` (`render_text`, `narrate_event`) e `05-match-events.md` §2.1 |
| [open] | C | **Slice C — primi eventi fenomenologici.** `Slip` e `ControlError` da `pitch_wear` × `weather` (parametri in `balance/`, arbitrari dichiarati); `Pressure` emesso. Direzione relazionale (IDEA-DESIGN-001). Sotto-fork A/B/C/D tracciato in OI-25/29/31 | B2 | Leggere §0, §6-ter, §7 (trailer `Note-Read`), poi aprire `balance/` |
| [next] | C.1 | **Pressure con input relazionale (primo passo verificabile).** Aggiungere `ConditionContext { player, nearest_opponent_distance }` in `pcfm-hidden`; `resolve_pressure` lo legge; test meccanico: due distanze diverse devono produrre distribuzioni diverse. Se non cambia, è ancora un dado. **Priorità non confermata (2026-11-02):** la palla (OI-37) potrebbe venire prima; `[next]` è il default finché OI-37(a) non è deciso | T (narrativa legge fields), direzione C | Leggere `pcfm-hidden/src/context.rs` e `lib.rs::resolve_pressure`, poi scrivere il test |
| [open] | C.2 | **Slip/ControlError riformulati (dopo C.1).** Non più con velocity hardcoded: input osservabile alternativo, o sessione dedicata a OI-25 (modello di movimento) | C.1 | Decidere: input alternativo a velocity, o aprire OI-25 |
## Decisioni operative (2026-11-01)

- **Finding (2026-11-01, closing):** the DP archival check is no longer
  executable. `DPs-Applied` became optional with the ritual cut, so the
  `git log` check cannot tell which DPs were used. Not urgent (3 active,
  max 10). If the DPs grow, reintroduce the trailer or an alternative
  check.

- **Organizzazione futura:** branch per **area di lavoro**, non per livello architetturale. I livelli si esprimono nei crate, non nei branch. Prerequisito del parallelismo: CI + branch protection (OI-20).
- **Board testuale e step 18:** il board di ADR-025 **non è la base** della demo posizionale. Con step 18 gli eventi diventano tracce di uno stato continuo; il board testuale scade, non si estende. La TUI sarà il veicolo del posizionale. **Non investire in abbellimenti del board.**


- **Fork non nel Plan (2026-11-02):** i fork A/B/C/D di slice C vivono in `IDEA-DESIGN-001` e OI-25/29/31, non nel Plan. Motivo: il Plan esegue (foglie con Prima azione); i fork espongono una decisione, e duplicarli nel Plan creerebbe un terzo stato (`[fork]`) che non è esecuzione, più la duplicazione che abbiamo appena eliminato rimuovendo `DECISION-MAP.md`. Il Plan ha C.1 che chiude il fork col test meccanico. Riaprire solo se i fork diventano 3+ e il Plan diventa illeggibile senza vederli.

- **Presenza fisica = budget di causalità (2026-11-02).** Un oggetto ha corpo nel motore solo se il suo corpo può essere causa di un evento che il giocatore deve poter capire. Tutto il resto è costo di modellazione che non produce spiegazione. Conseguenze: i 22 giocatori hanno presenza fisica (duelli, copertura); la palla ha **stato** posizionale, non corpo (è la posizione che conta, non la collisione); l'**arbitro non è stato del motore**: la sua posizione, se mostrata, è calcolata dal renderer in modo deterministico dagli eventi (layer visivo). Il fischio è normativo, **non dipende dalla posizione**: semplificazione dichiarata. ADR-019 D9 ("in-pitch, with Position and functional state") è interpretato come posizione presentazionale, non stato simulato; il comportamento (positioning) resta deferito al 2D come D9 già dice. Questo criterio rende coerenti la tesi sull'arbitro, la propensione sulla palla (OI-37) e il rifiuto della fisica in §0.

- **Palla — decisioni 2026-11-02 (dettaglio in `analysis/2026-11-02-engine-gaps.md` §4).** (a) Posizione intermedia in `InFlight` (non teletrasporto): stato persistente, salvato, nell'hash. (b) Porta = dato statico del campo; pali = ostacoli statici; rimbalzo con **angolo calcolato** (geometria dichiarata, non forze); `WORLD-MODEL` va annotato e "physical ball dynamics" ridefinito come "calcolato da forze". (c) `Pitch.condition` è un guscio (nessun consumatore) o riservato: da risolvere con `03-entities-minimal.md` §2.5. (d) Qualità del possesso = **campo dell'evento**, non stato persistente (a L0 il consumatore è la narrativa); gli **eventi** sono la base condivisa, narrativa e 2D sono proiezioni parallele.

- **Pass e Interception (2026-11-02).** `PassContext { passer, candidates: Vec<(PlayerId, freedom)> }`, `PassOutcome { receiver, complete }`: il ricevente è scelto da `pcfm-hidden` tra i candidati con la loro libertà (passatore primario, ricevente secondario). `Interception`: **terza via** — un resolve, due eventi; il difensore è il più vicino alla traiettoria (osservabile). Emettere `Pass` triplica il log (~8 → ~25 eventi). L'emissione richiede la palla in volo.

- **2D = debug TUI, non presentazione (2026-11-02).** ASCII top-down, una seconda vista TUI accanto al board (non un'alternativa). ADR-022 non è superseded: nota datata. ADR-025 va aggiornato nella motivazione (il pitch richiede il motore posizionale, non L2): il 2D di debug **è** il consumatore concreto che sblocca D3 su OI-25. Criterio di accettazione meccanico: le posizioni cambiano a ogni frame (diff tra frame). Non copre le regole.

- **Regole = rule module L0, non L3 plugin (2026-11-02).** ADR-019 D11 (fase 1 attempt → fase 2 outcome, sincrona). `IDEA-RULE-001` (discussing) è il contenitore: la discussione di oggi ne è il contenuto. Il **ruleset è un id versionato nel save** (conseguenza di ADR-013): un cambio di regole post-release costa un modulo, non un motore; le partite vecchie restano riproducibili. Golden hash per ruleset. `resolve_stoppage` core, plugin solo per la taratura. Da decidere: ruleset per-partita o per-stagione; chi lo sceglie (config a L0, competizione a L1).

- **2D — progressione e confine (2026-11-02).** La sequenza è: **2D debug (test) → 2D user (gaming) → espansioni (3D stadio, tifosi, ecc.)**. Il principio che la rende estendibile non è il test in sé, è il **confine**: nessun renderer parla con il motore (`MatchState`); tutti consumano i **tick record** (IDEA-INFRA-018). Il 2D debug è il primo consumatore; il 2D user cambia il rendering, non la sorgente; le espansioni sono consumatori ulteriori dello stesso substrato. Dove vive il 2D debug (`pcfm-test-runner --frames` o un crate dedicato) è conseguenza di questo confine, non una decisione a sé. **Aggiornato (pausa 2026-11-02):** il 2D debug arriva **dopo** il movimento/per-agent, non prima. Mostrare un 2D tra statue sarebbe mostrare un motore che non vogliamo (l'intercettazione statica è stata rifiutata). Criterio di accettazione del 2D debug: i frame cambiano a ogni tick (diff tra frame), non "sembra calcio". Misura 2026-11-02: una partita si rigenera in ~0,2 s (release, seed 42), quindi il replay rigenera e non salva; il live richiede streaming dal motore.

- **Pressure = evento che accompagna l'azione, step verso l'esito (2026-11-02).** `Pressure` è emesso **quando un'azione avviene** (Pass, Shot, Duel), non a ogni tick e non come evento isolato. È uno **step intermedio dichiarato** verso (c): quando `ControlError` esisterà, `Pressure` si sposterà sull'esito (la pressione che *produce* la perdita), senza riscrivere il calcolo. Il calcolo è `resolve_influence` sul punto della palla: chi pressa è il più influente tra gli avversari; il rango è l'output (ordine), non uno scalare. Se serve un'intensità, è una categoria discreta dichiarata su soglia, mai un numero in Plane B. Il modello (a) — ogni tick sopra soglia — è **rifiutato**: eventi che nessuno riconosce come calcio. Il modello (c) — solo sull'esito — non è eseguibile oggi (D3: `ControlError` non esiste). Costo: una `resolve_influence` per azione (~25 per partita), non per tick.
