//! Social dynamics example: emotional contagion, signaling, and memory
//! in a group of creatures.

use jantu::contagion::{
    EmotionalState, Susceptibility, aggregate_pressure, contagion_transfer, emotional_influence,
};
use jantu::memory::{
    MemoryTrace, MemoryType, neophobia_modifier, social_recognition, spatial_reliability,
};
use jantu::signals::{
    Signal, SignalFunction, SignalModality, detection_probability, receiver_response, signal_cost,
    signal_range,
};

fn main() {
    // --- Alarm call propagation ---
    println!("=== Alarm Call Propagation ===\n");

    let alarm = Signal::new(SignalModality::Acoustic, SignalFunction::Alarm, 0.9);
    let range = signal_range(alarm.modality, alarm.intensity);
    let cost = signal_cost(alarm.intensity, alarm.honesty);
    println!("Alarm call: range={range:.2}, cost={cost:.3} (honest signal)");

    // Detection at various distances
    for dist in [5.0, 25.0, 50.0, 80.0] {
        let det = detection_probability(alarm.intensity, dist, 100.0, 0.2);
        println!("  Distance {dist:.0}m, noise 0.2: detection probability {det:.2}");
    }

    // Receiver response: known vs unknown caller
    let friend = receiver_response(1.0, 0.9, 0.8);
    let stranger = receiver_response(1.0, 0.1, 0.8);
    println!("\nResponse to alarm from friend: {friend:.2}");
    println!("Response to alarm from stranger: {stranger:.2}");

    // --- Emotional contagion through the group ---
    println!("\n=== Emotional Contagion ===\n");

    // Alpha spots a predator — fear radiates outward
    let alpha_fear = 0.9;
    let alpha_rank = 0.95;

    let nearby = emotional_influence(alpha_fear, alpha_rank, 5.0, 100.0);
    let mid = emotional_influence(alpha_fear, alpha_rank, 30.0, 100.0);
    let far = emotional_influence(alpha_fear, alpha_rank, 70.0, 100.0);

    println!("Alpha (rank {alpha_rank}) fear {alpha_fear}:");
    println!("  Nearby (5m):  influence {nearby:.3}");
    println!("  Middle (30m): influence {mid:.3}");
    println!("  Far (70m):    influence {far:.3}");

    // Susceptibility varies by rank
    let omega = Susceptibility::new(0.8, 0.1, 0.5);
    let beta = Susceptibility::new(0.8, 0.7, 0.5);
    println!(
        "\nOmega susceptibility: {:.2} (low rank = more susceptible)",
        omega.effective()
    );
    println!(
        "Beta susceptibility:  {:.2} (high rank = less susceptible)",
        beta.effective()
    );

    // Transfer: omega already fearful gets amplified
    let omega_shift = contagion_transfer(nearby, omega.effective(), true);
    let beta_shift = contagion_transfer(nearby, beta.effective(), false);
    println!("\nFear transfer to omega (already fearful): {omega_shift:.3}");
    println!("Fear transfer to beta (calm):             {beta_shift:.3}");

    // Aggregate group pressure
    let influences = [
        (0.8, EmotionalState::Fear),
        (0.6, EmotionalState::Fear),
        (0.3, EmotionalState::Aggression),
        (0.2, EmotionalState::Calm),
    ];
    if let Some((state, total)) = aggregate_pressure(&influences) {
        println!("\nGroup emotional state: {state:?} (total pressure: {total:.2})");
    }

    // --- Memory and recognition ---
    println!("\n=== Memory & Recognition ===\n");

    // Building familiarity with a food source
    let mut food = MemoryTrace::new(MemoryType::FoodSource, 0.6, 0.7);
    println!(
        "New food memory: strength={:.2}, valence={:.2}, appetitive={}",
        food.strength,
        food.valence,
        food.is_appetitive()
    );

    food.reinforce(0.5);
    food.reinforce(0.3);
    println!(
        "After 2 reinforcements: strength={:.2}, count={}",
        food.strength, food.reinforcement_count
    );

    // Neophobia fades with familiarity
    let novel_fear = neophobia_modifier(0.0);
    let familiar_fear = neophobia_modifier(food.strength);
    println!("\nNeophobia modifier (novel): {novel_fear:.2}");
    println!("Neophobia modifier (familiar): {familiar_fear:.2}");

    // Spatial memory reliability
    let stable = spatial_reliability(0.8, 0.9);
    let volatile = spatial_reliability(0.8, 0.2);
    println!("\nSpatial reliability (stable env):   {stable:.2}");
    println!("Spatial reliability (volatile env):  {volatile:.2}");

    // Social recognition over encounters
    println!("\nSocial recognition buildup:");
    for encounters in [1, 3, 5, 10, 20] {
        let recent = social_recognition(encounters, 0.0);
        let old = social_recognition(encounters, 50.0);
        println!("  {encounters:2} encounters: recent={recent:.2}, 50 ticks ago={old:.2}");
    }
}
