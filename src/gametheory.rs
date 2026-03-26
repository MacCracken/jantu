//! Evolutionary game theory — contest strategies and stable equilibria.
//!
//! Models biological conflicts using game-theoretic frameworks:
//! hawk-dove (Maynard Smith & Price 1973), bourgeois strategy,
//! war of attrition, and producer-scrounger dynamics.

use serde::{Deserialize, Serialize};

/// Contest strategy in a hawk-dove game.
///
/// # Examples
///
/// ```
/// use jantu::gametheory::ContestStrategy;
///
/// let strategy = ContestStrategy::Hawk;
/// assert_ne!(strategy, ContestStrategy::Dove);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ContestStrategy {
    /// Escalate: fight until winning or injured.
    Hawk,
    /// Display only: retreat if opponent escalates.
    Dove,
    /// Conditional: play hawk if owner, dove if intruder.
    Bourgeois,
}

/// ESS hawk frequency in a hawk-dove game.
///
/// At evolutionary equilibrium, the proportion of hawks is `V / C` when
/// `V < C` (resource value < contest cost). When `V >= C`, hawks dominate.
///
/// `resource_value`: value of the contested resource.
/// `contest_cost`: cost of injury from escalated fight.
///
/// Returns the ESS proportion of hawks (0.0–1.0).
///
/// ```
/// use jantu::gametheory::hawk_dove_ess;
///
/// let freq = hawk_dove_ess(2.0, 10.0);
/// assert!((freq - 0.2).abs() < f32::EPSILON); // V/C = 2/10
///
/// let all_hawks = hawk_dove_ess(10.0, 5.0);
/// assert!((all_hawks - 1.0).abs() < f32::EPSILON); // V >= C
/// ```
#[must_use]
pub fn hawk_dove_ess(resource_value: f32, contest_cost: f32) -> f32 {
    if contest_cost <= 0.0 || resource_value >= contest_cost {
        return 1.0;
    }
    (resource_value / contest_cost).clamp(0.0, 1.0)
}

/// Payoff for a hawk-dove encounter.
///
/// `is_hawk`: whether the focal individual plays hawk.
/// `opponent_hawk`: whether the opponent plays hawk.
/// `resource_value`: value of the contested resource.
/// `contest_cost`: cost of injury from mutual escalation.
///
/// Returns the expected payoff for the focal individual.
///
/// ```
/// use jantu::gametheory::hawk_dove_payoff;
///
/// let hawk_v_dove = hawk_dove_payoff(true, false, 10.0, 20.0);
/// assert!((hawk_v_dove - 10.0).abs() < f32::EPSILON); // hawk wins all
///
/// let dove_v_dove = hawk_dove_payoff(false, false, 10.0, 20.0);
/// assert!((dove_v_dove - 5.0).abs() < f32::EPSILON); // split resource
/// ```
#[must_use]
pub fn hawk_dove_payoff(
    is_hawk: bool,
    opponent_hawk: bool,
    resource_value: f32,
    contest_cost: f32,
) -> f32 {
    match (is_hawk, opponent_hawk) {
        (true, true) => (resource_value - contest_cost) * 0.5, // both fight, 50/50
        (true, false) => resource_value,                       // hawk wins all
        (false, true) => 0.0,                                  // dove retreats
        (false, false) => resource_value * 0.5,                // split peacefully
    }
}

/// Bourgeois strategy payoff: play hawk if owner, dove if intruder.
///
/// `is_owner`: whether the focal individual is the territory holder.
/// `resource_value`: value of the contested resource.
/// `contest_cost`: cost of injury from escalation.
///
/// Returns expected payoff assuming the opponent also plays bourgeois.
///
/// ```
/// use jantu::gametheory::bourgeois_payoff;
///
/// let owner = bourgeois_payoff(true, 10.0, 20.0);
/// assert!((owner - 10.0).abs() < f32::EPSILON); // owner wins
///
/// let intruder = bourgeois_payoff(false, 10.0, 20.0);
/// assert!((intruder - 0.0).abs() < f32::EPSILON); // intruder retreats
/// ```
#[must_use]
pub fn bourgeois_payoff(is_owner: bool, resource_value: f32, _contest_cost: f32) -> f32 {
    // Bourgeois vs bourgeois: owner always wins, intruder always retreats
    if is_owner { resource_value } else { 0.0 }
}

/// Expected contest duration in a war of attrition.
///
/// Both contestants display until one quits. The expected duration depends
/// on resource value and the weaker contestant's persistence.
///
/// `resource_value`: value of the contested resource (higher → longer contests).
/// `own_strength`: focal individual's persistence capacity (0.0–1.0).
/// `opponent_strength`: opponent's persistence capacity (0.0–1.0).
///
/// Returns expected contest duration (arbitrary time units).
///
/// ```
/// use jantu::gametheory::war_of_attrition_duration;
///
/// let even = war_of_attrition_duration(10.0, 0.5, 0.5);
/// let mismatch = war_of_attrition_duration(10.0, 0.9, 0.2);
/// assert!(even > mismatch); // even matches last longer
/// ```
#[must_use]
pub fn war_of_attrition_duration(
    resource_value: f32,
    own_strength: f32,
    opponent_strength: f32,
) -> f32 {
    let own_strength = own_strength.clamp(0.0, 1.0);
    let opponent_strength = opponent_strength.clamp(0.0, 1.0);
    // Duration proportional to the minimum persistence (weaker quits first)
    // scaled by resource value (more valuable → longer display)
    let min_persistence = own_strength.min(opponent_strength);
    resource_value * min_persistence
}

/// Producer-scrounger payoff for a food finder.
///
/// In a group, producers find food but lose a portion to scroungers.
/// The optimal scrounger frequency is frequency-dependent.
///
/// `finder_share`: fraction of food the producer keeps (0.0–1.0).
/// `group_size`: total group members.
/// `scrounger_fraction`: proportion of group that scrounges (0.0–1.0).
///
/// Returns the producer's per-find payoff (0.0–1.0 of food value).
///
/// ```
/// use jantu::gametheory::producer_payoff;
///
/// let no_scroungers = producer_payoff(0.6, 5, 0.0);
/// let many_scroungers = producer_payoff(0.6, 5, 0.8);
/// assert!(no_scroungers > many_scroungers);
/// ```
#[must_use]
pub fn producer_payoff(finder_share: f32, group_size: u32, scrounger_fraction: f32) -> f32 {
    let finder_share = finder_share.clamp(0.0, 1.0);
    let scrounger_fraction = scrounger_fraction.clamp(0.0, 1.0);
    let group_size = group_size.max(1);

    // Producer keeps finder_share; remainder split among scroungers
    let n_scroungers = (group_size as f32 * scrounger_fraction).max(0.0);
    // More scroungers → more competition for the scraps → less taken per scrounger
    // But the producer still loses (1 - finder_share) regardless
    let scrounger_tax = (1.0 - finder_share) * (n_scroungers / (n_scroungers + 1.0));
    (1.0 - scrounger_tax).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hawk_dove_ess_cheap_resource() {
        let freq = hawk_dove_ess(2.0, 10.0);
        assert!((freq - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn hawk_dove_ess_expensive_resource() {
        assert_eq!(hawk_dove_ess(10.0, 5.0), 1.0);
    }

    #[test]
    fn hawk_dove_ess_zero_cost() {
        assert_eq!(hawk_dove_ess(5.0, 0.0), 1.0);
    }

    #[test]
    fn hawk_v_hawk_costly() {
        let payoff = hawk_dove_payoff(true, true, 10.0, 20.0);
        assert!((payoff - (-5.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn hawk_v_dove_wins() {
        assert!((hawk_dove_payoff(true, false, 10.0, 20.0) - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn dove_v_dove_splits() {
        assert!((hawk_dove_payoff(false, false, 10.0, 20.0) - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn bourgeois_owner_wins() {
        assert!((bourgeois_payoff(true, 10.0, 20.0) - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn bourgeois_intruder_retreats() {
        assert_eq!(bourgeois_payoff(false, 10.0, 20.0), 0.0);
    }

    #[test]
    fn war_even_longer_than_mismatch() {
        let even = war_of_attrition_duration(10.0, 0.5, 0.5);
        let mismatch = war_of_attrition_duration(10.0, 0.9, 0.2);
        assert!(even > mismatch);
    }

    #[test]
    fn war_zero_strength_instant() {
        assert_eq!(war_of_attrition_duration(10.0, 0.0, 0.5), 0.0);
    }

    #[test]
    fn producer_no_scroungers_keeps_all() {
        let payoff = producer_payoff(0.6, 5, 0.0);
        assert!((payoff - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn producer_more_scroungers_less_payoff() {
        let few = producer_payoff(0.6, 10, 0.2);
        let many = producer_payoff(0.6, 10, 0.8);
        assert!(few > many);
    }

    #[test]
    fn serde_roundtrip_contest_strategy() {
        for s in [
            ContestStrategy::Hawk,
            ContestStrategy::Dove,
            ContestStrategy::Bourgeois,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let s2: ContestStrategy = serde_json::from_str(&json).unwrap();
            assert_eq!(s, s2);
        }
    }
}
