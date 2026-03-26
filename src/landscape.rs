//! Landscape of fear — spatial risk perception and fear-mediated behavior.
//!
//! Models how perceived predation risk varies across space and modulates
//! foraging, movement, and habitat selection. Based on Laundré et al. (2001)
//! landscape of fear framework and Brown et al. (1999) ecology of fear.

use serde::{Deserialize, Serialize};

/// Habitat type affecting perceived risk.
///
/// # Examples
///
/// ```
/// use jantu::landscape::HabitatType;
///
/// let habitat = HabitatType::OpenGrassland;
/// assert_ne!(habitat, HabitatType::DenseForest);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HabitatType {
    /// Dense vegetation providing concealment.
    DenseForest,
    /// Moderate cover with visibility gaps.
    Woodland,
    /// Open terrain with minimal cover.
    OpenGrassland,
    /// Near water — high predator encounter risk.
    Waterhole,
    /// Rocky terrain with escape routes (crevices, cliffs).
    RockyRefuge,
}

/// Compute perceived predation risk at a location.
///
/// Risk depends on predator activity, available cover, and distance to
/// the nearest refuge. Higher risk → more vigilance, less foraging.
///
/// `predator_density`: predators per unit area (0.0+).
/// `cover`: vegetation/terrain concealment (0.0 = exposed, 1.0 = fully hidden).
/// `distance_to_refuge`: distance to nearest safe location (0.0+).
///
/// Returns perceived risk (0.0–1.0).
///
/// ```
/// use jantu::landscape::perceived_risk;
///
/// let exposed = perceived_risk(0.5, 0.1, 50.0);
/// let hidden = perceived_risk(0.5, 0.9, 5.0);
/// assert!(exposed > hidden);
/// ```
#[must_use]
pub fn perceived_risk(predator_density: f32, cover: f32, distance_to_refuge: f32) -> f32 {
    let predator_density = predator_density.max(0.0);
    let cover = cover.clamp(0.0, 1.0);
    let distance_to_refuge = distance_to_refuge.max(0.0);

    // Risk scales with predator density, inversely with cover,
    // and logarithmically with distance to refuge
    let exposure = 1.0 - cover * 0.8;
    let refuge_penalty = (1.0 + distance_to_refuge).ln() * 0.3;
    let base_risk = predator_density * exposure + refuge_penalty;

    // Sigmoid compression into [0, 1]
    let sigmoid = base_risk / (1.0 + base_risk);
    sigmoid.clamp(0.0, 1.0)
}

/// Fear-mediated foraging: how much does perceived risk reduce intake?
///
/// Animals in risky areas forage less efficiently due to vigilance,
/// reduced patch exploitation, and early departure.
///
/// `risk`: perceived predation risk (0.0–1.0, from [`perceived_risk`]).
/// `hunger`: current hunger drive (0.0–1.0). Desperate animals accept more risk.
///
/// Returns foraging efficiency multiplier (0.0–1.0).
///
/// ```
/// use jantu::landscape::fear_foraging_efficiency;
///
/// let safe = fear_foraging_efficiency(0.1, 0.5);
/// let scary = fear_foraging_efficiency(0.9, 0.5);
/// assert!(safe > scary);
///
/// // Starving animals forage despite fear
/// let desperate = fear_foraging_efficiency(0.9, 1.0);
/// assert!(desperate > scary);
/// ```
#[must_use]
pub fn fear_foraging_efficiency(risk: f32, hunger: f32) -> f32 {
    let risk = risk.clamp(0.0, 1.0);
    let hunger = hunger.clamp(0.0, 1.0);

    // Base efficiency reduced by risk
    let fear_suppression = 1.0 - risk * 0.8;
    // Hunger partially overrides fear (state-dependent risk-taking)
    let hunger_override = 1.0 + hunger * risk * 0.4;

    (fear_suppression * hunger_override).clamp(0.0, 1.0)
}

/// Group size effect on individual apprehension (many-eyes / dilution).
///
/// Larger groups reduce per-individual predation risk through:
/// - **Dilution effect**: each individual is less likely to be the target.
/// - **Many-eyes effect**: more sentinels detect predators sooner.
///
/// `group_size`: number of group members (1+).
/// `predator_activity`: baseline predator threat level (0.0–1.0).
///
/// Returns per-individual risk modifier (0.0–1.0).
///
/// ```
/// use jantu::landscape::group_dilution;
///
/// let solo = group_dilution(1, 0.5);
/// let herd = group_dilution(50, 0.5);
/// assert!(solo > herd);
/// ```
#[must_use]
pub fn group_dilution(group_size: u32, predator_activity: f32) -> f32 {
    let group_size = group_size.max(1);
    let predator_activity = predator_activity.clamp(0.0, 1.0);

    // Dilution: risk ~ 1/N; many-eyes: detection ~ sqrt(N)
    // Combined: individual risk ~ predator_activity / sqrt(N)
    predator_activity / (group_size as f32).sqrt()
}

/// Habitat quality adjusted for predation risk (net habitat value).
///
/// A rich but dangerous habitat may be less valuable than a poor but safe one.
///
/// `resource_quality`: food/water/shelter quality (0.0–1.0).
/// `risk`: perceived predation risk (0.0–1.0).
///
/// Returns net habitat value (can be negative for death-trap habitats).
///
/// ```
/// use jantu::landscape::net_habitat_value;
///
/// let safe_poor = net_habitat_value(0.3, 0.1);
/// let rich_dangerous = net_habitat_value(0.8, 0.9);
/// // Sometimes the safe-poor habitat is better
/// assert!(safe_poor > 0.0);
/// ```
#[must_use]
pub fn net_habitat_value(resource_quality: f32, risk: f32) -> f32 {
    let resource_quality = resource_quality.clamp(0.0, 1.0);
    let risk = risk.clamp(0.0, 1.0);
    resource_quality - risk * 0.8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposed_riskier_than_hidden() {
        let exposed = perceived_risk(0.5, 0.1, 50.0);
        let hidden = perceived_risk(0.5, 0.9, 5.0);
        assert!(exposed > hidden);
    }

    #[test]
    fn no_predators_low_risk() {
        let risk = perceived_risk(0.0, 0.5, 10.0);
        assert!(risk < 0.5);
    }

    #[test]
    fn fear_reduces_foraging() {
        let safe = fear_foraging_efficiency(0.1, 0.5);
        let scary = fear_foraging_efficiency(0.9, 0.5);
        assert!(safe > scary);
    }

    #[test]
    fn hunger_overrides_fear() {
        let sated = fear_foraging_efficiency(0.8, 0.2);
        let starving = fear_foraging_efficiency(0.8, 1.0);
        assert!(starving > sated);
    }

    #[test]
    fn group_dilution_effect() {
        let solo = group_dilution(1, 0.5);
        let herd = group_dilution(50, 0.5);
        assert!(solo > herd);
    }

    #[test]
    fn group_dilution_no_predators() {
        assert_eq!(group_dilution(10, 0.0), 0.0);
    }

    #[test]
    fn net_value_safe_positive() {
        assert!(net_habitat_value(0.5, 0.1) > 0.0);
    }

    #[test]
    fn net_value_dangerous_low() {
        let value = net_habitat_value(0.3, 0.9);
        assert!(value < 0.0);
    }

    #[test]
    fn serde_roundtrip_habitat_type() {
        for h in [
            HabitatType::DenseForest,
            HabitatType::Woodland,
            HabitatType::OpenGrassland,
            HabitatType::Waterhole,
            HabitatType::RockyRefuge,
        ] {
            let json = serde_json::to_string(&h).unwrap();
            let h2: HabitatType = serde_json::from_str(&json).unwrap();
            assert_eq!(h, h2);
        }
    }
}
