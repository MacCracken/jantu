//! Optimal foraging theory — prey selection, patch departure, and vigilance.
//!
//! Models how creatures optimize energy intake under predation risk.
//! Based on MacArthur & Pianka (1966) optimal diet, Charnov (1976)
//! marginal value theorem, and Brown (1988) giving-up density.

use serde::{Deserialize, Serialize};

/// Prey item profitability for optimal diet selection.
///
/// # Examples
///
/// ```
/// use jantu::foraging::PreyItem;
///
/// let beetle = PreyItem { energy: 5.0, handling_time: 2.0 };
/// assert!((beetle.profitability() - 2.5).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PreyItem {
    /// Net energy gained from consuming this item.
    pub energy: f32,
    /// Time required to pursue, capture, and consume (same units as encounter rate).
    pub handling_time: f32,
}

impl PreyItem {
    /// Energy per unit handling time — the currency of optimal diet theory.
    #[must_use]
    #[inline]
    pub fn profitability(&self) -> f32 {
        if self.handling_time <= 0.0 {
            return 0.0;
        }
        self.energy / self.handling_time
    }
}

/// Should the creature include this prey item in its diet?
///
/// Optimal diet model (MacArthur & Pianka 1966): include a prey type if its
/// profitability exceeds the average intake rate from higher-ranked prey alone.
///
/// `item`: the candidate prey item.
/// `current_intake_rate`: average energy/time from currently included prey types.
///
/// Returns `true` if the item should be taken.
///
/// ```
/// use jantu::foraging::{PreyItem, should_pursue};
///
/// let rich = PreyItem { energy: 10.0, handling_time: 1.0 }; // profitability 10.0
/// let poor = PreyItem { energy: 1.0, handling_time: 5.0 };  // profitability 0.2
///
/// assert!(should_pursue(&rich, 5.0));   // 10.0 > 5.0
/// assert!(!should_pursue(&poor, 5.0));  // 0.2 < 5.0
/// ```
#[must_use]
pub fn should_pursue(item: &PreyItem, current_intake_rate: f32) -> bool {
    item.profitability() > current_intake_rate
}

/// Should the creature leave this food patch? (Marginal Value Theorem)
///
/// Charnov (1976): leave when the marginal intake rate in the current patch
/// drops to the average habitat-wide intake rate (including travel time).
///
/// `current_patch_rate`: instantaneous intake rate in this patch (declining as it depletes).
/// `average_habitat_rate`: long-term average intake rate across all patches including travel.
///
/// Returns `true` if the creature should depart.
///
/// ```
/// use jantu::foraging::should_leave_patch;
///
/// assert!(should_leave_patch(0.5, 1.0));   // patch depleted below average
/// assert!(!should_leave_patch(2.0, 1.0));  // patch still rich
/// ```
#[must_use]
pub fn should_leave_patch(current_patch_rate: f32, average_habitat_rate: f32) -> bool {
    current_patch_rate <= average_habitat_rate
}

/// Giving-up density: food remaining when the forager departs.
///
/// Brown (1988): GUD reflects the balance between metabolic cost of foraging,
/// predation risk, and missed opportunity cost. Higher risk → higher GUD
/// (leave more food behind).
///
/// `predation_risk`: perceived danger level (0.0–1.0).
/// `metabolic_cost`: energy cost of foraging per unit time (0.0–1.0 normalized).
/// `patch_quality`: initial resource density (arbitrary units).
///
/// Returns the expected remaining food density when the forager leaves.
///
/// ```
/// use jantu::foraging::giving_up_density;
///
/// let safe = giving_up_density(0.1, 0.2, 10.0);
/// let risky = giving_up_density(0.8, 0.2, 10.0);
/// assert!(risky > safe); // leave more food in risky patches
/// ```
#[must_use]
pub fn giving_up_density(predation_risk: f32, metabolic_cost: f32, patch_quality: f32) -> f32 {
    let predation_risk = predation_risk.clamp(0.0, 1.0);
    let metabolic_cost = metabolic_cost.clamp(0.0, 1.0);
    // GUD = patch_quality * (risk + metabolic_cost) / (1 + risk + metabolic_cost)
    let cost = predation_risk + metabolic_cost;
    patch_quality * cost / (1.0 + cost)
}

/// Vigilance tradeoff: fraction of time spent scanning vs foraging.
///
/// Animals in riskier environments spend more time scanning for predators,
/// reducing foraging efficiency. Group size provides dilution (many-eyes effect).
///
/// `predation_risk`: perceived danger (0.0–1.0).
/// `hunger`: current hunger drive (0.0–1.0). Hungry animals reduce vigilance.
/// `group_size`: number of group members (more eyes → less individual vigilance).
///
/// Returns vigilance fraction (0.0–1.0) where 1.0 = pure scanning, 0.0 = pure foraging.
///
/// ```
/// use jantu::foraging::vigilance_fraction;
///
/// let solo_risky = vigilance_fraction(0.8, 0.3, 1);
/// let group_risky = vigilance_fraction(0.8, 0.3, 10);
/// assert!(solo_risky > group_risky); // group dilution reduces vigilance
///
/// let hungry = vigilance_fraction(0.8, 0.9, 1);
/// assert!(hungry < solo_risky); // hunger suppresses vigilance
/// ```
#[must_use]
pub fn vigilance_fraction(predation_risk: f32, hunger: f32, group_size: u32) -> f32 {
    let predation_risk = predation_risk.clamp(0.0, 1.0);
    let hunger = hunger.clamp(0.0, 1.0);
    let group_size = group_size.max(1);

    // Base vigilance proportional to risk
    let base = predation_risk * 0.8;
    // Hunger suppresses vigilance (risk-prone when starving)
    let hunger_suppression = 1.0 - hunger * 0.6;
    // Group dilution: 1/sqrt(N) scaling (many-eyes hypothesis)
    let group_dilution = 1.0 / (group_size as f32).sqrt();

    (base * hunger_suppression * group_dilution).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profitability_zero_handling() {
        let item = PreyItem {
            energy: 5.0,
            handling_time: 0.0,
        };
        assert_eq!(item.profitability(), 0.0);
    }

    #[test]
    fn optimal_diet_includes_profitable() {
        let rich = PreyItem {
            energy: 10.0,
            handling_time: 1.0,
        };
        assert!(should_pursue(&rich, 5.0));
    }

    #[test]
    fn optimal_diet_rejects_poor() {
        let poor = PreyItem {
            energy: 1.0,
            handling_time: 5.0,
        };
        assert!(!should_pursue(&poor, 5.0));
    }

    #[test]
    fn leave_depleted_patch() {
        assert!(should_leave_patch(0.5, 1.0));
    }

    #[test]
    fn stay_in_rich_patch() {
        assert!(!should_leave_patch(2.0, 1.0));
    }

    #[test]
    fn higher_risk_higher_gud() {
        let safe = giving_up_density(0.1, 0.2, 10.0);
        let risky = giving_up_density(0.8, 0.2, 10.0);
        assert!(risky > safe);
    }

    #[test]
    fn zero_risk_zero_cost_zero_gud() {
        let gud = giving_up_density(0.0, 0.0, 10.0);
        assert_eq!(gud, 0.0);
    }

    #[test]
    fn group_reduces_vigilance() {
        let solo = vigilance_fraction(0.8, 0.3, 1);
        let group = vigilance_fraction(0.8, 0.3, 10);
        assert!(solo > group);
    }

    #[test]
    fn hunger_reduces_vigilance() {
        let sated = vigilance_fraction(0.8, 0.1, 1);
        let hungry = vigilance_fraction(0.8, 0.9, 1);
        assert!(sated > hungry);
    }

    #[test]
    fn serde_roundtrip_prey_item() {
        let item = PreyItem {
            energy: 5.0,
            handling_time: 2.0,
        };
        let json = serde_json::to_string(&item).unwrap();
        let item2: PreyItem = serde_json::from_str(&json).unwrap();
        assert!((item.energy - item2.energy).abs() < f32::EPSILON);
        assert!((item.handling_time - item2.handling_time).abs() < f32::EPSILON);
    }
}
