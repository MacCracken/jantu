//! Stress accumulation and its effects on drive baselines.
//!
//! Models allostatic load — the cumulative wear from chronic stress.
//! Acute stress is adaptive (fight-or-flight); chronic stress degrades
//! health, cognition, and reproductive fitness (McEwen & Stellar, 1993).
//!
//! Two-tier model:
//! - **Acute stress**: fast-rising, fast-decaying cortisol-like response
//! - **Chronic stress (allostatic load)**: slow-building, slow-decaying
//!   cumulative damage from repeated/sustained stressors

use serde::{Deserialize, Serialize};

/// Stressor category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StressorType {
    /// Predation threat.
    Predation,
    /// Resource scarcity (food, water).
    ResourceScarcity,
    /// Social conflict (hierarchy challenges, exclusion).
    SocialConflict,
    /// Environmental (weather, habitat disturbance).
    Environmental,
    /// Overcrowding.
    Crowding,
    /// Isolation (social species deprived of contact).
    Isolation,
}

/// Stress state of a creature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressState {
    /// Acute stress level (0.0-1.0). Fast-changing, represents immediate cortisol response.
    pub acute: f32,
    /// Chronic stress / allostatic load (0.0-1.0). Slow-building cumulative damage.
    pub chronic: f32,
    /// Recovery capacity (0.0-1.0). Degrades with chronic stress.
    pub resilience: f32,
}

impl StressState {
    /// Create a fresh, unstressed state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            acute: 0.0,
            chronic: 0.0,
            resilience: 1.0,
        }
    }

    /// Apply an acute stressor.
    ///
    /// `intensity`: stressor severity (0.0-1.0).
    pub fn apply_stressor(&mut self, intensity: f32) {
        let intensity = intensity.clamp(0.0, 1.0);

        // Acute stress spikes immediately
        self.acute = (self.acute + intensity * 0.5).clamp(0.0, 1.0);

        // Chronic stress builds slowly — faster when resilience is low
        let chronic_gain = intensity * 0.05 * (2.0 - self.resilience);
        self.chronic = (self.chronic + chronic_gain).clamp(0.0, 1.0);

        // Resilience erodes with each stressor
        self.resilience = (self.resilience - intensity * 0.02).clamp(0.0, 1.0);
    }

    /// Recover over time. Call each tick with `dt` = time step.
    ///
    /// `safety`: how safe the environment is (0.0-1.0). Safe environments aid recovery.
    pub fn recover(&mut self, dt: f32, safety: f32) {
        let safety = safety.clamp(0.0, 1.0);

        // Acute stress decays quickly
        let acute_decay = 0.1 * dt * (0.5 + safety * 0.5);
        self.acute = (self.acute - acute_decay).max(0.0);

        // Chronic stress decays very slowly, only in safe conditions
        let chronic_decay = 0.005 * dt * safety * self.resilience;
        self.chronic = (self.chronic - chronic_decay).max(0.0);

        // Resilience recovers slowly in safe, low-stress conditions
        let resilience_gain = 0.01 * dt * safety * (1.0 - self.acute);
        self.resilience = (self.resilience + resilience_gain).clamp(0.0, 1.0);
    }

    /// Whether the creature is in acute distress.
    #[must_use]
    #[inline]
    pub fn is_distressed(&self) -> bool {
        self.acute > 0.7
    }

    /// Whether the creature has dangerous chronic stress levels.
    #[must_use]
    #[inline]
    pub fn is_chronically_stressed(&self) -> bool {
        self.chronic > 0.6
    }

    /// Overall stress impact on behavior (0.0 = no impact, 1.0 = debilitated).
    #[must_use]
    pub fn behavioral_impact(&self) -> f32 {
        // Acute stress dominates short-term; chronic degrades long-term
        let acute_impact = self.acute * 0.6;
        let chronic_impact = self.chronic * 0.4;
        (acute_impact + chronic_impact).clamp(0.0, 1.0)
    }
}

impl Default for StressState {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute drive baseline shift due to chronic stress.
///
/// Chronic stress raises anxiety-related drives (fear, aggression) and
/// suppresses luxury drives (curiosity, reproduction).
///
/// Returns a multiplier for the given drive.
/// - `> 1.0` = drive amplified
/// - `< 1.0` = drive suppressed
/// - `is_anxiety_drive`: true for fear/aggression, false for curiosity/reproduction
#[must_use]
pub fn stress_drive_modifier(chronic_stress: f32, is_anxiety_drive: bool) -> f32 {
    let chronic_stress = chronic_stress.clamp(0.0, 1.0);
    if is_anxiety_drive {
        1.0 + chronic_stress * 0.5 // up to 1.5x amplification
    } else {
        1.0 - chronic_stress * 0.6 // down to 0.4x suppression
    }
}

/// Compute immune suppression from chronic stress.
///
/// Returns a health modifier (0.0-1.0) where lower = more immunosuppressed.
#[must_use]
pub fn immune_function(chronic_stress: f32, resilience: f32) -> f32 {
    let chronic_stress = chronic_stress.clamp(0.0, 1.0);
    let resilience = resilience.clamp(0.0, 1.0);
    (1.0 - chronic_stress * 0.5) * (0.5 + resilience * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_state_no_stress() {
        let s = StressState::new();
        assert_eq!(s.acute, 0.0);
        assert_eq!(s.chronic, 0.0);
        assert!((s.resilience - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn stressor_raises_acute() {
        let mut s = StressState::new();
        s.apply_stressor(0.8);
        assert!(s.acute > 0.3);
    }

    #[test]
    fn repeated_stress_builds_chronic() {
        let mut s = StressState::new();
        for _ in 0..50 {
            s.apply_stressor(0.5);
        }
        assert!(
            s.chronic > 0.3,
            "chronic should build over time: {}",
            s.chronic
        );
    }

    #[test]
    fn resilience_erodes() {
        let mut s = StressState::new();
        for _ in 0..30 {
            s.apply_stressor(0.6);
        }
        assert!(
            s.resilience < 0.8,
            "resilience should erode: {}",
            s.resilience
        );
    }

    #[test]
    fn recovery_reduces_acute() {
        let mut s = StressState::new();
        s.apply_stressor(0.9);
        let before = s.acute;
        s.recover(10.0, 0.9);
        assert!(s.acute < before, "acute should decrease with recovery");
    }

    #[test]
    fn safe_environment_aids_recovery() {
        let mut safe = StressState::new();
        let mut dangerous = StressState::new();
        safe.apply_stressor(0.8);
        dangerous.apply_stressor(0.8);

        safe.recover(5.0, 1.0);
        dangerous.recover(5.0, 0.1);

        assert!(
            safe.acute < dangerous.acute,
            "safe environment should recover faster"
        );
    }

    #[test]
    fn chronic_stress_amplifies_anxiety_drives() {
        let mod_anxious = stress_drive_modifier(0.8, true);
        let mod_luxury = stress_drive_modifier(0.8, false);
        assert!(mod_anxious > 1.0, "anxiety drives should be amplified");
        assert!(mod_luxury < 1.0, "luxury drives should be suppressed");
    }

    #[test]
    fn no_stress_no_modifier() {
        assert!((stress_drive_modifier(0.0, true) - 1.0).abs() < f32::EPSILON);
        assert!((stress_drive_modifier(0.0, false) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn chronic_stress_suppresses_immunity() {
        let healthy = immune_function(0.0, 1.0);
        let stressed = immune_function(0.8, 0.5);
        assert!(healthy > stressed);
    }

    #[test]
    fn behavioral_impact_bounded() {
        let mut s = StressState::new();
        for _ in 0..100 {
            s.apply_stressor(1.0);
        }
        assert!((0.0..=1.0).contains(&s.behavioral_impact()));
    }

    #[test]
    fn serde_roundtrip_stressor_type() {
        for t in [
            StressorType::Predation,
            StressorType::ResourceScarcity,
            StressorType::SocialConflict,
            StressorType::Environmental,
            StressorType::Crowding,
            StressorType::Isolation,
        ] {
            let json = serde_json::to_string(&t).unwrap();
            let t2: StressorType = serde_json::from_str(&json).unwrap();
            assert_eq!(t, t2);
        }
    }

    #[test]
    fn serde_roundtrip_stress_state() {
        let mut s = StressState::new();
        s.apply_stressor(0.5);
        let json = serde_json::to_string(&s).unwrap();
        let s2: StressState = serde_json::from_str(&json).unwrap();
        assert!((s.acute - s2.acute).abs() < f32::EPSILON);
        assert!((s.chronic - s2.chronic).abs() < f32::EPSILON);
        assert!((s.resilience - s2.resilience).abs() < f32::EPSILON);
    }
}
