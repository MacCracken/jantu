use serde::{Deserialize, Serialize};

/// Creature lifecycle stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LifeStage {
    Egg,
    Juvenile,
    Adolescent,
    Adult,
    Elder,
    Deceased,
}

/// Metabolic rate relative to body mass (Kleiber's law).
///
/// BMR ∝ M^0.75 (3/4 power law)
///
/// ```
/// use jantu::lifecycle::basal_metabolic_rate;
///
/// let mouse = basal_metabolic_rate(0.02, 70.0);
/// let human = basal_metabolic_rate(70.0, 70.0);
/// assert!(human > mouse);
/// ```
#[must_use]
#[inline]
pub fn basal_metabolic_rate(body_mass_kg: f32, constant: f32) -> f32 {
    if body_mass_kg <= 0.0 {
        return 0.0;
    }
    constant * body_mass_kg.powf(0.75)
}

/// Lifespan scaling (Kleiber): larger animals live longer.
///
/// Lifespan ∝ M^0.25
#[must_use]
#[inline]
pub fn estimated_lifespan_years(body_mass_kg: f32, constant: f32) -> f32 {
    if body_mass_kg <= 0.0 {
        return 0.0;
    }
    constant * body_mass_kg.powf(0.25)
}

/// Heart rate scaling: smaller animals have faster hearts.
///
/// HR ∝ M^(-0.25)
#[must_use]
#[inline]
pub fn heart_rate_bpm(body_mass_kg: f32, constant: f32) -> f32 {
    if body_mass_kg <= 0.0 {
        return 0.0;
    }
    constant * body_mass_kg.powf(-0.25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_life_stage() {
        for s in [
            LifeStage::Egg,
            LifeStage::Juvenile,
            LifeStage::Adolescent,
            LifeStage::Adult,
            LifeStage::Elder,
            LifeStage::Deceased,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let s2: LifeStage = serde_json::from_str(&json).unwrap();
            assert_eq!(s, s2);
        }
    }

    #[test]
    fn larger_animal_higher_bmr() {
        let mouse = basal_metabolic_rate(0.02, 70.0);
        let human = basal_metabolic_rate(70.0, 70.0);
        assert!(human > mouse);
    }

    #[test]
    fn larger_animal_lives_longer() {
        let mouse = estimated_lifespan_years(0.02, 10.0);
        let elephant = estimated_lifespan_years(5000.0, 10.0);
        assert!(elephant > mouse);
    }

    #[test]
    fn smaller_animal_faster_heart() {
        let mouse = heart_rate_bpm(0.02, 200.0);
        let elephant = heart_rate_bpm(5000.0, 200.0);
        assert!(mouse > elephant);
    }

    #[test]
    fn zero_mass_safe() {
        assert_eq!(basal_metabolic_rate(0.0, 70.0), 0.0);
        assert_eq!(estimated_lifespan_years(0.0, 10.0), 0.0);
        assert_eq!(heart_rate_bpm(0.0, 200.0), 0.0);
    }

    #[test]
    fn kleiber_three_quarter_power() {
        // Doubling mass should increase BMR by ~1.68x (2^0.75)
        let m1 = basal_metabolic_rate(10.0, 70.0);
        let m2 = basal_metabolic_rate(20.0, 70.0);
        let ratio = m2 / m1;
        assert!(
            (ratio - 1.68).abs() < 0.05,
            "ratio should be ~1.68, got {ratio}"
        );
    }
}
