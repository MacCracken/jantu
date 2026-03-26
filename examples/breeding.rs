//! Genetic inheritance and mate selection example.
//! Demonstrates breeding two creatures and evaluating offspring fitness.

use jantu::genetics::{BehavioralGenome, HeritableTrait, crossover, genome_fitness, inherit_trait};
use jantu::mating::{FitnessTraits, display_cost, mate_acceptance, selection_pressure};

fn main() {
    // --- Mate selection ---
    println!("=== Mate Selection ===\n");

    let strong_male = FitnessTraits {
        condition: 0.9,
        display_quality: 0.85,
        territory_quality: 0.8,
        genetic_quality: 0.7,
        vigor: 0.8,
    };

    let weak_male = FitnessTraits {
        condition: 0.3,
        display_quality: 0.2,
        territory_quality: 0.1,
        genetic_quality: 0.4,
        vigor: 0.3,
    };

    println!(
        "Strong male attractiveness: {:.2}",
        strong_male.attractiveness()
    );
    println!(
        "Weak male attractiveness:   {:.2}",
        weak_male.attractiveness()
    );

    // Female choosiness increases with competition
    let threshold = 0.3;
    for competitors in [1, 3, 5, 10] {
        let accept_strong = mate_acceptance(&strong_male, threshold, competitors);
        let accept_weak = mate_acceptance(&weak_male, threshold, competitors);
        println!(
            "\n{competitors} competitors: strong accepted={accept_strong:.2}, weak accepted={accept_weak:.2}"
        );
    }

    // Display cost: handicap principle
    println!("\n=== Courtship Display Costs ===\n");
    for intensity in [0.3, 0.6, 0.9] {
        let cost_fit = display_cost(intensity, 0.9);
        let cost_weak = display_cost(intensity, 0.3);
        println!(
            "Intensity {intensity:.1}: fit male cost={cost_fit:.3}, weak male cost={cost_weak:.3}"
        );
    }

    // Sexual selection pressure from sex ratio
    println!("\n=== Selection Pressure ===\n");
    for ratio in [1.0, 2.0, 5.0, 10.0] {
        let pressure = selection_pressure(ratio);
        println!("Sex ratio {ratio:.0}:1 → trait amplification x{pressure:.2}");
    }

    // --- Genetic inheritance ---
    println!("\n=== Breeding ===\n");

    // Create two parent genomes with distinct personalities
    let bold_parent = BehavioralGenome {
        aggression: HeritableTrait::new(0.7, 0.4),
        boldness: HeritableTrait::new(0.9, 0.35),
        sociability: HeritableTrait::new(0.4, 0.3),
        activity: HeritableTrait::new(0.8, 0.45),
        exploration: HeritableTrait::new(0.8, 0.3),
    };

    let timid_parent = BehavioralGenome {
        aggression: HeritableTrait::new(0.2, 0.4),
        boldness: HeritableTrait::new(0.1, 0.35),
        sociability: HeritableTrait::new(0.9, 0.3),
        activity: HeritableTrait::new(0.3, 0.45),
        exploration: HeritableTrait::new(0.2, 0.3),
    };

    println!(
        "Bold parent:  agg={:.1} bold={:.1} soc={:.1} act={:.1} expl={:.1}",
        bold_parent.aggression.genotype,
        bold_parent.boldness.genotype,
        bold_parent.sociability.genotype,
        bold_parent.activity.genotype,
        bold_parent.exploration.genotype
    );
    println!(
        "Timid parent: agg={:.1} bold={:.1} soc={:.1} act={:.1} expl={:.1}",
        timid_parent.aggression.genotype,
        timid_parent.boldness.genotype,
        timid_parent.sociability.genotype,
        timid_parent.activity.genotype,
        timid_parent.exploration.genotype
    );

    // Produce offspring with small mutations
    let mutations = [0.05, -0.03, 0.02, 0.0, -0.04];
    let offspring = crossover(&bold_parent, &timid_parent, &mutations);

    println!(
        "\nOffspring:    agg={:.2} bold={:.2} soc={:.2} act={:.2} expl={:.2}",
        offspring.aggression.genotype,
        offspring.boldness.genotype,
        offspring.sociability.genotype,
        offspring.activity.genotype,
        offspring.exploration.genotype
    );

    // Simple trait inheritance demo
    let trait_val = inherit_trait(0.8, 0.2, 0.0);
    println!("\nMidparent blending: 0.8 + 0.2 → offspring {trait_val:.1}");
    let mutated = inherit_trait(0.8, 0.2, 0.15);
    println!("With +0.15 mutation: 0.8 + 0.2 → offspring {mutated:.2}");

    // --- Fitness evaluation ---
    println!("\n=== Fitness in Different Environments ===\n");

    // Harsh environment: rewards aggression and activity
    let harsh_weights = [0.4, 0.3, 0.1, 0.3, 0.2];
    let harsh_env = [0.8, 0.7, 0.3, 0.9, 0.6];

    // Social environment: rewards sociability and low aggression
    let social_weights = [0.1, 0.1, 0.5, 0.2, 0.3];
    let social_env = [0.2, 0.3, 0.9, 0.4, 0.7];

    for (name, genome) in [
        ("Bold", &bold_parent),
        ("Timid", &timid_parent),
        ("Offspring", &offspring),
    ] {
        let harsh_fit = genome_fitness(genome, &harsh_weights, &harsh_env);
        let social_fit = genome_fitness(genome, &social_weights, &social_env);
        println!(
            "{name:9} — harsh env fitness: {harsh_fit:.3}, social env fitness: {social_fit:.3}"
        );
    }
}
