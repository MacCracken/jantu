use serde::{Deserialize, Serialize};

/// Pack hunting coordination phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HuntPhase { Searching, Stalking, Chasing, Encircling, Attack, Feeding }

/// Pack hunting success probability based on pack size and prey size ratio.
#[must_use]
pub fn hunt_success_probability(pack_size: u32, prey_difficulty: f32) -> f32 {
    // Sigmoid: more pack members → higher success, but diminishing returns
    let size_factor = 1.0 - (-0.3 * pack_size as f32).exp();
    (size_factor / (prey_difficulty + 0.01)).clamp(0.0, 0.95)
}

/// Food share based on dominance hierarchy.
#[must_use]
pub fn food_share(rank: f32, pack_size: u32) -> f32 {
    if pack_size == 0 { return 0.0; }
    // Higher rank gets disproportionately more
    let total_weight: f32 = (0..pack_size).map(|i| (i as f32 / pack_size as f32 + 0.1)).sum();
    (rank + 0.1) / total_weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_pack_better_success() {
        let small = hunt_success_probability(2, 1.0);
        let large = hunt_success_probability(8, 1.0);
        assert!(large > small);
    }

    #[test]
    fn harder_prey_lower_success() {
        let easy = hunt_success_probability(5, 0.5);
        let hard = hunt_success_probability(5, 2.0);
        assert!(easy > hard);
    }

    #[test]
    fn alpha_gets_more_food() {
        let alpha_share = food_share(0.9, 5);
        let omega_share = food_share(0.1, 5);
        assert!(alpha_share > omega_share);
    }

    #[test]
    fn success_capped_at_95() {
        let prob = hunt_success_probability(100, 0.1);
        assert!(prob <= 0.95);
    }
}
