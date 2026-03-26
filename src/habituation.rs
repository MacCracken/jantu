//! Learning through habituation and sensitization (dual-process theory).
//!
//! Implements Groves & Thompson (1970) dual-process model:
//! - **Habituation**: decreased response to repeated innocuous stimuli
//! - **Sensitization**: increased response to intense/novel stimuli
//! - **Dishabituation**: restoration of habituated response via strong extraneous stimulus
//!
//! Response = base × (1 - H) × (1 + S)

use serde::{Deserialize, Serialize};

/// Tracks habituation and sensitization state for a single stimulus channel.
///
/// ```
/// use jantu::habituation::{StimulusResponse, HabituationParams};
///
/// let params = HabituationParams::default();
/// let mut sr = StimulusResponse::new();
/// assert!((sr.response_multiplier() - 1.0).abs() < f32::EPSILON);
///
/// // Repeated low-intensity exposure habituates
/// for _ in 0..20 {
///     sr.expose(0.1, &params);
/// }
/// assert!(sr.response_multiplier() < 0.5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StimulusResponse {
    /// Habituation level (0.0 = no habituation, approaches `h_max`).
    pub habituation: f32,
    /// Sensitization level (0.0 = no sensitization, decays over time).
    pub sensitization: f32,
    /// Number of times this stimulus has been encountered.
    pub exposure_count: u32,
}

/// Parameters controlling habituation/sensitization dynamics.
///
/// # Examples
///
/// ```
/// use jantu::habituation::HabituationParams;
///
/// let params = HabituationParams::default();
/// assert!(params.habituation_rate > 0.0);
/// assert!(params.sensitization_decay > params.habituation_decay);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabituationParams {
    /// Rate of habituation buildup per exposure (0.0-1.0).
    pub habituation_rate: f32,
    /// Maximum habituation level (0.0-1.0). Prey animals may never fully habituate.
    pub habituation_max: f32,
    /// Habituation decay rate per tick during rest (spontaneous recovery).
    pub habituation_decay: f32,
    /// Sensitization gain per unit of stimulus intensity.
    pub sensitization_rate: f32,
    /// Sensitization decay rate per tick (decays faster than habituation).
    pub sensitization_decay: f32,
}

impl Default for HabituationParams {
    fn default() -> Self {
        Self {
            habituation_rate: 0.15,
            habituation_max: 0.9,
            habituation_decay: 0.003,
            sensitization_rate: 0.3,
            sensitization_decay: 0.05,
        }
    }
}

impl StimulusResponse {
    /// Create a fresh stimulus response (no prior exposure).
    #[must_use]
    pub fn new() -> Self {
        Self {
            habituation: 0.0,
            sensitization: 0.0,
            exposure_count: 0,
        }
    }

    /// Record a stimulus exposure. Intensity in [0.0, 1.0].
    ///
    /// Low intensity builds habituation; high intensity builds sensitization.
    pub fn expose(&mut self, intensity: f32, params: &HabituationParams) {
        let intensity = intensity.clamp(0.0, 1.0);
        self.exposure_count = self.exposure_count.saturating_add(1);

        // Habituation builds with diminishing returns toward h_max.
        // Strong stimuli habituate slower (dual-process: intensity favors S over H).
        let intensity_dampening = 1.0 - intensity * 0.8;
        let h_room = params.habituation_max - self.habituation;
        self.habituation += params.habituation_rate * h_room * intensity_dampening;
        self.habituation = self.habituation.clamp(0.0, params.habituation_max);

        // Sensitization spikes proportional to intensity
        self.sensitization += params.sensitization_rate * intensity;
    }

    /// Decay both habituation and sensitization over elapsed time.
    ///
    /// Call once per tick with `dt` = time step.
    pub fn decay(&mut self, dt: f32, params: &HabituationParams) {
        // Spontaneous recovery: habituation decays (exponential)
        self.habituation *= (-params.habituation_decay * dt).exp();
        // Sensitization decays faster
        self.sensitization *= (-params.sensitization_decay * dt).exp();

        // Clamp near-zero values
        if self.habituation < 1e-6 {
            self.habituation = 0.0;
        }
        if self.sensitization < 1e-6 {
            self.sensitization = 0.0;
        }
    }

    /// Compute the effective response magnitude.
    ///
    /// Returns a multiplier: `(1 - H) × (1 + S)` where H is habituation
    /// and S is sensitization. A value of 1.0 means baseline response.
    #[must_use]
    #[inline]
    pub fn response_multiplier(&self) -> f32 {
        (1.0 - self.habituation) * (1.0 + self.sensitization)
    }

    /// Whether the creature is effectively habituated (response < 20% of baseline).
    #[must_use]
    #[inline]
    pub fn is_habituated(&self) -> bool {
        self.response_multiplier() < 0.2
    }

    /// Whether the creature is sensitized (response > 150% of baseline).
    #[must_use]
    #[inline]
    pub fn is_sensitized(&self) -> bool {
        self.response_multiplier() > 1.5
    }
}

impl Default for StimulusResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply dishabituation: a strong extraneous stimulus partially restores
/// response by boosting sensitization across all channels.
///
/// `intensity` is the strength of the dishabituating stimulus (0.0-1.0).
/// Returns the sensitization boost applied.
///
/// ```
/// use jantu::habituation::{dishabituation_boost, HabituationParams};
///
/// let params = HabituationParams::default();
/// let boost = dishabituation_boost(0.9, &params);
/// assert!(boost > 0.0);
/// ```
#[must_use]
pub fn dishabituation_boost(intensity: f32, params: &HabituationParams) -> f32 {
    let intensity = intensity.clamp(0.0, 1.0);
    params.sensitization_rate * intensity * 1.5
}

/// Compute stimulus generalization: how much habituation transfers
/// from a familiar stimulus to a similar one.
///
/// `similarity` in [0.0, 1.0] where 1.0 = identical, 0.0 = completely different.
///
/// ```
/// use jantu::habituation::generalized_habituation;
///
/// let identical = generalized_habituation(0.8, 1.0);
/// let different = generalized_habituation(0.8, 0.2);
/// assert!(identical > different);
/// ```
#[must_use]
pub fn generalized_habituation(source_habituation: f32, similarity: f32) -> f32 {
    let similarity = similarity.clamp(0.0, 1.0);
    source_habituation * similarity * similarity // quadratic falloff
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_params() -> HabituationParams {
        HabituationParams::default()
    }

    #[test]
    fn fresh_stimulus_baseline_response() {
        let sr = StimulusResponse::new();
        assert!((sr.response_multiplier() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn repeated_low_intensity_habituates() {
        let params = default_params();
        let mut sr = StimulusResponse::new();
        for _ in 0..20 {
            sr.expose(0.1, &params);
        }
        // Low intensity → habituation dominates, response should drop
        assert!(
            sr.response_multiplier() < 0.5,
            "expected habituation, got multiplier {}",
            sr.response_multiplier()
        );
    }

    #[test]
    fn high_intensity_sensitizes() {
        let params = default_params();
        let mut sr = StimulusResponse::new();
        // Repeated high-intensity exposure builds sensitization faster than habituation
        for _ in 0..5 {
            sr.expose(1.0, &params);
        }
        assert!(
            sr.is_sensitized(),
            "high intensity should sensitize, got multiplier {}",
            sr.response_multiplier()
        );
    }

    #[test]
    fn habituation_caps_at_max() {
        let params = default_params();
        let mut sr = StimulusResponse::new();
        for _ in 0..100 {
            sr.expose(0.0, &params); // zero intensity → pure habituation
        }
        assert!(sr.habituation <= params.habituation_max + f32::EPSILON);
    }

    #[test]
    fn spontaneous_recovery() {
        let params = default_params();
        let mut sr = StimulusResponse::new();
        // Habituate
        for _ in 0..20 {
            sr.expose(0.0, &params);
        }
        let habituated = sr.habituation;

        // Rest for a long time
        sr.decay(1000.0, &params);
        assert!(
            sr.habituation < habituated * 0.1,
            "habituation should decay during rest"
        );
    }

    #[test]
    fn sensitization_decays_faster_than_habituation() {
        let params = default_params();
        let mut sr = StimulusResponse::new();
        sr.expose(0.5, &params);
        let h0 = sr.habituation;
        let s0 = sr.sensitization;

        sr.decay(10.0, &params);
        let h_remaining = sr.habituation / h0;
        let s_remaining = sr.sensitization / s0;
        assert!(
            s_remaining < h_remaining,
            "sensitization should decay faster: s_rem={s_remaining}, h_rem={h_remaining}"
        );
    }

    #[test]
    fn dishabituation_boosts_response() {
        let params = default_params();
        let mut sr = StimulusResponse::new();

        // Habituate fully
        for _ in 0..30 {
            sr.expose(0.0, &params);
        }
        let before = sr.response_multiplier();

        // Dishabituation via strong extraneous stimulus
        let boost = dishabituation_boost(0.9, &params);
        sr.sensitization += boost;
        let after = sr.response_multiplier();
        assert!(
            after > before,
            "dishabituation should restore response: before={before}, after={after}"
        );
    }

    #[test]
    fn generalization_gradient() {
        let source_h = 0.8;
        let identical = generalized_habituation(source_h, 1.0);
        let similar = generalized_habituation(source_h, 0.7);
        let different = generalized_habituation(source_h, 0.2);

        assert!((identical - source_h).abs() < f32::EPSILON);
        assert!(similar < identical);
        assert!(different < similar);
        assert!(different < 0.1); // quadratic falloff makes very different stimuli near-zero
    }

    #[test]
    fn exposure_count_tracks() {
        let params = default_params();
        let mut sr = StimulusResponse::new();
        sr.expose(0.5, &params);
        sr.expose(0.5, &params);
        sr.expose(0.5, &params);
        assert_eq!(sr.exposure_count, 3);
    }

    #[test]
    fn serde_roundtrip_stimulus_response() {
        let mut sr = StimulusResponse::new();
        sr.habituation = 0.5;
        sr.sensitization = 0.3;
        sr.exposure_count = 7;
        let json = serde_json::to_string(&sr).unwrap();
        let sr2: StimulusResponse = serde_json::from_str(&json).unwrap();
        assert!((sr.habituation - sr2.habituation).abs() < f32::EPSILON);
        assert!((sr.sensitization - sr2.sensitization).abs() < f32::EPSILON);
        assert_eq!(sr.exposure_count, sr2.exposure_count);
    }

    #[test]
    fn serde_roundtrip_habituation_params() {
        let p = HabituationParams::default();
        let json = serde_json::to_string(&p).unwrap();
        let p2: HabituationParams = serde_json::from_str(&json).unwrap();
        assert!((p.habituation_rate - p2.habituation_rate).abs() < f32::EPSILON);
        assert!((p.habituation_max - p2.habituation_max).abs() < f32::EPSILON);
    }
}
