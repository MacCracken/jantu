//! Predator-prey co-evolution hooks.
//!
//! Models the evolutionary arms race between predators and prey:
//! speed vs. endurance, camouflage vs. detection, venom vs. resistance.
//! Provides trait pressure calculations and adaptation dynamics.

use serde::{Deserialize, Serialize};

/// Ecological role in a predator-prey relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EcologicalRole {
    /// Apex predator — no natural predators.
    ApexPredator,
    /// Mesopredator — predator that is also prey.
    Mesopredator,
    /// Primary prey — herbivore or lower trophic level.
    Prey,
    /// Parasite — exploits host without immediate kill.
    Parasite,
}

/// Arms race trait category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ArmsRaceTrait {
    /// Speed (predator pursuit vs. prey escape).
    Speed,
    /// Detection (predator sensing vs. prey camouflage).
    Detection,
    /// Toxicity (prey venom/poison vs. predator resistance).
    Toxicity,
    /// Armor (prey defense vs. predator bite force).
    Armor,
    /// Endurance (predator persistence vs. prey stamina).
    Endurance,
    /// Group defense (prey coordination vs. predator pack tactics).
    GroupDefense,
}

/// Trait levels for a predator-prey interaction pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitMatchup {
    /// Predator's trait level (0.0-1.0).
    pub predator_trait: f32,
    /// Prey's counter-trait level (0.0-1.0).
    pub prey_trait: f32,
    /// The trait being contested.
    pub trait_type: ArmsRaceTrait,
}

impl TraitMatchup {
    /// Create a new trait matchup.
    #[must_use]
    pub fn new(trait_type: ArmsRaceTrait, predator_trait: f32, prey_trait: f32) -> Self {
        Self {
            predator_trait: predator_trait.clamp(0.0, 1.0),
            prey_trait: prey_trait.clamp(0.0, 1.0),
            trait_type,
        }
    }

    /// Predator advantage: how much the predator's trait exceeds the prey's counter.
    #[must_use]
    #[inline]
    pub fn predator_advantage(&self) -> f32 {
        (self.predator_trait - self.prey_trait).max(0.0)
    }

    /// Prey advantage (inverse).
    #[must_use]
    #[inline]
    pub fn prey_advantage(&self) -> f32 {
        (self.prey_trait - self.predator_trait).max(0.0)
    }
}

/// Compute the selection pressure on a trait given the current matchup.
///
/// When the opponent's trait is higher, pressure to evolve increases.
/// Returns a pressure value (0.0-1.0) that can drive trait change.
#[must_use]
pub fn trait_pressure(own_trait: f32, opponent_trait: f32, selection_intensity: f32) -> f32 {
    let own_trait = own_trait.clamp(0.0, 1.0);
    let opponent_trait = opponent_trait.clamp(0.0, 1.0);
    let selection_intensity = selection_intensity.clamp(0.0, 1.0);

    // Pressure increases when opponent is better; saturates via tanh-like curve
    let deficit = (opponent_trait - own_trait).max(0.0);
    let raw_pressure = deficit * selection_intensity;
    // Diminishing returns as trait approaches maximum
    raw_pressure * (1.0 - own_trait * 0.5)
}

/// Red Queen effect: both species must keep evolving just to maintain relative fitness.
///
/// Given the rates of trait change for predator and prey, returns the
/// net fitness shift for the predator. Positive = predator gaining ground,
/// negative = prey gaining ground, near-zero = Red Queen stasis.
#[must_use]
pub fn red_queen_balance(predator_rate: f32, prey_rate: f32) -> f32 {
    predator_rate - prey_rate
}

/// Compute encounter rate between predator and prey populations.
///
/// Based on Lotka-Volterra encounter rate: proportional to both population
/// densities and predator search efficiency.
///
/// - `predator_density`: predators per unit area
/// - `prey_density`: prey per unit area
/// - `search_efficiency`: predator's ability to find prey (0.0-1.0)
///
/// Returns encounters per unit time per unit area.
#[must_use]
pub fn encounter_rate(predator_density: f32, prey_density: f32, search_efficiency: f32) -> f32 {
    if predator_density <= 0.0 || prey_density <= 0.0 {
        return 0.0;
    }
    let search_efficiency = search_efficiency.clamp(0.0, 1.0);
    predator_density * prey_density * search_efficiency
}

/// Compute predator functional response (Holling Type II).
///
/// Models the relationship between prey density and predator consumption rate,
/// accounting for handling time.
///
/// - `prey_density`: prey per unit area
/// - `attack_rate`: predator attack rate coefficient
/// - `handling_time`: time to subdue and consume one prey item
///
/// Returns consumption rate (prey per predator per time unit).
#[must_use]
pub fn functional_response_type2(prey_density: f32, attack_rate: f32, handling_time: f32) -> f32 {
    if prey_density <= 0.0 || attack_rate <= 0.0 {
        return 0.0;
    }
    (attack_rate * prey_density) / (1.0 + attack_rate * handling_time * prey_density)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predator_advantage_when_faster() {
        let m = TraitMatchup::new(ArmsRaceTrait::Speed, 0.8, 0.5);
        assert!((m.predator_advantage() - 0.3).abs() < f32::EPSILON);
        assert_eq!(m.prey_advantage(), 0.0);
    }

    #[test]
    fn prey_advantage_when_better_camouflaged() {
        let m = TraitMatchup::new(ArmsRaceTrait::Detection, 0.4, 0.7);
        assert!((m.prey_advantage() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn trait_pressure_increases_with_deficit() {
        let low = trait_pressure(0.7, 0.8, 0.5);
        let high = trait_pressure(0.3, 0.8, 0.5);
        assert!(high > low, "bigger deficit = more pressure");
    }

    #[test]
    fn no_pressure_when_ahead() {
        let pressure = trait_pressure(0.9, 0.5, 1.0);
        assert_eq!(pressure, 0.0);
    }

    #[test]
    fn red_queen_stasis() {
        let balance = red_queen_balance(0.05, 0.05);
        assert!(balance.abs() < f32::EPSILON);
    }

    #[test]
    fn red_queen_predator_gaining() {
        assert!(red_queen_balance(0.08, 0.03) > 0.0);
    }

    #[test]
    fn encounter_rate_proportional() {
        let r1 = encounter_rate(10.0, 50.0, 0.5);
        let r2 = encounter_rate(20.0, 50.0, 0.5);
        assert!((r2 - r1 * 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn encounter_rate_zero_density() {
        assert_eq!(encounter_rate(0.0, 50.0, 0.5), 0.0);
        assert_eq!(encounter_rate(10.0, 0.0, 0.5), 0.0);
    }

    #[test]
    fn holling_type2_saturates() {
        let low_prey = functional_response_type2(1.0, 0.5, 0.2);
        let high_prey = functional_response_type2(100.0, 0.5, 0.2);
        // At very high prey density, consumption approaches 1/handling_time = 5.0
        assert!(high_prey > low_prey);
        assert!(high_prey < 5.0 + f32::EPSILON);
    }

    #[test]
    fn holling_type2_zero_safe() {
        assert_eq!(functional_response_type2(0.0, 0.5, 0.2), 0.0);
        assert_eq!(functional_response_type2(10.0, 0.0, 0.2), 0.0);
    }

    #[test]
    fn serde_roundtrip_ecological_role() {
        for r in [
            EcologicalRole::ApexPredator,
            EcologicalRole::Mesopredator,
            EcologicalRole::Prey,
            EcologicalRole::Parasite,
        ] {
            let json = serde_json::to_string(&r).unwrap();
            let r2: EcologicalRole = serde_json::from_str(&json).unwrap();
            assert_eq!(r, r2);
        }
    }

    #[test]
    fn serde_roundtrip_arms_race_trait() {
        for t in [
            ArmsRaceTrait::Speed,
            ArmsRaceTrait::Detection,
            ArmsRaceTrait::Toxicity,
            ArmsRaceTrait::Armor,
            ArmsRaceTrait::Endurance,
            ArmsRaceTrait::GroupDefense,
        ] {
            let json = serde_json::to_string(&t).unwrap();
            let t2: ArmsRaceTrait = serde_json::from_str(&json).unwrap();
            assert_eq!(t, t2);
        }
    }

    #[test]
    fn serde_roundtrip_trait_matchup() {
        let m = TraitMatchup::new(ArmsRaceTrait::Speed, 0.7, 0.6);
        let json = serde_json::to_string(&m).unwrap();
        let m2: TraitMatchup = serde_json::from_str(&json).unwrap();
        assert!((m.predator_trait - m2.predator_trait).abs() < f32::EPSILON);
        assert_eq!(m.trait_type, m2.trait_type);
    }
}
