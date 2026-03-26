use jantu::instinct::{DriveLevel, Instinct, InstinctType, dominant_instinct};
use jantu::lifecycle::{basal_metabolic_rate, estimated_lifespan_years, heart_rate_bpm};
use jantu::pack::hunt_success_probability;
use jantu::social::{HierarchyPosition, group_cohesion};
use jantu::survival::select_threat_response;

fn main() {
    // Model a wolf's instincts
    let mut hunger = Instinct::new(InstinctType::Hunger);
    hunger.drive = DriveLevel::new(0.7);
    hunger.update_priority();

    let mut fear = Instinct::new(InstinctType::Fear);
    fear.drive = DriveLevel::new(0.2);
    fear.update_priority();

    let mut social = Instinct::new(InstinctType::Social);
    social.drive = DriveLevel::new(0.5);
    social.update_priority();

    let instincts = [hunger, fear, social];
    if let Some(dominant) = dominant_instinct(&instincts) {
        println!(
            "Dominant instinct: {:?} (priority: {:.2})",
            dominant.instinct_type, dominant.priority
        );
    }

    // Threat response: aggressive alpha wolf
    let response = select_threat_response(0.8, 0.6, 1.3, 0.9);
    println!("Threat response: {response:?}");

    // Pack hunting
    let solo = hunt_success_probability(1, 1.0);
    let pack = hunt_success_probability(6, 1.0);
    println!(
        "Hunt success — solo: {:.0}%, pack of 6: {:.0}%",
        solo * 100.0,
        pack * 100.0
    );

    // Social hierarchy
    let alpha = HierarchyPosition::new(0.95);
    let beta = HierarchyPosition::new(0.6);
    println!(
        "Alpha vs beta contest: {}",
        if alpha.contest(&beta, 0.8, 0.8) {
            "alpha wins"
        } else {
            "beta wins"
        }
    );

    // Group cohesion
    let cohesion = group_cohesion(&[5.0, 8.0, 3.0, 6.0], 100.0);
    println!("Pack cohesion: {cohesion:.2}");

    // Allometric scaling (wolf ~40kg)
    let bmr = basal_metabolic_rate(40.0, 70.0);
    let lifespan = estimated_lifespan_years(40.0, 10.0);
    let hr = heart_rate_bpm(40.0, 200.0);
    println!("Wolf (40kg) — BMR: {bmr:.1}, lifespan: {lifespan:.1}y, heart rate: {hr:.0} bpm");
}
