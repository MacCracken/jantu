use serde::{Deserialize, Serialize};

/// Creature lifecycle stage.
///
/// # Examples
///
/// ```
/// use jantu::lifecycle::LifeStage;
///
/// let stage = LifeStage::Adult;
/// assert_ne!(stage, LifeStage::Juvenile);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LifeStage {
    /// Unhatched or gestating.
    Egg,
    /// Post-birth, pre-adolescent.
    Juvenile,
    /// Developing toward maturity.
    Adolescent,
    /// Reproductively mature.
    Adult,
    /// Post-reproductive decline.
    Elder,
    /// No longer alive.
    Deceased,
}

/// General allometric scaling: `constant * mass^exponent`.
///
/// All biological scaling relationships follow this form. Use specific
/// convenience functions below for standard Kleiber exponents, or call
/// this directly for taxa-specific scaling.
///
/// ```
/// use jantu::lifecycle::allometric_scale;
///
/// // Kleiber BMR: constant=70, exponent=0.75
/// let bmr = allometric_scale(40.0, 70.0, 0.75);
/// assert!(bmr > 0.0);
///
/// // 2/3 scaling (surface-area law, some taxa)
/// let bmr_23 = allometric_scale(40.0, 70.0, 0.667);
/// assert!(bmr_23 < bmr);
/// ```
#[must_use]
#[inline]
pub fn allometric_scale(body_mass_kg: f32, constant: f32, exponent: f32) -> f32 {
    if body_mass_kg <= 0.0 {
        return 0.0;
    }
    constant * body_mass_kg.powf(exponent)
}

/// Metabolic rate relative to body mass (Kleiber's law).
///
/// BMR ∝ M^0.75 (3/4 power law). For taxa-specific exponents, use
/// [`allometric_scale`] directly.
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
    allometric_scale(body_mass_kg, constant, 0.75)
}

/// Lifespan scaling (Kleiber): larger animals live longer.
///
/// Lifespan ∝ M^0.25. For taxa-specific exponents, use
/// [`allometric_scale`] directly.
///
/// ```
/// use jantu::lifecycle::estimated_lifespan_years;
///
/// let mouse = estimated_lifespan_years(0.02, 10.0);
/// let elephant = estimated_lifespan_years(5000.0, 10.0);
/// assert!(elephant > mouse);
/// ```
#[must_use]
#[inline]
pub fn estimated_lifespan_years(body_mass_kg: f32, constant: f32) -> f32 {
    allometric_scale(body_mass_kg, constant, 0.25)
}

/// Heart rate scaling: smaller animals have faster hearts.
///
/// HR ∝ M^(-0.25). For taxa-specific exponents, use
/// [`allometric_scale`] directly.
///
/// ```
/// use jantu::lifecycle::heart_rate_bpm;
///
/// let mouse_hr = heart_rate_bpm(0.02, 200.0);
/// let elephant_hr = heart_rate_bpm(5000.0, 200.0);
/// assert!(mouse_hr > elephant_hr);
/// ```
#[must_use]
#[inline]
pub fn heart_rate_bpm(body_mass_kg: f32, constant: f32) -> f32 {
    allometric_scale(body_mass_kg, constant, -0.25)
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
