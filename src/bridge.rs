//! Cross-crate bridges — convert primitive values from other AGNOS science crates
//! into jantu behavior parameters and vice versa.
//!
//! Always available — takes primitive values (f32/f64), no science crate deps.

// ── Raasta bridges (navigation) ────────────────────────────────────────────

/// Convert creature position and target to a pathfinding request vector `[dx, dy, dz]`.
///
/// Returns the unnormalized direction from position to target.
#[must_use]
#[inline]
pub fn target_to_direction(position: [f32; 3], target: [f32; 3]) -> [f32; 3] {
    [
        target[0] - position[0],
        target[1] - position[1],
        target[2] - position[2],
    ]
}

/// Convert group centroid and spread to a crowd destination radius.
///
/// Creatures within the radius are "arrived"; outside need pathfinding.
#[must_use]
#[inline]
pub fn group_spread_to_arrival_radius(spread_m: f32, group_size: u32) -> f32 {
    if group_size == 0 {
        return 0.0;
    }
    spread_m * (group_size as f32).sqrt()
}

// ── Vanaspati bridges (botany) ─────────────────────────────────────────────

/// Convert canopy cover density (0.0–1.0) to concealment factor for prey/predator.
///
/// Higher canopy → harder to detect/be detected.
#[must_use]
#[inline]
pub fn canopy_to_concealment(canopy_density: f32) -> f32 {
    canopy_density.clamp(0.0, 1.0) * 0.8 // max 80% concealment
}

/// Convert food source availability (0.0–1.0) to foraging attraction weight.
///
/// Higher food → stronger pull toward the food source.
#[must_use]
#[inline]
pub fn food_to_foraging_weight(food_availability: f32) -> f32 {
    food_availability.clamp(0.0, 1.0).powi(2) // quadratic — marginal food less attractive
}

// ── Badal bridges (weather) ────────────────────────────────────────────────

/// Convert temperature (°C) to activity level scaling (0.0–1.0).
///
/// Most terrestrial animals have reduced activity at extreme temperatures.
/// Optimal range: 15–30°C. Below 0°C or above 40°C: minimal activity.
#[must_use]
pub fn temperature_to_activity_scale(temperature_celsius: f32) -> f32 {
    if !(-10.0..=50.0).contains(&temperature_celsius) {
        return 0.0;
    }
    // Bell curve centered at 22°C
    let optimal = 22.0_f32;
    let sigma = 15.0_f32;
    let diff = temperature_celsius - optimal;
    (-(diff * diff) / (2.0 * sigma * sigma)).exp()
}

/// Convert precipitation rate (mm/hr) to shelter-seeking urgency (0.0–1.0).
///
/// Light rain: low urgency. Heavy rain: high urgency.
#[must_use]
#[inline]
pub fn precipitation_to_shelter_urgency(rate_mm_hr: f32) -> f32 {
    if rate_mm_hr <= 0.0 {
        return 0.0;
    }
    (rate_mm_hr / 25.0).clamp(0.0, 1.0) // 25 mm/hr = maximum urgency
}

// ── Garjan bridges (sound synthesis) ───────────────────────────────────────

/// Convert creature speed (m/s) and body size (m) to footstep synthesis parameters.
///
/// Returns `(step_rate_hz, impact_force_normalized)`.
#[must_use]
pub fn locomotion_to_footstep_params(speed_ms: f32, body_size_m: f32) -> (f32, f32) {
    if body_size_m <= 0.0 {
        return (0.0, 0.0);
    }
    // Step frequency scales with speed / stride_length
    let stride_length = body_size_m * 2.0; // rough: stride ≈ 2× body length
    let step_rate = if stride_length > 0.0 {
        speed_ms / stride_length
    } else {
        0.0
    };
    // Impact force scales with mass ∝ size³, normalized
    let impact = (body_size_m / 2.0).clamp(0.0, 1.0);
    (step_rate, impact)
}

/// Convert alarm call trigger (threat level 0.0–1.0) to vocalization urgency.
///
/// Returns a synthesis intensity parameter (0.0–1.0).
#[must_use]
#[inline]
pub fn threat_to_alarm_intensity(threat_level: f32) -> f32 {
    // Exponential response — small threats barely audible, high threats very loud
    let t = threat_level.clamp(0.0, 1.0);
    t * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_direction_basic() {
        let d = target_to_direction([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!((d[0] - 1.0).abs() < 0.001);
    }

    #[test]
    fn group_spread_basic() {
        let r = group_spread_to_arrival_radius(2.0, 4);
        assert!((r - 4.0).abs() < 0.01); // 2 × sqrt(4) = 4
    }

    #[test]
    fn group_spread_zero() {
        assert!((group_spread_to_arrival_radius(1.0, 0)).abs() < 0.001);
    }

    #[test]
    fn canopy_concealment() {
        assert!((canopy_to_concealment(1.0) - 0.8).abs() < 0.01);
        assert!((canopy_to_concealment(0.0)).abs() < 0.001);
    }

    #[test]
    fn food_foraging_weight() {
        assert!((food_to_foraging_weight(1.0) - 1.0).abs() < 0.01);
        assert!((food_to_foraging_weight(0.0)).abs() < 0.001);
    }

    #[test]
    fn temperature_activity_optimal() {
        let a = temperature_to_activity_scale(22.0);
        assert!(a > 0.9, "optimal temp should give high activity: {a}");
    }

    #[test]
    fn temperature_activity_extreme() {
        assert!((temperature_to_activity_scale(50.1)).abs() < 0.001);
        assert!((temperature_to_activity_scale(-10.1)).abs() < 0.001);
    }

    #[test]
    fn precipitation_shelter() {
        assert!((precipitation_to_shelter_urgency(0.0)).abs() < 0.001);
        assert!((precipitation_to_shelter_urgency(25.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn footstep_params_basic() {
        let (rate, impact) = locomotion_to_footstep_params(2.0, 1.0);
        assert!(rate > 0.0);
        assert!(impact > 0.0 && impact <= 1.0);
    }

    #[test]
    fn footstep_params_zero_size() {
        let (rate, impact) = locomotion_to_footstep_params(2.0, 0.0);
        assert!((rate).abs() < 0.001);
        assert!((impact).abs() < 0.001);
    }

    #[test]
    fn alarm_intensity_scales() {
        let low = threat_to_alarm_intensity(0.2);
        let high = threat_to_alarm_intensity(0.8);
        assert!(high > low);
    }
}
