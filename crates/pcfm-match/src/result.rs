//! The result of a match simulation.

use pcfm_core::hash::Hash;
use pcfm_stats::EventChain;
use serde::{Deserialize, Serialize};

use crate::narrative::NarrativeEntry;

/// The score of a match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Score {
    /// The home team's goals.
    pub home: u8,
    /// The away team's goals.
    pub away: u8,
}

/// The statistics of a match.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchStatistics {
    /// Possession, as `(home, away)`, sums to 1.0.
    pub possession: (f32, f32),
    /// Shots, as `(home, away)`.
    pub shots: (u16, u16),
    /// Goals, as `(home, away)`.
    pub goals: (u8, u8),
}

/// A per-tick snapshot of the simulation state.
///
/// This is the canonical temporal record of the match: a rendering engine or
/// viewer can reconstruct a near-instant record from these snapshots, but the
/// simulation itself is defined by the tick loop and the event chain. In other
/// words, the frame is merely a projection of the tick state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TickSnapshot {
    /// The simulation tick.
    pub tick: u64,
    /// Score at this tick.
    pub score: Score,
    /// Home possession ticks accumulated by this point.
    pub home_possession_ticks: u64,
    /// Away possession ticks accumulated by this point.
    pub away_possession_ticks: u64,
    /// Number of events currently in the chain.
    pub events_in_chain: usize,
    /// The chain hash for this tick.
    pub state_hash: Hash,
}

/// The result of a match simulation.
///
/// The `narrative` field is **data**, not a rendered string. It is filled by
/// the **renderer** (e.g. `TextMatchEngine`), not by `simulate_match`. A
/// renderer that does not need narrative leaves it empty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    /// The score.
    pub score: Score,
    /// The append-only event chain.
    pub events: EventChain,
    /// The statistics.
    pub stats: MatchStatistics,
    /// The state hash, equal to `events.chain_hash()`.
    pub state_hash: Hash,
    /// The narrative entries. Empty if no renderer has filled them.
    pub narrative: Vec<NarrativeEntry>,
    /// The simulated state at each tick.
    pub snapshots: Vec<TickSnapshot>,
}
