//! Communication signals — alarm calls, mating calls, territorial displays.
//!
//! Models animal signaling theory: honest signals (Zahavi), ritualized displays,
//! signal detection, and receiver response. Signals have cost, range, and
//! information content.

use serde::{Deserialize, Serialize};

/// Signal modality (communication channel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SignalModality {
    /// Auditory signals (calls, songs, echolocation).
    Acoustic,
    /// Visual signals (colors, postures, bioluminescence).
    Visual,
    /// Chemical signals (scent marks, pheromones).
    Chemical,
    /// Tactile signals (grooming, nudging).
    Tactile,
    /// Vibrational signals (substrate drumming, web plucking).
    Vibrational,
    /// Electrical signals (weakly electric fish).
    Electric,
}

/// Signal function (purpose).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SignalFunction {
    /// Warning of predator presence.
    Alarm,
    /// Attracting mates.
    MatingCall,
    /// Defending territory.
    TerritorialDisplay,
    /// Begging for food (offspring to parent).
    Begging,
    /// Coordinating group movement.
    Contact,
    /// Signaling submission.
    Submission,
    /// Signaling aggression/threat.
    Threat,
    /// Announcing food discovery to group.
    FoodCall,
}

/// A signal emission event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// What modality is used.
    pub modality: SignalModality,
    /// What function the signal serves.
    pub function: SignalFunction,
    /// Signal intensity/amplitude (0.0-1.0).
    pub intensity: f32,
    /// Honesty of the signal (0.0-1.0). Honest signals are costly to produce.
    pub honesty: f32,
}

impl Signal {
    /// Create a new signal.
    #[must_use]
    pub fn new(modality: SignalModality, function: SignalFunction, intensity: f32) -> Self {
        Self {
            modality,
            function,
            intensity: intensity.clamp(0.0, 1.0),
            honesty: 1.0,
        }
    }
}

/// Compute the effective range of a signal based on modality and intensity.
///
/// Returns range as a multiplier of base detection distance.
/// Different modalities travel different distances.
#[must_use]
pub fn signal_range(modality: SignalModality, intensity: f32) -> f32 {
    let intensity = intensity.clamp(0.0, 1.0);
    let modality_factor = match modality {
        SignalModality::Acoustic => 1.0,    // sound carries well
        SignalModality::Visual => 0.8,      // line-of-sight dependent
        SignalModality::Chemical => 0.6,    // diffusion limited
        SignalModality::Tactile => 0.05,    // requires contact
        SignalModality::Vibrational => 0.3, // substrate-dependent
        SignalModality::Electric => 0.15,   // short range
    };
    modality_factor * intensity
}

/// Compute the energetic cost of producing a signal.
///
/// Honest signals are costlier (handicap principle). Higher intensity costs more.
#[must_use]
pub fn signal_cost(intensity: f32, honesty: f32) -> f32 {
    let intensity = intensity.clamp(0.0, 1.0);
    let honesty = honesty.clamp(0.0, 1.0);
    // Honest, intense signals are expensive
    intensity * intensity * (0.3 + honesty * 0.7)
}

/// Compute receiver's detection probability for a signal.
///
/// - `signal_intensity`: emitted signal strength
/// - `distance`: distance between emitter and receiver
/// - `max_range`: maximum detection range
/// - `noise_level`: ambient noise in the channel (0.0-1.0)
///
/// Returns detection probability (0.0-1.0).
#[must_use]
pub fn detection_probability(
    signal_intensity: f32,
    distance: f32,
    max_range: f32,
    noise_level: f32,
) -> f32 {
    if distance >= max_range || max_range <= 0.0 {
        return 0.0;
    }
    let signal_intensity = signal_intensity.clamp(0.0, 1.0);
    let noise_level = noise_level.clamp(0.0, 1.0);

    // Signal attenuates with distance (inverse square)
    let normalized = distance / max_range;
    let attenuation = (1.0 - normalized) * (1.0 - normalized);

    // Signal-to-noise ratio determines detection
    let signal_strength = signal_intensity * attenuation;
    let snr = signal_strength / (noise_level + 0.01);

    // Sigmoid detection: 1 / (1 + exp(-k*(snr - threshold)))
    let sigmoid = 1.0 / (1.0 + (-6.0 * (snr - 0.5)).exp());
    sigmoid.clamp(0.0, 1.0)
}

/// Compute the receiver's response intensity to a detected signal.
///
/// - `signal_honesty`: how honest the signal is (0.0-1.0)
/// - `familiarity_with_sender`: how well the receiver knows the sender (0.0-1.0)
/// - `receiver_state_match`: how relevant the signal is to receiver's current needs (0.0-1.0)
///
/// Returns response intensity (0.0-1.0).
#[must_use]
pub fn receiver_response(
    signal_honesty: f32,
    familiarity_with_sender: f32,
    receiver_state_match: f32,
) -> f32 {
    let signal_honesty = signal_honesty.clamp(0.0, 1.0);
    let familiarity_with_sender = familiarity_with_sender.clamp(0.0, 1.0);
    let receiver_state_match = receiver_state_match.clamp(0.0, 1.0);

    // Trust signal more if honest and from familiar individual
    let trust = signal_honesty * (0.4 + familiarity_with_sender * 0.6);
    (trust * receiver_state_match).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acoustic_travels_farthest() {
        let acoustic = signal_range(SignalModality::Acoustic, 0.8);
        let tactile = signal_range(SignalModality::Tactile, 0.8);
        assert!(acoustic > tactile);
    }

    #[test]
    fn louder_signal_farther_range() {
        let quiet = signal_range(SignalModality::Acoustic, 0.2);
        let loud = signal_range(SignalModality::Acoustic, 0.9);
        assert!(loud > quiet);
    }

    #[test]
    fn honest_signals_cost_more() {
        let honest = signal_cost(0.7, 1.0);
        let deceptive = signal_cost(0.7, 0.1);
        assert!(honest > deceptive);
    }

    #[test]
    fn intense_signals_cost_more() {
        let soft = signal_cost(0.2, 0.8);
        let loud = signal_cost(0.9, 0.8);
        assert!(loud > soft);
    }

    #[test]
    fn detection_decreases_with_distance() {
        let close = detection_probability(0.8, 5.0, 100.0, 0.1);
        let far = detection_probability(0.8, 80.0, 100.0, 0.1);
        assert!(close > far, "close={close}, far={far}");
    }

    #[test]
    fn noise_reduces_detection() {
        let quiet = detection_probability(0.5, 30.0, 100.0, 0.05);
        let noisy = detection_probability(0.5, 30.0, 100.0, 0.8);
        assert!(quiet > noisy, "quiet={quiet}, noisy={noisy}");
    }

    #[test]
    fn out_of_range_no_detection() {
        assert_eq!(detection_probability(1.0, 101.0, 100.0, 0.0), 0.0);
    }

    #[test]
    fn familiar_honest_signal_strong_response() {
        let response = receiver_response(0.9, 0.8, 0.9);
        assert!(response > 0.5, "should respond strongly: {response}");
    }

    #[test]
    fn dishonest_stranger_weak_response() {
        let response = receiver_response(0.2, 0.1, 0.8);
        assert!(response < 0.2, "should respond weakly: {response}");
    }

    #[test]
    fn irrelevant_signal_low_response() {
        let response = receiver_response(0.9, 0.9, 0.1);
        assert!(
            response < 0.15,
            "irrelevant signal should be ignored: {response}"
        );
    }

    #[test]
    fn serde_roundtrip_signal_modality() {
        for m in [
            SignalModality::Acoustic,
            SignalModality::Visual,
            SignalModality::Chemical,
            SignalModality::Tactile,
            SignalModality::Vibrational,
            SignalModality::Electric,
        ] {
            let json = serde_json::to_string(&m).unwrap();
            let m2: SignalModality = serde_json::from_str(&json).unwrap();
            assert_eq!(m, m2);
        }
    }

    #[test]
    fn serde_roundtrip_signal_function() {
        for f in [
            SignalFunction::Alarm,
            SignalFunction::MatingCall,
            SignalFunction::TerritorialDisplay,
            SignalFunction::Begging,
            SignalFunction::Contact,
            SignalFunction::Submission,
            SignalFunction::Threat,
            SignalFunction::FoodCall,
        ] {
            let json = serde_json::to_string(&f).unwrap();
            let f2: SignalFunction = serde_json::from_str(&json).unwrap();
            assert_eq!(f, f2);
        }
    }

    #[test]
    fn serde_roundtrip_signal() {
        let s = Signal::new(SignalModality::Acoustic, SignalFunction::Alarm, 0.9);
        let json = serde_json::to_string(&s).unwrap();
        let s2: Signal = serde_json::from_str(&json).unwrap();
        assert_eq!(s.modality, s2.modality);
        assert_eq!(s.function, s2.function);
        assert!((s.intensity - s2.intensity).abs() < f32::EPSILON);
    }
}
