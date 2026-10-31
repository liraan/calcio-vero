//! Narrative — turning events into a story.
//!
//! # Purpose
//!
//! Convert a `MatchResult`'s events into a list of `NarrativeEntry`.
//! Each entry is a **data** struct, not a rendered string:
//!
//! - `minute`, `event_id`, `template_key`, `params`, `importance` are
//!   structured data.
//! - `text` is a **default-language rendering** (English), ready for
//!   display. Localised renderings are produced by looking up
//!   `template_key` in a locale file and substituting `params`.
//!
//! This makes the narrative usable by any renderer:
//!
//! - `TextMatchEngine` uses `text` directly (radio commentary).
//! - A `ThreeDEngine` can use `text` as subtitles, or use
//!   `template_key + params` to render its own localised text.
//! - A debug tool can display `template_key + params` directly.
//!
//! The narrative is **deterministic**: the same events produce the same
//! entries. No randomness, no time, no external state.

use std::collections::BTreeMap;

use pcfm_core::ids::{EventId, PlayerId};
use pcfm_model::Match;
use pcfm_stats::{EventChain, EventType, MatchEvent};
use serde::{Deserialize, Serialize};

/// How important a narrative entry is.
///
/// Renderers use this to decide how prominently to display the entry
/// (e.g. bold for `High`, normal for `Low`, an alert for `Critical`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Importance {
    /// Low importance (e.g. a generic pass).
    Low,
    /// Medium importance (e.g. a shot, a foul).
    Medium,
    /// High importance (e.g. a save, a card).
    High,
    /// Critical importance (e.g. a goal, a penalty, a match abandoned).
    Critical,
}

/// A single narrative entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrativeEntry {
    /// The minute of the match (0–120).
    pub minute: u8,
    /// The ID of the event this entry narrates.
    pub event_id: EventId,
    /// The template key (e.g. `"narrative.goal"`). Ready for i18n.
    pub template_key: String,
    /// The parameters for the template (e.g. `player`, `minute`).
    pub params: BTreeMap<String, String>,
    /// The rendered text, in English (the default language).
    pub text: String,
    /// The importance of the entry.
    pub importance: Importance,
}

/// Builds the narrative from the match and its event chain.
///
/// The entries are in chronological order (by event order in the chain).
/// Every event has exactly one entry.
#[must_use]
pub fn narrate(match_: &Match, chain: &EventChain) -> Vec<NarrativeEntry> {
    let mut names: BTreeMap<PlayerId, &str> = BTreeMap::new();
    for p in &match_.home.players {
        names.insert(p.id, p.name.as_str());
    }
    for p in &match_.away.players {
        names.insert(p.id, p.name.as_str());
    }

    chain
        .events()
        .iter()
        .map(|ev| narrate_event(ev, &names))
        .collect()
}

fn narrate_event(ev: &MatchEvent, names: &BTreeMap<PlayerId, &str>) -> NarrativeEntry {
    let (template_key, importance) = template_for(ev.event_type);
    let player = names.get(&ev.player_id).copied().unwrap_or("<unknown>");
    let minute = ev.timestamp_minute;

    let mut params = BTreeMap::new();
    params.insert("player".to_string(), player.to_string());
    params.insert("minute".to_string(), minute.to_string());

    let text = render_text(template_key, player, minute);

    NarrativeEntry {
        minute,
        event_id: ev.event_id,
        template_key: template_key.to_string(),
        params,
        text,
        importance,
    }
}

/// Returns the template key and importance for an event type.
const fn template_for(event_type: EventType) -> (&'static str, Importance) {
    match event_type {
        EventType::Shot => ("narrative.shot", Importance::Medium),
        EventType::Pass => ("narrative.pass", Importance::Low),
        EventType::Cross => ("narrative.cross", Importance::Low),
        EventType::Dribble => ("narrative.dribble", Importance::Medium),
        EventType::KeyPass => ("narrative.key_pass", Importance::High),
        EventType::Tackle => ("narrative.tackle", Importance::Medium),
        EventType::Interception => ("narrative.interception", Importance::Medium),
        EventType::Clearance => ("narrative.clearance", Importance::Low),
        EventType::Block => ("narrative.block", Importance::Medium),
        EventType::Save => ("narrative.save", Importance::High),
        EventType::Duel => ("narrative.duel", Importance::Low),
        EventType::Foul => ("narrative.foul", Importance::Medium),
        EventType::Card => ("narrative.card", Importance::High),
        EventType::Corner => ("narrative.corner", Importance::Low),
        EventType::FreeKick => ("narrative.free_kick", Importance::Medium),
        EventType::Penalty => ("narrative.penalty", Importance::Critical),
        EventType::Substitution => ("narrative.substitution", Importance::High),
        EventType::Injury => ("narrative.injury", Importance::High),
        EventType::Goal => ("narrative.goal", Importance::Critical),
        EventType::Assist => ("narrative.assist", Importance::High),
        EventType::ProgressiveCarry => ("narrative.progressive_carry", Importance::Medium),
        EventType::ProgressivePass => ("narrative.progressive_pass", Importance::Medium),
        EventType::Pressure => ("narrative.pressure", Importance::Low),
        EventType::RandomEvent => ("narrative.random_event", Importance::Medium),
        EventType::HalfTime => ("narrative.half_time", Importance::High),
        EventType::FullTime => ("narrative.full_time", Importance::High),
        EventType::MatchAbandoned => ("narrative.match_abandoned", Importance::Critical),
    }
}

/// Renders the text in English (the default language).
///
/// TODO: when `locales/` exists, replace this with a lookup of
/// `template_key` in the locale file and a substitution of `params`.
fn render_text(template_key: &str, player: &str, minute: u8) -> String {
    match template_key {
        "narrative.shot" => format!("{player} shoots in the {minute}'"),
        "narrative.pass" => format!("{player} passes in the {minute}'"),
        "narrative.cross" => format!("{player} crosses in the {minute}'"),
        "narrative.dribble" => format!("{player} dribbles in the {minute}'"),
        "narrative.key_pass" => format!("{player} plays a key pass in the {minute}'"),
        "narrative.tackle" => format!("{player} tackles in the {minute}'"),
        "narrative.interception" => format!("{player} intercepts in the {minute}'"),
        "narrative.clearance" => format!("{player} clears in the {minute}'"),
        "narrative.block" => format!("{player} blocks in the {minute}'"),
        "narrative.save" => format!("Save by {player} in the {minute}'"),
        "narrative.duel" => format!("{player} wins a duel in the {minute}'"),
        "narrative.foul" => format!("Foul by {player} in the {minute}'"),
        "narrative.card" => format!("Card for {player} in the {minute}'"),
        "narrative.corner" => format!("Corner for {player} in the {minute}'"),
        "narrative.free_kick" => format!("Free kick for {player} in the {minute}'"),
        "narrative.penalty" => format!("PENALTY! {player} in the {minute}'"),
        "narrative.substitution" => format!("Substitution: {player} in the {minute}'"),
        "narrative.injury" => format!("{player} is injured in the {minute}'"),
        "narrative.goal" => format!("GOAL! {player} scores in the {minute}'"),
        "narrative.assist" => format!("Assist by {player} in the {minute}'"),
        "narrative.progressive_carry" => format!("{player} carries forward in the {minute}'"),
        "narrative.progressive_pass" => {
            format!("{player} plays a progressive pass in the {minute}'")
        }
        "narrative.pressure" => format!("{player} presses in the {minute}'"),
        "narrative.random_event" => format!("Random event: {player} in the {minute}'"),
        "narrative.half_time" => "Half-time".to_string(),
        "narrative.full_time" => "Full-time".to_string(),
        "narrative.match_abandoned" => "Match abandoned".to_string(),
        _ => format!("Event by {player} in the {minute}'"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcfm_core::ids::{EventId, MatchId, PlayerId, RefereeId, SquadId};
    use pcfm_core::time::GameInstant;
    use pcfm_model::{
        Foot, FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
        PsychologicalStateMinimal, Referee, RoleId, Squad,
    };
    use pcfm_stats::{EventChain, EventType, MatchEvent};

    fn make_squad(id: u32, slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("Player{slot_base}_{i}"),
                    RoleId::new("midfielder").unwrap(),
                    Foot::Right,
                    180,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(id, 0),
            players,
            vec![],
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn make_match() -> Match {
        Match::new(
            MatchId::new(1, 0),
            make_squad(1, 0),
            make_squad(2, 100),
            Pitch::default(),
            Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).unwrap(),
            MatchSeed::from_raw(42),
            MatchRules::default(),
        )
    }

    fn make_event(slot: u32, minute: u8, event_type: EventType, player_id: PlayerId) -> MatchEvent {
        MatchEvent {
            event_id: EventId::new(slot, 0),
            match_id: MatchId::new(1, 0),
            player_id,
            timestamp_minute: minute,
            timestamp_second: 0,
            minutes_played_cumulative: u16::from(minute),
            career_minutes_cumulative: u32::from(minute),
            season_label: None,
            event_type,
            fields: serde_json::json!({}),
            observed_attrs_snapshot_hash: None,
            created_at: GameInstant::from_raw(u64::from(minute)),
        }
    }

    #[test]
    fn one_entry_per_event() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Shot, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(3, 30, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative.len(), 3);
    }

    #[test]
    fn entries_are_in_chronological_order() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Shot, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(3, 30, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative[0].minute, 10);
        assert_eq!(narrative[1].minute, 20);
        assert_eq!(narrative[2].minute, 30);
    }

    #[test]
    fn every_entry_has_text_and_key() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        for entry in &narrative {
            assert_ne!(entry.text, "");
            assert_ne!(entry.template_key, "");
            assert!(entry.minute <= 120);
        }
    }

    #[test]
    fn goal_is_critical() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 45, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative[0].importance, Importance::Critical);
        assert_eq!(narrative[0].template_key, "narrative.goal");
        assert!(narrative[0].text.contains("GOAL"));
    }

    #[test]
    fn pass_is_low() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative[0].importance, Importance::Low);
        assert_eq!(narrative[0].template_key, "narrative.pass");
    }

    #[test]
    fn empty_chain_produces_empty_narrative() {
        let m = make_match();
        let chain = EventChain::new();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative.len(), 0);
    }

    #[test]
    fn narrate_is_deterministic() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 10, EventType::Pass, PlayerId::new(0, 0)))
            .unwrap();
        chain
            .try_append(make_event(2, 20, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let a = narrate(&m, &chain);
        let b = narrate(&m, &chain);
        assert_eq!(a, b);
    }

    #[test]
    fn params_contain_player_and_minute() {
        let m = make_match();
        let mut chain = EventChain::new();
        chain
            .try_append(make_event(1, 45, EventType::Goal, PlayerId::new(0, 0)))
            .unwrap();
        let narrative = narrate(&m, &chain);
        assert_eq!(narrative[0].params.get("player").unwrap(), "Player0_0");
        assert_eq!(narrative[0].params.get("minute").unwrap(), "45");
    }
}
