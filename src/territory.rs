use serde::{Deserialize, Serialize};

/// Territory marking strength (0.0 = unmarked, 1.0 = freshly marked).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TerritoryMark {
    pub position: [f32; 3],
    pub strength: f32,
    pub owner_id: u64,
}

impl TerritoryMark {
    /// Decay the mark over time.
    pub fn decay(&mut self, rate: f32) {
        self.strength = (self.strength - rate).max(0.0);
    }

    /// Is this mark still detectable?
    #[must_use]
    #[inline]
    pub fn is_active(&self) -> bool {
        self.strength > 0.05
    }
}

/// Territorial response when encountering another's territory.
#[must_use]
pub fn territorial_response(own_strength: f32, intruder_strength: f32, aggression: f32) -> f32 {
    // Returns aggression modifier (>1.0 = more aggressive, <1.0 = retreat)
    let dominance = own_strength / (intruder_strength + 0.01);
    dominance * aggression
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_territory_mark() {
        let m = TerritoryMark {
            position: [1.0, 2.0, 3.0],
            strength: 0.8,
            owner_id: 42,
        };
        let json = serde_json::to_string(&m).unwrap();
        let m2: TerritoryMark = serde_json::from_str(&json).unwrap();
        assert_eq!(m.position, m2.position);
        assert!((m.strength - m2.strength).abs() < f32::EPSILON);
        assert_eq!(m.owner_id, m2.owner_id);
    }

    #[test]
    fn mark_decays() {
        let mut m = TerritoryMark {
            position: [0.0; 3],
            strength: 1.0,
            owner_id: 1,
        };
        m.decay(0.3);
        assert!((m.strength - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn mark_decays_to_zero() {
        let mut m = TerritoryMark {
            position: [0.0; 3],
            strength: 0.1,
            owner_id: 1,
        };
        m.decay(0.5);
        assert_eq!(m.strength, 0.0);
    }

    #[test]
    fn weak_mark_inactive() {
        let m = TerritoryMark {
            position: [0.0; 3],
            strength: 0.01,
            owner_id: 1,
        };
        assert!(!m.is_active());
    }

    #[test]
    fn stronger_territory_more_aggressive() {
        let strong = territorial_response(1.0, 0.5, 0.8);
        let weak = territorial_response(0.5, 1.0, 0.8);
        assert!(strong > weak);
    }
}
