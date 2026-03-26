//! Kin selection and inclusive fitness (Hamilton 1964).
//!
//! Models altruistic behavior between related individuals.
//! Hamilton's rule: an altruistic act evolves when `r * B > C`
//! (relatedness × benefit to recipient > cost to actor).

use serde::{Deserialize, Serialize};

/// Standard relatedness coefficients.
///
/// # Examples
///
/// ```
/// use jantu::kin::Relatedness;
///
/// assert!((Relatedness::FULL_SIBLING - 0.5).abs() < f32::EPSILON);
/// assert!((Relatedness::PARENT_OFFSPRING - 0.5).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Relatedness;

impl Relatedness {
    /// Clones / identical twins.
    pub const CLONE: f32 = 1.0;
    /// Parent to offspring, or full siblings.
    pub const PARENT_OFFSPRING: f32 = 0.5;
    /// Full siblings (diploid, sexual reproduction).
    pub const FULL_SIBLING: f32 = 0.5;
    /// Half siblings.
    pub const HALF_SIBLING: f32 = 0.25;
    /// Grandparent to grandchild, or uncle/aunt to nephew/niece.
    pub const GRANDPARENT: f32 = 0.25;
    /// First cousins.
    pub const FIRST_COUSIN: f32 = 0.125;
    /// Unrelated individuals.
    pub const UNRELATED: f32 = 0.0;
    /// Haplodiploid sisters (e.g., Hymenoptera: ants, bees, wasps).
    pub const HAPLODIPLOID_SISTER: f32 = 0.75;
}

/// Hamilton's rule: should the actor perform an altruistic act?
///
/// Returns `true` when `r * benefit > cost` — the inclusive fitness
/// gain from helping the relative exceeds the direct fitness cost.
///
/// `relatedness`: coefficient of relatedness (0.0–1.0).
/// `benefit`: fitness gain to the recipient.
/// `cost`: fitness cost to the actor.
///
/// ```
/// use jantu::kin::{hamiltons_rule, Relatedness};
///
/// // Help a full sibling at moderate cost
/// assert!(hamiltons_rule(Relatedness::FULL_SIBLING, 10.0, 3.0)); // 0.5 * 10 > 3
/// assert!(!hamiltons_rule(Relatedness::FIRST_COUSIN, 10.0, 3.0)); // 0.125 * 10 < 3
/// ```
#[must_use]
pub fn hamiltons_rule(relatedness: f32, benefit: f32, cost: f32) -> bool {
    relatedness * benefit > cost
}

/// Compute inclusive fitness from direct and indirect components.
///
/// `direct_fitness`: fitness from the individual's own reproduction.
/// `indirect_fitness`: total fitness benefits provided to relatives.
/// `average_relatedness`: mean relatedness to those helped.
///
/// Returns total inclusive fitness.
///
/// ```
/// use jantu::kin::inclusive_fitness;
///
/// let total = inclusive_fitness(5.0, 8.0, 0.5);
/// assert!((total - 9.0).abs() < f32::EPSILON); // 5 + 8 * 0.5
/// ```
#[must_use]
pub fn inclusive_fitness(
    direct_fitness: f32,
    indirect_fitness: f32,
    average_relatedness: f32,
) -> f32 {
    direct_fitness + indirect_fitness * average_relatedness.clamp(0.0, 1.0)
}

/// Alarm call decision: should the creature risk its life to warn relatives?
///
/// Combines Hamilton's rule with predation risk. The creature calls if
/// the inclusive fitness benefit exceeds the survival cost.
///
/// `relatedness`: average relatedness to nearby group members.
/// `group_size`: number of relatives that would benefit.
/// `survival_reduction`: probability of being caught due to calling (0.0–1.0).
/// `predator_lethality`: probability that capture means death (0.0–1.0).
///
/// Returns `true` if the creature should call.
///
/// ```
/// use jantu::kin::should_alarm_call;
///
/// // Closely related group, moderate risk
/// assert!(should_alarm_call(0.5, 5, 0.1, 0.5));
/// // Unrelated group, high risk
/// assert!(!should_alarm_call(0.0, 5, 0.3, 0.8));
/// ```
#[must_use]
pub fn should_alarm_call(
    relatedness: f32,
    group_size: u32,
    survival_reduction: f32,
    predator_lethality: f32,
) -> bool {
    let cost = survival_reduction.clamp(0.0, 1.0) * predator_lethality.clamp(0.0, 1.0);
    let benefit_per_member = survival_reduction * 0.8; // warning saves ~80% of the risk
    let total_benefit = benefit_per_member * group_size as f32;
    hamiltons_rule(relatedness, total_benefit, cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamiltons_rule_sibling_altruism() {
        assert!(hamiltons_rule(0.5, 10.0, 3.0));
    }

    #[test]
    fn hamiltons_rule_rejects_costly() {
        assert!(!hamiltons_rule(0.5, 10.0, 6.0));
    }

    #[test]
    fn hamiltons_rule_unrelated_never() {
        assert!(!hamiltons_rule(0.0, 100.0, 0.01));
    }

    #[test]
    fn inclusive_fitness_combines() {
        let total = inclusive_fitness(5.0, 8.0, 0.5);
        assert!((total - 9.0).abs() < f32::EPSILON);
    }

    #[test]
    fn inclusive_fitness_zero_relatedness() {
        let total = inclusive_fitness(5.0, 100.0, 0.0);
        assert!((total - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn alarm_call_related_group() {
        assert!(should_alarm_call(0.5, 5, 0.1, 0.5));
    }

    #[test]
    fn alarm_call_unrelated_no() {
        assert!(!should_alarm_call(0.0, 5, 0.3, 0.8));
    }

    #[test]
    fn haplodiploid_super_sisters() {
        // Haplodiploid sisters (r=0.75) more altruistic than diploid (r=0.5)
        assert!(hamiltons_rule(Relatedness::HAPLODIPLOID_SISTER, 4.0, 2.5));
        assert!(!hamiltons_rule(Relatedness::FULL_SIBLING, 4.0, 2.5));
    }

    #[test]
    fn serde_roundtrip_relatedness() {
        let r = Relatedness;
        let json = serde_json::to_string(&r).unwrap();
        let _r2: Relatedness = serde_json::from_str(&json).unwrap();
    }
}
