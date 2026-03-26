use serde::{Deserialize, Serialize};

/// Swarm behavior type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SwarmBehavior {
    Foraging,    // pheromone trail following (ants)
    Nesting,     // hive construction
    Swarming,    // collective migration
    Defense,     // coordinated defense response
    Recruitment, // signaling others to a resource
}

/// Pheromone deposit (ant colony optimization).
///
/// ```
/// use jantu::swarm::{Pheromone, PheromoneType};
///
/// let mut p = Pheromone { position: [0.0; 3], strength: 1.0, pheromone_type: PheromoneType::Trail };
/// p.evaporate(0.3);
/// assert!((p.strength - 0.7).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pheromone {
    pub position: [f32; 3],
    pub strength: f32,
    pub pheromone_type: PheromoneType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PheromoneType {
    Food,
    Alarm,
    Trail,
    Home,
}

impl Pheromone {
    pub fn evaporate(&mut self, rate: f32) {
        self.strength = (self.strength - rate).max(0.0);
    }
    #[must_use]
    #[inline]
    pub fn is_detectable(&self) -> bool {
        self.strength > 0.01
    }
}

/// Ant colony path selection probability (pheromone intensity / sum of all).
///
/// ```
/// use jantu::swarm::path_selection_probability;
///
/// let prob = path_selection_probability(3.0, &[3.0, 7.0]);
/// assert!((prob - 0.3).abs() < 0.01);
/// ```
#[must_use]
pub fn path_selection_probability(path_pheromone: f32, all_pheromones: &[f32]) -> f32 {
    let total: f32 = all_pheromones.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    path_pheromone / total
}

/// Quorum sensing: has the swarm reached critical mass for a decision?
///
/// ```
/// use jantu::swarm::quorum_reached;
///
/// assert!(quorum_reached(7, 10, 0.6));
/// assert!(!quorum_reached(4, 10, 0.6));
/// ```
#[must_use]
pub fn quorum_reached(votes: u32, total: u32, threshold: f32) -> bool {
    if total == 0 {
        return false;
    }
    (votes as f32 / total as f32) >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_swarm_behavior() {
        for b in [
            SwarmBehavior::Foraging,
            SwarmBehavior::Nesting,
            SwarmBehavior::Swarming,
            SwarmBehavior::Defense,
            SwarmBehavior::Recruitment,
        ] {
            let json = serde_json::to_string(&b).unwrap();
            let b2: SwarmBehavior = serde_json::from_str(&json).unwrap();
            assert_eq!(b, b2);
        }
    }

    #[test]
    fn serde_roundtrip_pheromone() {
        let p = Pheromone {
            position: [1.0, 2.0, 3.0],
            strength: 0.75,
            pheromone_type: PheromoneType::Food,
        };
        let json = serde_json::to_string(&p).unwrap();
        let p2: Pheromone = serde_json::from_str(&json).unwrap();
        assert!((p.strength - p2.strength).abs() < f32::EPSILON);
        assert_eq!(p.position, p2.position);
        assert_eq!(p.pheromone_type, p2.pheromone_type);
    }

    #[test]
    fn serde_roundtrip_pheromone_type() {
        for pt in [
            PheromoneType::Food,
            PheromoneType::Alarm,
            PheromoneType::Trail,
            PheromoneType::Home,
        ] {
            let json = serde_json::to_string(&pt).unwrap();
            let pt2: PheromoneType = serde_json::from_str(&json).unwrap();
            assert_eq!(pt, pt2);
        }
    }

    #[test]
    fn pheromone_evaporates() {
        let mut p = Pheromone {
            position: [0.0; 3],
            strength: 1.0,
            pheromone_type: PheromoneType::Trail,
        };
        p.evaporate(0.3);
        assert!((p.strength - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn path_selection_proportional() {
        let prob = path_selection_probability(3.0, &[3.0, 7.0]);
        assert!((prob - 0.3).abs() < 0.01);
    }

    #[test]
    fn quorum_reached_majority() {
        assert!(quorum_reached(7, 10, 0.6));
        assert!(!quorum_reached(4, 10, 0.6));
    }

    #[test]
    fn quorum_empty_group() {
        assert!(!quorum_reached(0, 0, 0.5));
    }
}
