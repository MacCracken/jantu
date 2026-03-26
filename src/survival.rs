use serde::{Deserialize, Serialize};

/// Survival state of a creature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SurvivalState {
    Thriving,   // all needs met, low stress
    Stable,     // needs mostly met
    Stressed,   // one or more critical drives
    Threatened, // immediate danger detected
    Fleeing,    // active flight response
    Fighting,   // active fight response
    Freezing,   // freeze response (playing dead)
}

/// Threat response classification (fight, flight, freeze, fawn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ThreatResponse {
    Fight,  // confront the threat (high aggression, size advantage)
    Flight, // run away (high fear, speed advantage)
    Freeze, // remain motionless (camouflage, small size)
    Fawn,   // submit/appease (social species, lower rank)
}

/// Select threat response based on creature traits.
///
/// aggression (0-1), speed (0-1), size relative to threat (0-2, 1=equal), social_rank (0-1).
#[must_use]
pub fn select_threat_response(
    aggression: f32,
    speed: f32,
    relative_size: f32,
    social_rank: f32,
) -> ThreatResponse {
    let fight_score = aggression * 0.5 + relative_size * 0.3 + social_rank * 0.2;
    let flight_score = speed * 0.6 + (1.0 - aggression) * 0.4;
    let freeze_score = (1.0 - speed) * 0.5 + (1.0 - relative_size).max(0.0) * 0.5;
    let fawn_score = (1.0 - social_rank) * 0.6 + (1.0 - aggression) * 0.4;

    let scores = [
        (fight_score, ThreatResponse::Fight),
        (flight_score, ThreatResponse::Flight),
        (freeze_score, ThreatResponse::Freeze),
        (fawn_score, ThreatResponse::Fawn),
    ];
    // SAFETY: scores is a fixed 4-element array, so max_by always returns Some.
    // Using unwrap_or with a default to satisfy the zero-unwrap rule.
    scores
        .iter()
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .map_or(ThreatResponse::Freeze, |s| s.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_survival_state() {
        for s in [
            SurvivalState::Thriving,
            SurvivalState::Stable,
            SurvivalState::Stressed,
            SurvivalState::Threatened,
            SurvivalState::Fleeing,
            SurvivalState::Fighting,
            SurvivalState::Freezing,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let s2: SurvivalState = serde_json::from_str(&json).unwrap();
            assert_eq!(s, s2);
        }
    }

    #[test]
    fn serde_roundtrip_threat_response() {
        for r in [
            ThreatResponse::Fight,
            ThreatResponse::Flight,
            ThreatResponse::Freeze,
            ThreatResponse::Fawn,
        ] {
            let json = serde_json::to_string(&r).unwrap();
            let r2: ThreatResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(r, r2);
        }
    }

    #[test]
    fn aggressive_large_creature_fights() {
        let r = select_threat_response(0.9, 0.3, 1.5, 0.8);
        assert_eq!(r, ThreatResponse::Fight);
    }

    #[test]
    fn fast_timid_creature_flees() {
        let r = select_threat_response(0.1, 0.9, 0.5, 0.5);
        assert_eq!(r, ThreatResponse::Flight);
    }

    #[test]
    fn submissive_low_rank_fawns() {
        let r = select_threat_response(0.1, 0.1, 0.3, 0.1);
        assert_eq!(r, ThreatResponse::Fawn);
    }

    #[test]
    fn slow_small_freezes() {
        let r = select_threat_response(0.1, 0.1, 0.3, 0.5);
        // Low speed + small size → freeze or fawn
        assert!(matches!(r, ThreatResponse::Freeze | ThreatResponse::Fawn));
    }
}
