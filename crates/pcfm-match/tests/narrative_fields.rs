//! T (2026-11-02): the narrative reads the event fields.
//!
//! The structured substrate (`*_slot`, `goal`) is populated from
//! `MatchEvent.fields`; the rendered text uses it. A missing field falls
//! back to the old text. An old save (JSON without the new fields)
//! deserializes with `None` (no `SCHEMA_VERSION` bump).

use pcfm_core::ids::{ActorId, EventId, MatchId, PlayerId};
use pcfm_core::time::GameInstant;
use pcfm_fixtures::match_fixture;
use pcfm_match::narrate;
use pcfm_stats::{EventChain, EventType, Half, MatchEvent};

fn ev(actor_slot: Option<u32>, event_type: EventType, fields: serde_json::Value) -> MatchEvent {
    MatchEvent {
        event_id: EventId::new(1, 0),
        match_id: MatchId::new(1, 0),
        actor: actor_slot.map(|s| ActorId::Player(PlayerId::new(s, 0))),
        half: Half::First,
        timestamp_minute: 10,
        timestamp_second: 0,
        minutes_played_cumulative: 10,
        event_type,
        fields,
        created_at: GameInstant::from_raw(10),
    }
}

#[test]
fn tackle_has_structured_slots_and_text() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    // Defender (away, slot 105) tackles the attacker (home, slot 5).
    chain
        .try_append(ev(
            Some(105),
            EventType::Tackle,
            serde_json::json!({ "from": 5, "to": 105 }),
        ))
        .expect("append");
    let entries = narrate(&m, &chain);
    assert_eq!(entries.len(), 1);
    let e = &entries[0];
    assert_eq!(e.actor_slot, Some(105));
    assert_eq!(e.from_slot, Some(5));
    assert_eq!(e.to_slot, Some(105));
    assert!(e.text.contains("Home 5"), "text: {}", e.text);
}

#[test]
fn shot_wide_when_goal_false() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    chain
        .try_append(ev(
            Some(5),
            EventType::Shot,
            serde_json::json!({ "goal": false }),
        ))
        .expect("append");
    let e = &narrate(&m, &chain)[0];
    assert_eq!(e.goal, Some(false));
    assert!(e.text.contains("wide"), "text: {}", e.text);
}

#[test]
fn duel_lost_when_winner_is_not_actor() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    // Actor is home 5; winner is home 7. The actor lost.
    chain
        .try_append(ev(
            Some(5),
            EventType::Duel,
            serde_json::json!({ "winner_slot": 7 }),
        ))
        .expect("append");
    let e = &narrate(&m, &chain)[0];
    assert_eq!(e.winner_slot, Some(7));
    assert!(e.text.contains("loses"), "text: {}", e.text);
}

#[test]
fn missing_fields_falls_back() {
    let m = match_fixture(42).expect("fixture");
    let mut chain = EventChain::new();
    chain
        .try_append(ev(Some(5), EventType::Tackle, serde_json::json!({})))
        .expect("append");
    let e = &narrate(&m, &chain)[0];
    assert_eq!(e.from_slot, None);
    assert_eq!(e.to_slot, None);
    assert!(e.text.contains("tackles in the"), "text: {}", e.text);
}

#[test]
fn old_json_without_new_fields_deserializes() {
    // Simulates a save written before this commit: no `actor_slot`,
    // `from_slot`, `to_slot`, `winner_slot`, `goal`.
    let json = serde_json::json!({
        "minute": 7,
        "event_id": { "slot": 1, "generation": 0 },
        "text": "GOAL!",
        "importance": "Critical"
    });
    let e: pcfm_match::NarrativeEntry = serde_json::from_value(json).expect("old save");
    assert_eq!(e.actor_slot, None);
    assert_eq!(e.goal, None);
}
