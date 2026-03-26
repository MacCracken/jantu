//! Predator-prey ecosystem simulation demonstrating coevolution,
//! migration, and stress dynamics.

use jantu::coevolution::{
    ArmsRaceTrait, TraitMatchup, encounter_rate, functional_response_type2, red_queen_balance,
    trait_pressure,
};
use jantu::migration::{MigrationStrategy, migration_energy_cost, migratory_urge, season_from_day};
use jantu::stress::{StressState, immune_function, stress_drive_modifier};

fn main() {
    // --- Predator-prey arms race ---
    println!("=== Predator-Prey Arms Race ===\n");

    let speed = TraitMatchup::new(ArmsRaceTrait::Speed, 0.7, 0.6);
    let camouflage = TraitMatchup::new(ArmsRaceTrait::Detection, 0.5, 0.8);

    println!(
        "Speed matchup: predator {:.1} vs prey {:.1} → predator advantage {:.2}",
        speed.predator_trait,
        speed.prey_trait,
        speed.predator_advantage()
    );
    println!(
        "Camo matchup:  predator {:.1} vs prey {:.1} → prey advantage {:.2}",
        camouflage.predator_trait,
        camouflage.prey_trait,
        camouflage.prey_advantage()
    );

    // Selection pressure on the prey to get faster
    let pressure = trait_pressure(0.6, 0.7, 0.8);
    println!("Selection pressure on prey speed: {pressure:.3}");

    // Red Queen: both evolving at same rate = stasis
    let balance = red_queen_balance(0.05, 0.05);
    println!("Red Queen balance (equal rates): {balance:.3} (near zero = stasis)");

    // Encounter rate and functional response
    let encounters = encounter_rate(2.0, 50.0, 0.3);
    let consumption = functional_response_type2(50.0, 0.3, 0.2);
    println!("\nEncounters/tick: {encounters:.1}");
    println!("Consumption rate (Holling Type II): {consumption:.2} prey/predator/tick");

    // --- Seasonal migration ---
    println!("\n=== Seasonal Migration ===\n");

    for day in [80, 170, 260, 350] {
        let season = season_from_day(day);
        let obligate = migratory_urge(day, 0.5, 0.5, MigrationStrategy::Obligate);
        let facultative = migratory_urge(day, 0.3, 0.2, MigrationStrategy::Facultative);
        println!(
            "Day {day:3} ({season:?}): obligate urge {obligate:.2}, facultative urge {facultative:.2}"
        );
    }

    let fly_cost = migration_energy_cost(0.5, true, 0.0);
    let walk_cost = migration_energy_cost(40.0, false, 0.0);
    let headwind_cost = migration_energy_cost(0.5, true, 0.8);
    println!("\nMigration energy cost:");
    println!("  Songbird (0.5kg, flying, calm):    {fly_cost:.3}");
    println!("  Wildebeest (40kg, walking, calm):   {walk_cost:.3}");
    println!("  Songbird (0.5kg, flying, headwind): {headwind_cost:.3}");

    // --- Chronic stress cascade ---
    println!("\n=== Stress Cascade ===\n");

    let mut stress = StressState::new();
    println!(
        "Fresh: acute={:.2}, chronic={:.2}, resilience={:.2}",
        stress.acute, stress.chronic, stress.resilience
    );

    // Repeated predator encounters
    for i in 1..=5 {
        stress.apply_stressor(0.6);
        let fear_mod = stress_drive_modifier(stress.chronic, true);
        let curiosity_mod = stress_drive_modifier(stress.chronic, false);
        let immune = immune_function(stress.chronic, stress.resilience);
        println!(
            "Stressor #{i}: acute={:.2}, chronic={:.2}, resilience={:.2} | fear x{fear_mod:.2}, curiosity x{curiosity_mod:.2}, immune={immune:.2}",
            stress.acute, stress.chronic, stress.resilience
        );
    }

    // Recovery in safe environment
    println!("\nRecovering in safe environment...");
    for tick in [10.0, 50.0, 100.0] {
        let mut recovering = stress.clone();
        recovering.recover(tick, 0.9);
        println!(
            "  After {tick:.0} ticks: acute={:.2}, chronic={:.2}",
            recovering.acute, recovering.chronic
        );
    }
}
