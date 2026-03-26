use serde::{Deserialize, Serialize};

/// Social role within a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SocialRole { Alpha, Beta, Subordinate, Omega, Sentinel, Scout, Worker, Queen, Drone, Solitary }

/// Position in dominance hierarchy (0.0 = bottom, 1.0 = top).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct HierarchyPosition(f32);

impl HierarchyPosition {
    #[must_use]
    pub fn new(value: f32) -> Self { Self(value.clamp(0.0, 1.0)) }
    #[must_use] #[inline]
    pub fn value(&self) -> f32 { self.0 }
    #[must_use] #[inline]
    pub fn is_dominant(&self) -> bool { self.0 > 0.7 }
    #[must_use] #[inline]
    pub fn is_subordinate(&self) -> bool { self.0 < 0.3 }

    /// Dominance contest: higher position + aggression wins.
    #[must_use]
    pub fn contest(&self, opponent: &Self, self_aggression: f32, opponent_aggression: f32) -> bool {
        (self.0 * self_aggression) > (opponent.0 * opponent_aggression)
    }
}

/// Group cohesion (0.0 = scattered, 1.0 = tight formation).
#[must_use]
pub fn group_cohesion(distances: &[f32], max_distance: f32) -> f32 {
    if distances.is_empty() || max_distance <= 0.0 { return 0.0; }
    let avg = distances.iter().sum::<f32>() / distances.len() as f32;
    (1.0 - avg / max_distance).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_is_dominant() {
        assert!(HierarchyPosition::new(0.9).is_dominant());
        assert!(!HierarchyPosition::new(0.5).is_dominant());
    }

    #[test]
    fn contest_higher_wins() {
        let alpha = HierarchyPosition::new(0.9);
        let beta = HierarchyPosition::new(0.6);
        assert!(alpha.contest(&beta, 0.8, 0.8));
    }

    #[test]
    fn aggression_can_overcome_rank() {
        let low_rank = HierarchyPosition::new(0.4);
        let high_rank = HierarchyPosition::new(0.6);
        // Very aggressive low-rank vs timid high-rank
        assert!(low_rank.contest(&high_rank, 1.0, 0.3));
    }

    #[test]
    fn tight_group_high_cohesion() {
        let cohesion = group_cohesion(&[1.0, 2.0, 1.5], 100.0);
        assert!(cohesion > 0.9);
    }

    #[test]
    fn scattered_group_low_cohesion() {
        let cohesion = group_cohesion(&[80.0, 90.0, 95.0], 100.0);
        assert!(cohesion < 0.2);
    }
}
