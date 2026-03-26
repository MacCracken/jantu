//! Spatial and social memory — recognition of locations, individuals, and threats.
//!
//! Models how creatures build and use spatial maps, recognize familiar individuals,
//! and remember threat locations. Memory fades over time (power-law forgetting)
//! and can be reinforced by repeated exposure.

use serde::{Deserialize, Serialize};

/// Category of remembered information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MemoryType {
    /// Location of food sources.
    FoodSource,
    /// Location of water sources.
    WaterSource,
    /// Location of shelter/nest.
    Shelter,
    /// Dangerous location (predator territory, traps).
    Threat,
    /// Known individual (social recognition).
    Individual,
    /// Territorial boundary.
    Territory,
    /// Migration waypoint.
    Waypoint,
}

/// A single memory trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTrace {
    /// What kind of thing is remembered.
    pub memory_type: MemoryType,
    /// Strength of the memory (0.0-1.0). Decays over time, reinforced by exposure.
    pub strength: f32,
    /// How many times this memory has been reinforced.
    pub reinforcement_count: u32,
    /// Emotional valence associated with the memory (-1.0 = aversive, +1.0 = appetitive).
    pub valence: f32,
}

impl MemoryTrace {
    /// Create a new memory trace.
    #[must_use]
    pub fn new(memory_type: MemoryType, initial_strength: f32, valence: f32) -> Self {
        Self {
            memory_type,
            strength: initial_strength.clamp(0.0, 1.0),
            reinforcement_count: 1,
            valence: valence.clamp(-1.0, 1.0),
        }
    }

    /// Reinforce the memory (re-exposure strengthens it).
    pub fn reinforce(&mut self, amount: f32) {
        let amount = amount.clamp(0.0, 1.0);
        self.reinforcement_count = self.reinforcement_count.saturating_add(1);
        // Diminishing returns on reinforcement
        let room = 1.0 - self.strength;
        self.strength += room * amount * 0.5;
        self.strength = self.strength.clamp(0.0, 1.0);
    }

    /// Apply forgetting over time.
    ///
    /// Uses power-law decay: memories with more reinforcements decay slower.
    pub fn forget(&mut self, dt: f32) {
        // Base decay rate, reduced by reinforcement count (well-practiced memories persist)
        let reinforcement_protection = 1.0 / (1.0 + self.reinforcement_count as f32 * 0.2);
        let decay = 0.01 * dt * reinforcement_protection;
        self.strength = (self.strength - decay).max(0.0);
    }

    /// Whether this memory is still accessible (above recall threshold).
    #[must_use]
    #[inline]
    pub fn is_accessible(&self) -> bool {
        self.strength > 0.1
    }

    /// Whether this is an aversive (negative) memory.
    #[must_use]
    #[inline]
    pub fn is_aversive(&self) -> bool {
        self.valence < -0.3
    }

    /// Whether this is an appetitive (positive) memory.
    #[must_use]
    #[inline]
    pub fn is_appetitive(&self) -> bool {
        self.valence > 0.3
    }
}

/// Compute familiarity-based response bias.
///
/// Familiar stimuli elicit reduced fear response (neophobia reduction).
/// `familiarity` is the memory strength for this stimulus (0.0-1.0).
///
/// Returns a fear modifier (0.0-1.0) where lower = less fear.
#[must_use]
pub fn neophobia_modifier(familiarity: f32) -> f32 {
    let familiarity = familiarity.clamp(0.0, 1.0);
    // Novel things are scary; familiar things less so
    1.0 - familiarity * 0.7
}

/// Compute spatial memory reliability — how much to trust a remembered location.
///
/// Factors: memory strength, time since last visit, environment stability.
///
/// - `memory_strength`: current trace strength (0.0-1.0)
/// - `environment_stability`: how stable the environment is (0.0-1.0)
///
/// Returns reliability (0.0-1.0).
#[must_use]
pub fn spatial_reliability(memory_strength: f32, environment_stability: f32) -> f32 {
    let memory_strength = memory_strength.clamp(0.0, 1.0);
    let environment_stability = environment_stability.clamp(0.0, 1.0);
    memory_strength * (0.3 + environment_stability * 0.7)
}

/// Compute social recognition strength.
///
/// Repeated encounters strengthen recognition. Returns the expected recognition
/// level after `encounters` meetings with decay `time_since_last` ticks ago.
#[must_use]
pub fn social_recognition(encounters: u32, time_since_last: f32) -> f32 {
    if encounters == 0 {
        return 0.0;
    }
    // Build up: asymptotic approach to 1.0 with more encounters
    let buildup = 1.0 - (-0.3 * encounters as f32).exp();
    // Decay since last encounter
    let decay = (-0.01 * time_since_last).exp();
    buildup * decay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_memory_has_strength() {
        let m = MemoryTrace::new(MemoryType::FoodSource, 0.8, 0.5);
        assert!((m.strength - 0.8).abs() < f32::EPSILON);
        assert_eq!(m.reinforcement_count, 1);
    }

    #[test]
    fn reinforcement_increases_strength() {
        let mut m = MemoryTrace::new(MemoryType::FoodSource, 0.5, 0.3);
        let before = m.strength;
        m.reinforce(0.6);
        assert!(m.strength > before);
        assert_eq!(m.reinforcement_count, 2);
    }

    #[test]
    fn reinforcement_diminishing_returns() {
        let mut m = MemoryTrace::new(MemoryType::Shelter, 0.9, 0.5);
        let before = m.strength;
        m.reinforce(0.5);
        let gain = m.strength - before;
        assert!(gain < 0.1, "near-max memory should gain little: {gain}");
    }

    #[test]
    fn forgetting_reduces_strength() {
        let mut m = MemoryTrace::new(MemoryType::Threat, 0.8, -0.9);
        m.forget(50.0);
        assert!(m.strength < 0.8);
    }

    #[test]
    fn reinforced_memories_decay_slower() {
        let mut fresh = MemoryTrace::new(MemoryType::FoodSource, 0.8, 0.5);
        let mut practiced = MemoryTrace::new(MemoryType::FoodSource, 0.8, 0.5);
        for _ in 0..10 {
            practiced.reinforce(0.3);
        }
        // Reset to same strength for fair comparison
        practiced.strength = 0.8;

        fresh.forget(20.0);
        practiced.forget(20.0);
        assert!(
            practiced.strength > fresh.strength,
            "practiced should decay slower: practiced={}, fresh={}",
            practiced.strength,
            fresh.strength
        );
    }

    #[test]
    fn inaccessible_when_forgotten() {
        let mut m = MemoryTrace::new(MemoryType::WaterSource, 0.5, 0.3);
        m.forget(200.0);
        assert!(!m.is_accessible());
    }

    #[test]
    fn valence_classification() {
        let threat = MemoryTrace::new(MemoryType::Threat, 0.7, -0.8);
        assert!(threat.is_aversive());
        assert!(!threat.is_appetitive());

        let food = MemoryTrace::new(MemoryType::FoodSource, 0.7, 0.8);
        assert!(food.is_appetitive());
        assert!(!food.is_aversive());
    }

    #[test]
    fn neophobia_decreases_with_familiarity() {
        let novel = neophobia_modifier(0.0);
        let familiar = neophobia_modifier(0.9);
        assert!(novel > familiar, "novel should be scarier");
        assert!((novel - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn spatial_reliability_depends_on_stability() {
        let stable = spatial_reliability(0.8, 0.9);
        let unstable = spatial_reliability(0.8, 0.1);
        assert!(stable > unstable);
    }

    #[test]
    fn social_recognition_builds_with_encounters() {
        let few = social_recognition(1, 0.0);
        let many = social_recognition(10, 0.0);
        assert!(many > few);
    }

    #[test]
    fn social_recognition_decays_with_time() {
        let recent = social_recognition(5, 0.0);
        let old = social_recognition(5, 100.0);
        assert!(recent > old);
    }

    #[test]
    fn social_recognition_zero_encounters() {
        assert_eq!(social_recognition(0, 10.0), 0.0);
    }

    #[test]
    fn serde_roundtrip_memory_type() {
        for t in [
            MemoryType::FoodSource,
            MemoryType::WaterSource,
            MemoryType::Shelter,
            MemoryType::Threat,
            MemoryType::Individual,
            MemoryType::Territory,
            MemoryType::Waypoint,
        ] {
            let json = serde_json::to_string(&t).unwrap();
            let t2: MemoryType = serde_json::from_str(&json).unwrap();
            assert_eq!(t, t2);
        }
    }

    #[test]
    fn serde_roundtrip_memory_trace() {
        let m = MemoryTrace::new(MemoryType::Threat, 0.7, -0.5);
        let json = serde_json::to_string(&m).unwrap();
        let m2: MemoryTrace = serde_json::from_str(&json).unwrap();
        assert!((m.strength - m2.strength).abs() < f32::EPSILON);
        assert!((m.valence - m2.valence).abs() < f32::EPSILON);
        assert_eq!(m.memory_type, m2.memory_type);
    }
}
