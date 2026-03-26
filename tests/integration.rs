use jantu::instinct::{DriveLevel, Instinct, InstinctType, dominant_instinct};
use jantu::lifecycle::{basal_metabolic_rate, estimated_lifespan_years, heart_rate_bpm};
use jantu::pack::{food_share, hunt_success_probability};
use jantu::social::{HierarchyPosition, group_cohesion};
use jantu::survival::{ThreatResponse, select_threat_response};
use jantu::swarm::{Pheromone, PheromoneType, path_selection_probability, quorum_reached};
use jantu::territory::{TerritoryMark, territorial_response};

/// A creature with high fear drive should select fear as dominant instinct
/// and choose flight when it has speed advantage.
#[test]
fn fearful_fast_creature_flees() {
    let mut fear = Instinct::new(InstinctType::Fear);
    fear.drive = DriveLevel::new(0.4);
    fear.update_priority();

    let mut hunger = Instinct::new(InstinctType::Hunger);
    hunger.drive = DriveLevel::new(0.4);
    hunger.update_priority();

    let instincts = [fear, hunger];
    let dom = dominant_instinct(&instincts).expect("non-empty slice");
    assert_eq!(dom.instinct_type, InstinctType::Fear);

    // Fast, small, non-aggressive creature → flight
    let response = select_threat_response(0.1, 0.9, 0.4, 0.3);
    assert_eq!(response, ThreatResponse::Flight);
}

/// An alpha wolf hunting: high aggression leads to fight response,
/// and large pack size gives good hunt success probability.
#[test]
fn alpha_predator_hunts_successfully() {
    // Alpha fights threats
    let response = select_threat_response(0.9, 0.5, 1.5, 0.9);
    assert_eq!(response, ThreatResponse::Fight);

    // Large pack has good hunt success against moderate prey
    let success = hunt_success_probability(8, 1.0);
    assert!(success > 0.5, "pack of 8 should have >50% success");

    // Alpha gets the most food
    let alpha_share = food_share(0.9, 8);
    let omega_share = food_share(0.1, 8);
    assert!(alpha_share > omega_share);
}

/// Territory marking decays over time and affects aggression.
#[test]
fn territory_lifecycle() {
    let mut mark = TerritoryMark {
        position: [10.0, 0.0, 10.0],
        strength: 1.0,
        owner_id: 1,
    };
    assert!(mark.is_active());

    // Decay over several ticks
    for _ in 0..20 {
        mark.decay(0.05);
    }
    assert!(!mark.is_active(), "mark should expire after enough decay");

    // Stronger territory holder is more aggressive
    let strong = territorial_response(1.0, 0.3, 0.8);
    let weak = territorial_response(0.3, 1.0, 0.8);
    assert!(strong > weak);
}

/// Swarm intelligence: pheromone trails guide path selection,
/// and quorum sensing triggers collective decisions.
#[test]
fn swarm_decision_making() {
    let mut trail = Pheromone {
        position: [5.0, 0.0, 5.0],
        strength: 1.0,
        pheromone_type: PheromoneType::Trail,
    };
    assert!(trail.is_detectable());

    // Strongest trail gets highest selection probability
    let prob = path_selection_probability(5.0, &[1.0, 2.0, 5.0, 2.0]);
    assert!(prob > 0.4, "strongest trail should dominate");

    // Quorum needs threshold
    assert!(!quorum_reached(3, 10, 0.5), "30% < 50% threshold");
    assert!(quorum_reached(6, 10, 0.5), "60% >= 50% threshold");

    // Pheromone evaporates
    trail.evaporate(0.95);
    assert!(trail.strength < 0.1);
}

/// Social hierarchy determines contest outcomes and group cohesion.
#[test]
fn social_dynamics() {
    let alpha = HierarchyPosition::new(0.9);
    let omega = HierarchyPosition::new(0.2);

    assert!(alpha.is_dominant());
    assert!(omega.is_subordinate());
    assert!(alpha.contest(&omega, 0.8, 0.8));

    // But high aggression can overcome low rank
    assert!(omega.contest(&alpha, 1.0, 0.1));

    // Tight group = high cohesion
    let tight = group_cohesion(&[2.0, 3.0, 1.5], 100.0);
    let scattered = group_cohesion(&[80.0, 90.0, 95.0], 100.0);
    assert!(tight > scattered);
}

/// Kleiber's law: metabolic scaling across body sizes is consistent.
#[test]
fn allometric_scaling_consistency() {
    // Mouse (20g) vs elephant (5000kg)
    let mouse_bmr = basal_metabolic_rate(0.02, 70.0);
    let elephant_bmr = basal_metabolic_rate(5000.0, 70.0);
    assert!(elephant_bmr > mouse_bmr);

    // But per-kg metabolic rate is HIGHER for smaller animals
    let mouse_per_kg = mouse_bmr / 0.02;
    let elephant_per_kg = elephant_bmr / 5000.0;
    assert!(mouse_per_kg > elephant_per_kg);

    // Smaller animals: faster heart, shorter life
    let mouse_hr = heart_rate_bpm(0.02, 200.0);
    let elephant_hr = heart_rate_bpm(5000.0, 200.0);
    assert!(mouse_hr > elephant_hr);

    let mouse_life = estimated_lifespan_years(0.02, 10.0);
    let elephant_life = estimated_lifespan_years(5000.0, 10.0);
    assert!(elephant_life > mouse_life);
}
