//! View model for the board renderer (ADR-025).
//!
//! `MatchViewModel` projects a [`MatchResult`] into a read-only,
//! serialisable shape for presentation. It reads Plane B only: the
//! `MatchResult`, never the engine's internal match state nor the
//! hidden Plane A.
//!
//! Structural invariant (ADR-022 §3, ADR-025): this module must not
//! reference the engine's internal state type, the hidden crate, or the
//! pitch model. The test at the bottom of this file checks it mechanically
//! by scanning its own source with `include_str!`.

use pcfm_match::{Importance, MatchResult, NarrativeEntry};
use pcfm_stats::{EventType, MatchEvent};
use serde::Serialize;

/// A single row of the event list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EventRow {
    /// Match minute, 0-120.
    pub minute: u8,
    /// Match second, 0-59.
    pub second: u8,
    /// Event kind, as a stable display string.
    pub kind: &'static str,
    /// Player slot.
    pub player_slot: u32,
}

/// A single row of the narrative list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeRow {
    /// Match minute, 0-120.
    pub minute: u8,
    /// The narrative text, as produced by the engine.
    pub text: String,
    /// Importance, as a stable display string.
    pub importance: &'static str,
}

/// A read-only projection of a [`MatchResult`] for the board renderer.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MatchViewModel {
    /// Home goals, away goals.
    pub score: (u8, u8),
    /// Possession as percentages (home, away), summing to 100.0.
    pub possession_pct: (f32, f32),
    /// Shots, as `(home, away)`.
    pub shots: (u16, u16),
    /// Goals, as `(home, away)`.
    pub goals: (u8, u8),
    /// The event list, in chain order.
    pub events: Vec<EventRow>,
    /// The narrative, in engine order.
    pub narrative: Vec<NarrativeRow>,
    /// The state hash, lowercase hex.
    pub state_hash: String,
}

impl MatchViewModel {
    /// Projects a `MatchResult` into a view model.
    ///
    /// Pure: no I/O, no RNG, no mutation of the source.
    #[must_use]
    pub fn from_result(result: &MatchResult) -> Self {
        let events = result.events.events().iter().map(event_row).collect();
        let narrative = result.narrative.iter().map(narrative_row).collect();
        Self {
            score: (result.score.home, result.score.away),
            possession_pct: (
                result.stats.possession.0 * 100.0,
                result.stats.possession.1 * 100.0,
            ),
            shots: result.stats.shots,
            goals: result.stats.goals,
            events,
            narrative,
            state_hash: hex(result.state_hash.as_bytes()),
        }
    }
}

/// Projects one `MatchEvent` into an `EventRow`.
const fn event_row(event: &MatchEvent) -> EventRow {
    EventRow {
        minute: event.timestamp_minute,
        second: event.timestamp_second,
        kind: event_kind(event.event_type),
        player_slot: event.player_id.slot(),
    }
}

/// Projects one `NarrativeEntry` into a `NarrativeRow`.
fn narrative_row(entry: &NarrativeEntry) -> NarrativeRow {
    NarrativeRow {
        minute: entry.minute,
        text: entry.text.clone(),
        importance: importance_kind(entry.importance),
    }
}

/// A stable display string for every `EventType`.
///
/// Exhaustive: no `_ =>` arm. When a new variant is added to `EventType`,
/// this function stops compiling until the mapping is updated. That is the
/// intended behaviour: the board must know every event kind.
const fn event_kind(kind: EventType) -> &'static str {
    match kind {
        EventType::Shot => "shot",
        EventType::Pass => "pass",
        EventType::Cross => "cross",
        EventType::Dribble => "dribble",
        EventType::KeyPass => "key_pass",
        EventType::Tackle => "tackle",
        EventType::Interception => "interception",
        EventType::Clearance => "clearance",
        EventType::Block => "block",
        EventType::Save => "save",
        EventType::Duel => "duel",
        EventType::Foul => "foul",
        EventType::Card => "card",
        EventType::Corner => "corner",
        EventType::FreeKick => "free_kick",
        EventType::Penalty => "penalty",
        EventType::Substitution => "substitution",
        EventType::Injury => "injury",
        EventType::Goal => "goal",
        EventType::Assist => "assist",
        EventType::ProgressiveCarry => "progressive_carry",
        EventType::ProgressivePass => "progressive_pass",
        EventType::Pressure => "pressure",
        EventType::RandomEvent => "random_event",
        EventType::HalfTime => "half_time",
        EventType::FullTime => "full_time",
        EventType::MatchAbandoned => "match_abandoned",
    }
}

/// A stable display string for every `Importance`.
const fn importance_kind(importance: Importance) -> &'static str {
    match importance {
        Importance::Low => "low",
        Importance::Medium => "medium",
        Importance::High => "high",
        Importance::Critical => "critical",
    }
}

/// Lowercase hex of a byte slice.
fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ADR-025 §Enforcement: this module must not reference the hidden
    /// crate, the engine's internal match state type, or the pitch model.
    /// Mechanical check on the file's own source, via `include_str!`.
    #[test]
    fn viewmodel_does_not_reference_forbidden_types() {
        let source = include_str!("viewmodel.rs");
        let forbidden = [
            concat!("pcfm", "_hidden"),
            concat!("Match", "State"),
            concat!("pcfm_model", "::pitch"),
        ];
        for needle in forbidden {
            assert!(
                !source.contains(needle),
                "viewmodel.rs must not reference {needle} (ADR-022 §3, ADR-025)"
            );
        }
    }

    /// Every `EventType` maps to a non-empty string.
    #[test]
    fn every_event_kind_has_a_display_string() {
        let kinds = [
            EventType::Shot,
            EventType::Pass,
            EventType::Cross,
            EventType::Dribble,
            EventType::KeyPass,
            EventType::Tackle,
            EventType::Interception,
            EventType::Clearance,
            EventType::Block,
            EventType::Save,
            EventType::Duel,
            EventType::Foul,
            EventType::Card,
            EventType::Corner,
            EventType::FreeKick,
            EventType::Penalty,
            EventType::Substitution,
            EventType::Injury,
            EventType::Goal,
            EventType::Assist,
            EventType::ProgressiveCarry,
            EventType::ProgressivePass,
            EventType::Pressure,
            EventType::RandomEvent,
            EventType::HalfTime,
            EventType::FullTime,
            EventType::MatchAbandoned,
        ];
        for kind in kinds {
            let s = event_kind(kind);
            assert!(!s.is_empty(), "empty display string for {kind:?}");
        }
    }

    /// Hex of a known byte slice is lowercase.
    #[test]
    fn hex_is_lowercase() {
        assert_eq!(hex(&[0xDE, 0xAD, 0xBE, 0xEF]), "deadbeef");
    }
}
