//! Mate selection and courtship behaviors.
//!
//! Models fitness evaluation, sexual selection, and courtship dynamics.
//! Based on Zahavi's handicap principle, Fisher's runaway selection,
//! and good-genes theory.

use serde::{Deserialize, Serialize};

/// Mating system classification.
///
/// # Examples
///
/// ```
/// use jantu::mating::MatingSystem;
///
/// let system = MatingSystem::Monogamous;
/// assert_ne!(system, MatingSystem::Lek);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MatingSystem {
    /// One male, one female (e.g., wolves, albatrosses).
    Monogamous,
    /// One male, multiple females (e.g., elk, gorillas).
    Polygynous,
    /// One female, multiple males (e.g., jacanas, some pipefish).
    Polyandrous,
    /// Multiple partners for both sexes (e.g., bonobos).
    Promiscuous,
    /// Males display at a communal site; females choose (e.g., sage grouse).
    Lek,
}

/// Courtship display phase.
///
/// # Examples
///
/// ```
/// use jantu::mating::CourtshipPhase;
///
/// let phase = CourtshipPhase::Displaying;
/// assert_ne!(phase, CourtshipPhase::Rejected);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CourtshipPhase {
    /// Searching for potential mates.
    Searching,
    /// Displaying to attract attention.
    Displaying,
    /// Actively courting a specific individual.
    Courting,
    /// Mutual assessment (both parties evaluate).
    Assessment,
    /// Pair bond formed.
    Bonded,
    /// Rejected — courtship failed.
    Rejected,
}

/// Fitness traits relevant to mate selection.
///
/// # Examples
///
/// ```
/// use jantu::mating::FitnessTraits;
///
/// let traits = FitnessTraits {
///     condition: 0.9, display_quality: 0.8,
///     territory_quality: 0.7, genetic_quality: 0.6, vigor: 0.8,
/// };
/// assert!(traits.attractiveness() > 0.5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessTraits {
    /// Physical condition / body quality (0.0-1.0).
    pub condition: f32,
    /// Display quality — song, plumage, dance (0.0-1.0).
    pub display_quality: f32,
    /// Territory quality held by this individual (0.0-1.0).
    pub territory_quality: f32,
    /// Genetic compatibility/diversity indicator (0.0-1.0).
    pub genetic_quality: f32,
    /// Age-related vigor (0.0-1.0, peaks in prime).
    pub vigor: f32,
}

impl FitnessTraits {
    /// Compute the overall mate attractiveness score.
    ///
    /// Weights reflect typical female choice priorities:
    /// display > condition > territory > genetics > vigor.
    #[must_use]
    pub fn attractiveness(&self) -> f32 {
        let score = self.display_quality * 0.30
            + self.condition * 0.25
            + self.territory_quality * 0.20
            + self.genetic_quality * 0.15
            + self.vigor * 0.10;
        score.clamp(0.0, 1.0)
    }
}

/// Evaluate mate choice: will the chooser accept this candidate?
///
/// ```
/// use jantu::mating::{FitnessTraits, mate_acceptance};
///
/// let good = FitnessTraits {
///     condition: 0.9, display_quality: 0.85,
///     territory_quality: 0.8, genetic_quality: 0.7, vigor: 0.8,
/// };
/// let prob = mate_acceptance(&good, 0.3, 2);
/// assert!(prob > 0.5);
/// ```
///
/// - `candidate`: fitness traits of the potential mate
/// - `chooser_threshold`: minimum attractiveness the chooser will accept (0.0-1.0)
/// - `competition`: number of alternative candidates available (more choice → pickier)
///
/// Returns acceptance probability (0.0-1.0).
#[must_use]
pub fn mate_acceptance(candidate: &FitnessTraits, chooser_threshold: f32, competition: u32) -> f32 {
    let attractiveness = candidate.attractiveness();
    let threshold = chooser_threshold.clamp(0.0, 1.0);

    // More competition raises the effective threshold
    let competition_factor = 1.0 + (competition as f32).ln_1p() * 0.15;
    let effective_threshold = (threshold * competition_factor).clamp(0.0, 1.0);

    if attractiveness < effective_threshold {
        return 0.0;
    }

    // Probability increases with how far above threshold
    let excess = attractiveness - effective_threshold;
    let max_excess = 1.0 - effective_threshold;
    if max_excess <= 0.0 {
        return 1.0;
    }
    (excess / max_excess).clamp(0.0, 1.0)
}

/// Compute courtship display cost (Zahavi's handicap principle).
///
/// More elaborate displays are costlier but signal higher quality.
/// Returns the energy cost as a fraction of reserves (0.0-1.0).
///
/// ```
/// use jantu::mating::display_cost;
///
/// let strong = display_cost(0.8, 0.9);
/// let weak = display_cost(0.8, 0.3);
/// assert!(weak > strong); // poor condition makes displays costlier
/// ```
#[must_use]
pub fn display_cost(display_intensity: f32, body_condition: f32) -> f32 {
    let display_intensity = display_intensity.clamp(0.0, 1.0);
    let body_condition = body_condition.clamp(0.0, 1.0);

    // Higher intensity costs more; poor condition makes it proportionally costlier
    let base_cost = display_intensity * display_intensity; // quadratic cost
    let condition_penalty = 1.0 + (1.0 - body_condition) * 0.5;
    (base_cost * condition_penalty).clamp(0.0, 1.0)
}

/// Sexual selection pressure: how much does competition amplify trait expression?
///
/// `operational_sex_ratio`: ratio of competing sex to limiting sex (e.g., 3.0 = 3 males per female).
/// Higher ratio = stronger selection pressure on the competing sex.
///
/// Returns a trait amplification multiplier (1.0 = no extra pressure).
///
/// ```
/// use jantu::mating::selection_pressure;
///
/// let balanced = selection_pressure(1.0);
/// let skewed = selection_pressure(5.0);
/// assert!(skewed > balanced);
/// ```
#[must_use]
pub fn selection_pressure(operational_sex_ratio: f32) -> f32 {
    if operational_sex_ratio <= 0.0 {
        return 1.0;
    }
    // Logarithmic scaling: pressure increases with imbalance but with diminishing returns
    1.0 + operational_sex_ratio.ln_1p() * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_mate() -> FitnessTraits {
        FitnessTraits {
            condition: 0.9,
            display_quality: 0.85,
            territory_quality: 0.8,
            genetic_quality: 0.7,
            vigor: 0.8,
        }
    }

    fn poor_mate() -> FitnessTraits {
        FitnessTraits {
            condition: 0.2,
            display_quality: 0.1,
            territory_quality: 0.1,
            genetic_quality: 0.3,
            vigor: 0.2,
        }
    }

    #[test]
    fn good_mate_more_attractive() {
        assert!(good_mate().attractiveness() > poor_mate().attractiveness());
    }

    #[test]
    fn attractiveness_bounded() {
        let a = good_mate().attractiveness();
        assert!((0.0..=1.0).contains(&a));
    }

    #[test]
    fn high_quality_accepted() {
        let prob = mate_acceptance(&good_mate(), 0.3, 2);
        assert!(prob > 0.5, "good mate should be accepted: {prob}");
    }

    #[test]
    fn low_quality_rejected() {
        let prob = mate_acceptance(&poor_mate(), 0.5, 5);
        assert_eq!(prob, 0.0, "poor mate should be rejected");
    }

    #[test]
    fn more_competition_pickier() {
        let few = mate_acceptance(&good_mate(), 0.5, 1);
        let many = mate_acceptance(&good_mate(), 0.5, 20);
        assert!(
            few >= many,
            "more competition should not increase acceptance: few={few}, many={many}"
        );
    }

    #[test]
    fn display_cost_increases_with_intensity() {
        let low = display_cost(0.2, 0.8);
        let high = display_cost(0.9, 0.8);
        assert!(high > low);
    }

    #[test]
    fn poor_condition_costlier_display() {
        let healthy = display_cost(0.7, 0.9);
        let sickly = display_cost(0.7, 0.2);
        assert!(sickly > healthy);
    }

    #[test]
    fn balanced_sex_ratio_low_pressure() {
        let balanced = selection_pressure(1.0);
        let skewed = selection_pressure(5.0);
        assert!(skewed > balanced);
    }

    #[test]
    fn zero_ratio_baseline() {
        assert!((selection_pressure(0.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn serde_roundtrip_mating_system() {
        for s in [
            MatingSystem::Monogamous,
            MatingSystem::Polygynous,
            MatingSystem::Polyandrous,
            MatingSystem::Promiscuous,
            MatingSystem::Lek,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let s2: MatingSystem = serde_json::from_str(&json).unwrap();
            assert_eq!(s, s2);
        }
    }

    #[test]
    fn serde_roundtrip_courtship_phase() {
        for p in [
            CourtshipPhase::Searching,
            CourtshipPhase::Displaying,
            CourtshipPhase::Courting,
            CourtshipPhase::Assessment,
            CourtshipPhase::Bonded,
            CourtshipPhase::Rejected,
        ] {
            let json = serde_json::to_string(&p).unwrap();
            let p2: CourtshipPhase = serde_json::from_str(&json).unwrap();
            assert_eq!(p, p2);
        }
    }

    #[test]
    fn serde_roundtrip_fitness_traits() {
        let t = good_mate();
        let json = serde_json::to_string(&t).unwrap();
        let t2: FitnessTraits = serde_json::from_str(&json).unwrap();
        assert!((t.condition - t2.condition).abs() < f32::EPSILON);
        assert!((t.display_quality - t2.display_quality).abs() < f32::EPSILON);
    }
}
