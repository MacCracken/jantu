//! Genetic trait inheritance — heritable behavior parameters.
//!
//! Models quantitative genetics for behavioral traits: heritability,
//! mutation, crossover, and phenotypic expression. Traits are continuous
//! values influenced by multiple genes (polygenic) with environmental noise.

use serde::{Deserialize, Serialize};

/// A heritable behavioral trait with genetic and environmental components.
///
/// ```
/// use jantu::genetics::HeritableTrait;
///
/// let t = HeritableTrait::new(0.8, 0.6);
/// // phenotype = genotype * heritability + environment * (1 - heritability)
/// let phenotype = t.phenotype(0.2);
/// assert!((phenotype - 0.56).abs() < 0.01);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeritableTrait {
    /// Genotypic value (0.0-1.0). The genetic component.
    pub genotype: f32,
    /// Heritability (0.0-1.0). Proportion of phenotypic variance due to genetics.
    /// h² in quantitative genetics. Higher = more faithfully inherited.
    pub heritability: f32,
}

impl HeritableTrait {
    /// Create a new heritable trait.
    #[must_use]
    pub fn new(genotype: f32, heritability: f32) -> Self {
        Self {
            genotype: genotype.clamp(0.0, 1.0),
            heritability: heritability.clamp(0.0, 1.0),
        }
    }

    /// Express the phenotype given an environmental influence.
    ///
    /// `environment`: environmental contribution to the trait (0.0-1.0).
    /// Returns the expressed phenotype (0.0-1.0).
    #[must_use]
    pub fn phenotype(&self, environment: f32) -> f32 {
        let environment = environment.clamp(0.0, 1.0);
        let genetic_component = self.genotype * self.heritability;
        let environmental_component = environment * (1.0 - self.heritability);
        (genetic_component + environmental_component).clamp(0.0, 1.0)
    }
}

/// A behavioral genotype — a set of heritable trait values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralGenome {
    /// Innate aggression tendency.
    pub aggression: HeritableTrait,
    /// Innate boldness (vs. shyness). Affects exploration and risk-taking.
    pub boldness: HeritableTrait,
    /// Innate sociability. Affects group affiliation drive.
    pub sociability: HeritableTrait,
    /// Innate activity level. Affects metabolism and movement.
    pub activity: HeritableTrait,
    /// Innate exploration tendency (neophilia vs. neophobia).
    pub exploration: HeritableTrait,
}

impl BehavioralGenome {
    /// Create a genome with default moderate values and typical heritabilities.
    #[must_use]
    pub fn default_genome() -> Self {
        Self {
            aggression: HeritableTrait::new(0.5, 0.4),
            boldness: HeritableTrait::new(0.5, 0.35),
            sociability: HeritableTrait::new(0.5, 0.3),
            activity: HeritableTrait::new(0.5, 0.45),
            exploration: HeritableTrait::new(0.5, 0.3),
        }
    }
}

/// Compute offspring genotype from two parents via midparent blending
/// with mutation.
///
/// ```
/// use jantu::genetics::inherit_trait;
///
/// let offspring = inherit_trait(0.8, 0.4, 0.0);
/// assert!((offspring - 0.6).abs() < f32::EPSILON);
///
/// // Clamped to [0.0, 1.0]
/// assert_eq!(inherit_trait(0.9, 0.9, 0.5), 1.0);
/// ```
#[must_use]
pub fn inherit_trait(parent_a: f32, parent_b: f32, mutation: f32) -> f32 {
    let midparent = (parent_a + parent_b) * 0.5;
    (midparent + mutation).clamp(0.0, 1.0)
}

/// Compute offspring genome from two parent genomes.
///
/// `mutations` provides per-trait mutation offsets [aggression, boldness,
/// sociability, activity, exploration].
#[must_use]
pub fn crossover(
    parent_a: &BehavioralGenome,
    parent_b: &BehavioralGenome,
    mutations: &[f32; 5],
) -> BehavioralGenome {
    BehavioralGenome {
        aggression: HeritableTrait::new(
            inherit_trait(
                parent_a.aggression.genotype,
                parent_b.aggression.genotype,
                mutations[0],
            ),
            (parent_a.aggression.heritability + parent_b.aggression.heritability) * 0.5,
        ),
        boldness: HeritableTrait::new(
            inherit_trait(
                parent_a.boldness.genotype,
                parent_b.boldness.genotype,
                mutations[1],
            ),
            (parent_a.boldness.heritability + parent_b.boldness.heritability) * 0.5,
        ),
        sociability: HeritableTrait::new(
            inherit_trait(
                parent_a.sociability.genotype,
                parent_b.sociability.genotype,
                mutations[2],
            ),
            (parent_a.sociability.heritability + parent_b.sociability.heritability) * 0.5,
        ),
        activity: HeritableTrait::new(
            inherit_trait(
                parent_a.activity.genotype,
                parent_b.activity.genotype,
                mutations[3],
            ),
            (parent_a.activity.heritability + parent_b.activity.heritability) * 0.5,
        ),
        exploration: HeritableTrait::new(
            inherit_trait(
                parent_a.exploration.genotype,
                parent_b.exploration.genotype,
                mutations[4],
            ),
            (parent_a.exploration.heritability + parent_b.exploration.heritability) * 0.5,
        ),
    }
}

/// Compute fitness from a genome in a given environment.
///
/// `trait_weights`: importance of each trait [aggression, boldness, sociability, activity, exploration].
/// `environment`: environmental conditions for each trait (0.0-1.0).
///
/// Returns a fitness score (0.0-1.0).
#[must_use]
pub fn genome_fitness(
    genome: &BehavioralGenome,
    trait_weights: &[f32; 5],
    environment: &[f32; 5],
) -> f32 {
    let traits = [
        &genome.aggression,
        &genome.boldness,
        &genome.sociability,
        &genome.activity,
        &genome.exploration,
    ];

    let total_weight: f32 = trait_weights.iter().sum();
    if total_weight <= 0.0 {
        return 0.0;
    }

    let weighted_sum: f32 = traits
        .iter()
        .zip(trait_weights.iter())
        .zip(environment.iter())
        .map(|((t, &w), &e)| t.phenotype(e) * w)
        .sum();

    (weighted_sum / total_weight).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phenotype_blends_genetics_and_environment() {
        let t = HeritableTrait::new(0.8, 0.6);
        let phenotype = t.phenotype(0.2);
        // 0.8 * 0.6 + 0.2 * 0.4 = 0.48 + 0.08 = 0.56
        assert!((phenotype - 0.56).abs() < 0.01);
    }

    #[test]
    fn high_heritability_tracks_genotype() {
        let t = HeritableTrait::new(0.9, 1.0);
        let phenotype = t.phenotype(0.1);
        assert!((phenotype - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn zero_heritability_tracks_environment() {
        let t = HeritableTrait::new(0.9, 0.0);
        let phenotype = t.phenotype(0.3);
        assert!((phenotype - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn inheritance_midparent() {
        let offspring = inherit_trait(0.8, 0.4, 0.0);
        assert!((offspring - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn inheritance_with_mutation() {
        let offspring = inherit_trait(0.5, 0.5, 0.1);
        assert!((offspring - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn inheritance_clamps() {
        assert_eq!(inherit_trait(0.9, 0.9, 0.5), 1.0);
        assert_eq!(inherit_trait(0.1, 0.1, -0.5), 0.0);
    }

    #[test]
    fn crossover_produces_valid_genome() {
        let a = BehavioralGenome::default_genome();
        let b = BehavioralGenome::default_genome();
        let offspring = crossover(&a, &b, &[0.0; 5]);
        assert!((0.0..=1.0).contains(&offspring.aggression.genotype));
        assert!((0.0..=1.0).contains(&offspring.boldness.genotype));
    }

    #[test]
    fn fitness_responds_to_environment() {
        let genome = BehavioralGenome::default_genome();
        let weights = [1.0, 1.0, 1.0, 1.0, 1.0];
        let good_env = [0.8, 0.8, 0.8, 0.8, 0.8];
        let bad_env = [0.1, 0.1, 0.1, 0.1, 0.1];
        let fit_good = genome_fitness(&genome, &weights, &good_env);
        let fit_bad = genome_fitness(&genome, &weights, &bad_env);
        assert!(fit_good > fit_bad, "good env should yield higher fitness");
    }

    #[test]
    fn fitness_bounded() {
        let genome = BehavioralGenome::default_genome();
        let weights = [1.0; 5];
        let env = [0.5; 5];
        let f = genome_fitness(&genome, &weights, &env);
        assert!((0.0..=1.0).contains(&f));
    }

    #[test]
    fn serde_roundtrip_heritable_trait() {
        let t = HeritableTrait::new(0.7, 0.4);
        let json = serde_json::to_string(&t).unwrap();
        let t2: HeritableTrait = serde_json::from_str(&json).unwrap();
        assert!((t.genotype - t2.genotype).abs() < f32::EPSILON);
        assert!((t.heritability - t2.heritability).abs() < f32::EPSILON);
    }

    #[test]
    fn serde_roundtrip_behavioral_genome() {
        let g = BehavioralGenome::default_genome();
        let json = serde_json::to_string(&g).unwrap();
        let g2: BehavioralGenome = serde_json::from_str(&json).unwrap();
        assert!((g.aggression.genotype - g2.aggression.genotype).abs() < f32::EPSILON);
        assert!((g.boldness.heritability - g2.boldness.heritability).abs() < f32::EPSILON);
    }
}
