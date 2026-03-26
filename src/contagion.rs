//! Emotional contagion — affective state spreading through groups.
//!
//! Models how fear, aggression, and other emotional states propagate between
//! nearby creatures. Based on Hatfield et al. (1993) emotional contagion theory
//! adapted for animal behavior (social facilitation, alarm spreading).
//!
//! Key dynamics:
//! - Proximity increases transmission (inverse-square falloff)
//! - Social rank modulates influence (higher rank = more contagious)
//! - Susceptibility varies by individual and state

use serde::{Deserialize, Serialize};

/// An emotional state that can spread between creatures.
///
/// # Examples
///
/// ```
/// use jantu::contagion::EmotionalState;
///
/// let state = EmotionalState::Fear;
/// assert_ne!(state, EmotionalState::Calm);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EmotionalState {
    /// Fear/alarm — fastest spreading, highest priority.
    Fear,
    /// Aggression — spreads in defensive/territorial contexts.
    Aggression,
    /// Calm — suppresses fear/aggression, spreads slowly.
    Calm,
    /// Excitement — foraging success, play behavior.
    Excitement,
}

/// Susceptibility profile controlling how easily a creature catches emotions.
///
/// # Examples
///
/// ```
/// use jantu::contagion::Susceptibility;
///
/// let low_rank = Susceptibility::new(0.7, 0.1, 0.3);
/// let high_rank = Susceptibility::new(0.7, 0.9, 0.3);
/// assert!(low_rank.effective() > high_rank.effective());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Susceptibility {
    /// Base susceptibility (0.0-1.0). Higher = more easily influenced.
    pub base: f32,
    /// Social rank modifier. Lower-ranked creatures are typically more susceptible.
    pub rank: f32,
    /// Current arousal level. Already-aroused creatures are more susceptible to
    /// congruent emotions.
    pub arousal: f32,
}

impl Susceptibility {
    /// Create a new susceptibility profile.
    #[must_use]
    pub fn new(base: f32, rank: f32, arousal: f32) -> Self {
        Self {
            base: base.clamp(0.0, 1.0),
            rank: rank.clamp(0.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
        }
    }

    /// Effective susceptibility considering rank and arousal.
    #[must_use]
    #[inline]
    pub fn effective(&self) -> f32 {
        // Lower rank → more susceptible; higher arousal → more susceptible
        let rank_factor = 1.0 - self.rank * 0.5;
        (self.base * rank_factor * (1.0 + self.arousal * 0.5)).clamp(0.0, 1.0)
    }
}

/// Compute the emotional influence one creature exerts on another.
///
/// ```
/// use jantu::contagion::emotional_influence;
///
/// let close = emotional_influence(0.8, 0.7, 5.0, 100.0);
/// let far = emotional_influence(0.8, 0.7, 80.0, 100.0);
/// assert!(close > far);
/// ```
///
/// - `emitter_intensity`: how strongly the emitter is feeling the emotion (0.0-1.0)
/// - `emitter_rank`: social rank of the emitter (0.0-1.0, higher = more influential)
/// - `distance`: distance between the two creatures
/// - `max_range`: maximum range of influence
///
/// Returns the raw influence magnitude (0.0-1.0) before susceptibility scaling.
#[must_use]
pub fn emotional_influence(
    emitter_intensity: f32,
    emitter_rank: f32,
    distance: f32,
    max_range: f32,
) -> f32 {
    if distance >= max_range || max_range <= 0.0 {
        return 0.0;
    }
    let emitter_intensity = emitter_intensity.clamp(0.0, 1.0);
    let emitter_rank = emitter_rank.clamp(0.0, 1.0);

    // Inverse-square falloff with normalization
    let normalized_dist = distance / max_range;
    let proximity = (1.0 - normalized_dist) * (1.0 - normalized_dist);

    // Higher rank = more contagious
    let rank_boost = 0.5 + emitter_rank * 0.5;

    (emitter_intensity * proximity * rank_boost).clamp(0.0, 1.0)
}

/// Compute the aggregate emotional pressure on a creature from multiple group members.
///
/// Each entry in `influences` is (influence_magnitude, emotional_state).
/// Returns the dominant state and its total pressure.
///
/// Congruent influences sum; the state with the highest total wins.
///
/// ```
/// use jantu::contagion::{aggregate_pressure, EmotionalState};
///
/// let influences = [(0.8, EmotionalState::Fear), (0.5, EmotionalState::Calm)];
/// let (state, _) = aggregate_pressure(&influences).unwrap();
/// assert_eq!(state, EmotionalState::Fear);
/// ```
#[must_use]
pub fn aggregate_pressure(influences: &[(f32, EmotionalState)]) -> Option<(EmotionalState, f32)> {
    if influences.is_empty() {
        return None;
    }

    let mut fear_total = 0.0_f32;
    let mut aggression_total = 0.0_f32;
    let mut calm_total = 0.0_f32;
    let mut excitement_total = 0.0_f32;

    for &(magnitude, state) in influences {
        match state {
            EmotionalState::Fear => fear_total += magnitude,
            EmotionalState::Aggression => aggression_total += magnitude,
            EmotionalState::Calm => calm_total += magnitude,
            EmotionalState::Excitement => excitement_total += magnitude,
        }
    }

    let candidates = [
        (fear_total, EmotionalState::Fear),
        (aggression_total, EmotionalState::Aggression),
        (calm_total, EmotionalState::Calm),
        (excitement_total, EmotionalState::Excitement),
    ];

    candidates
        .iter()
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(core::cmp::Ordering::Equal))
        .map(|&(total, state)| (state, total))
}

/// Compute emotional contagion transfer: how much an emotion shifts the receiver's state.
///
/// `influence`: raw influence from `emotional_influence()`
/// `susceptibility`: receiver's effective susceptibility
/// `state_match`: whether the receiver already feels the same emotion (amplifies transfer)
///
/// Returns the drive change to apply (0.0-1.0).
///
/// ```
/// use jantu::contagion::contagion_transfer;
///
/// let matched = contagion_transfer(0.5, 0.8, true);
/// let unmatched = contagion_transfer(0.5, 0.8, false);
/// assert!(matched > unmatched);
/// ```
#[must_use]
pub fn contagion_transfer(influence: f32, susceptibility: f32, state_match: bool) -> f32 {
    let base = influence * susceptibility;
    let match_bonus = if state_match { 1.5 } else { 1.0 };
    (base * match_bonus).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_creatures_more_influenced() {
        let close = emotional_influence(0.8, 0.5, 1.0, 100.0);
        let far = emotional_influence(0.8, 0.5, 80.0, 100.0);
        assert!(close > far, "close={close}, far={far}");
    }

    #[test]
    fn beyond_range_no_influence() {
        let inf = emotional_influence(1.0, 1.0, 101.0, 100.0);
        assert_eq!(inf, 0.0);
    }

    #[test]
    fn higher_rank_more_contagious() {
        let alpha = emotional_influence(0.8, 0.9, 10.0, 100.0);
        let omega = emotional_influence(0.8, 0.1, 10.0, 100.0);
        assert!(alpha > omega, "alpha={alpha}, omega={omega}");
    }

    #[test]
    fn susceptibility_rank_effect() {
        let low_rank = Susceptibility::new(0.7, 0.1, 0.3);
        let high_rank = Susceptibility::new(0.7, 0.9, 0.3);
        assert!(
            low_rank.effective() > high_rank.effective(),
            "lower rank should be more susceptible"
        );
    }

    #[test]
    fn aggregate_fear_dominates() {
        let influences = vec![
            (0.8, EmotionalState::Fear),
            (0.6, EmotionalState::Fear),
            (0.5, EmotionalState::Calm),
        ];
        let (state, _) = aggregate_pressure(&influences).unwrap();
        assert_eq!(state, EmotionalState::Fear);
    }

    #[test]
    fn aggregate_empty_returns_none() {
        assert!(aggregate_pressure(&[]).is_none());
    }

    #[test]
    fn contagion_transfer_amplified_by_match() {
        let matched = contagion_transfer(0.5, 0.8, true);
        let unmatched = contagion_transfer(0.5, 0.8, false);
        assert!(matched > unmatched);
    }

    #[test]
    fn contagion_transfer_clamped() {
        let result = contagion_transfer(1.0, 1.0, true);
        assert!(result <= 1.0);
    }

    #[test]
    fn serde_roundtrip_emotional_state() {
        for s in [
            EmotionalState::Fear,
            EmotionalState::Aggression,
            EmotionalState::Calm,
            EmotionalState::Excitement,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let s2: EmotionalState = serde_json::from_str(&json).unwrap();
            assert_eq!(s, s2);
        }
    }

    #[test]
    fn serde_roundtrip_susceptibility() {
        let s = Susceptibility::new(0.7, 0.3, 0.5);
        let json = serde_json::to_string(&s).unwrap();
        let s2: Susceptibility = serde_json::from_str(&json).unwrap();
        assert!((s.base - s2.base).abs() < f32::EPSILON);
        assert!((s.rank - s2.rank).abs() < f32::EPSILON);
        assert!((s.arousal - s2.arousal).abs() < f32::EPSILON);
    }
}
