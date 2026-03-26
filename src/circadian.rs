//! Circadian rhythm drive modifiers.
//!
//! Models endogenous biological clocks that modulate creature drives based on
//! time of day. Supports diurnal, nocturnal, and crepuscular activity patterns.
//!
//! Uses sinusoidal oscillators anchored to a 24-hour cycle (or custom period
//! for non-Earth environments).

use serde::{Deserialize, Serialize};

/// Activity pattern classification.
///
/// # Examples
///
/// ```
/// use jantu::circadian::ActivityPattern;
///
/// let pattern = ActivityPattern::Nocturnal;
/// assert_ne!(pattern, ActivityPattern::Diurnal);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ActivityPattern {
    /// Active during daylight (peak at solar noon).
    Diurnal,
    /// Active during darkness (peak at midnight).
    Nocturnal,
    /// Active at dawn and dusk (two peaks).
    Crepuscular,
    /// Active regardless of time (no circadian modulation).
    Cathemeral,
}

/// Circadian clock state for a creature.
///
/// ```
/// use jantu::circadian::{CircadianClock, ActivityPattern};
///
/// let clock = CircadianClock::new(ActivityPattern::Diurnal);
/// let noon = clock.activity_level(12.0);
/// let midnight = clock.activity_level(0.0);
/// assert!(noon > midnight);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianClock {
    /// The creature's innate activity pattern.
    pub pattern: ActivityPattern,
    /// Period length in hours (default 24.0 for Earth).
    pub period_hours: f32,
    /// Phase offset in hours (shifts the peak activity time).
    pub phase_offset: f32,
    /// Amplitude of the rhythm (0.0-1.0). Higher = stronger day/night difference.
    pub amplitude: f32,
}

impl CircadianClock {
    /// Create a new circadian clock with the given activity pattern.
    #[must_use]
    pub fn new(pattern: ActivityPattern) -> Self {
        Self {
            pattern,
            period_hours: 24.0,
            phase_offset: 0.0,
            amplitude: 0.7,
        }
    }

    /// Compute the activity multiplier at a given time of day.
    ///
    /// `hour` is the current time in hours (0.0-period). Returns a multiplier
    /// where 1.0 = peak activity and (1 - amplitude) = minimum activity.
    #[must_use]
    pub fn activity_level(&self, hour: f32) -> f32 {
        if self.pattern == ActivityPattern::Cathemeral {
            return 1.0;
        }
        if self.period_hours <= 0.0 {
            return 1.0;
        }

        let phase = core::f32::consts::TAU * (hour - self.phase_offset) / self.period_hours;

        let raw = match self.pattern {
            // Diurnal: peak at noon (hour 12 with default phase)
            ActivityPattern::Diurnal => (phase - core::f32::consts::PI).cos(),
            // Nocturnal: peak at midnight (inverted diurnal)
            ActivityPattern::Nocturnal => -(phase - core::f32::consts::PI).cos(),
            // Crepuscular: peaks at dawn (hour 6) and dusk (hour 18)
            ActivityPattern::Crepuscular => -(2.0 * phase).cos(),
            // Cathemeral handled by early return above; future variants default to flat
            _ => return 1.0,
        };

        // Map [-1, 1] → [1 - amplitude, 1]
        let normalized = (raw + 1.0) * 0.5; // [0, 1]
        1.0 - self.amplitude + self.amplitude * normalized
    }

    /// Compute a drive modifier for a specific instinct at the given hour.
    ///
    /// Returns a multiplier to apply to the instinct's drive level.
    /// Activity-linked drives (hunger, thirst) increase during active periods;
    /// rest drive increases during inactive periods.
    ///
    /// ```
    /// use jantu::circadian::{CircadianClock, ActivityPattern};
    ///
    /// let clock = CircadianClock::new(ActivityPattern::Diurnal);
    /// let rest_at_midnight = clock.drive_modifier(0.0, true);
    /// let rest_at_noon = clock.drive_modifier(12.0, true);
    /// assert!(rest_at_midnight > rest_at_noon);
    /// ```
    #[must_use]
    pub fn drive_modifier(&self, hour: f32, is_rest_drive: bool) -> f32 {
        let activity = self.activity_level(hour);
        if is_rest_drive {
            // Rest drive is inverse of activity
            1.0 + (1.0 - activity) * self.amplitude
        } else {
            // Active drives scale with activity
            activity
        }
    }
}

/// Compute the zeitgeber (time-giver) adjustment for entraining a circadian
/// clock to an external light cycle.
///
/// `current_phase` and `light_phase` are in hours. Returns a phase correction
/// in hours to gradually entrain the clock. `entrainment_rate` controls speed
/// (0.0-1.0, typically 0.1-0.3 per day).
///
/// ```
/// use jantu::circadian::zeitgeber_correction;
///
/// let correction = zeitgeber_correction(2.0, 0.0, 24.0, 0.2);
/// assert!(correction < 0.0); // pulls phase backward toward light
/// ```
#[must_use]
pub fn zeitgeber_correction(
    current_phase: f32,
    light_phase: f32,
    period: f32,
    entrainment_rate: f32,
) -> f32 {
    if period <= 0.0 {
        return 0.0;
    }
    let entrainment_rate = entrainment_rate.clamp(0.0, 1.0);

    // Shortest path around the cycle
    let mut diff = light_phase - current_phase;
    let half = period * 0.5;
    if diff > half {
        diff -= period;
    } else if diff < -half {
        diff += period;
    }
    diff * entrainment_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diurnal_peak_at_noon() {
        let clock = CircadianClock::new(ActivityPattern::Diurnal);
        let noon = clock.activity_level(12.0);
        let midnight = clock.activity_level(0.0);
        assert!(
            noon > midnight,
            "diurnal should peak at noon: noon={noon}, midnight={midnight}"
        );
    }

    #[test]
    fn nocturnal_peak_at_midnight() {
        let clock = CircadianClock::new(ActivityPattern::Nocturnal);
        let noon = clock.activity_level(12.0);
        let midnight = clock.activity_level(0.0);
        assert!(
            midnight > noon,
            "nocturnal should peak at midnight: midnight={midnight}, noon={noon}"
        );
    }

    #[test]
    fn crepuscular_peaks_at_dawn_and_dusk() {
        let clock = CircadianClock::new(ActivityPattern::Crepuscular);
        let dawn = clock.activity_level(6.0);
        let noon = clock.activity_level(12.0);
        let dusk = clock.activity_level(18.0);
        assert!(dawn > noon, "crepuscular should peak at dawn over noon");
        assert!(dusk > noon, "crepuscular should peak at dusk over noon");
    }

    #[test]
    fn cathemeral_always_one() {
        let clock = CircadianClock::new(ActivityPattern::Cathemeral);
        for hour in [0.0, 6.0, 12.0, 18.0, 23.0] {
            assert!(
                (clock.activity_level(hour) - 1.0).abs() < f32::EPSILON,
                "cathemeral should always be 1.0"
            );
        }
    }

    #[test]
    fn activity_within_bounds() {
        let clock = CircadianClock::new(ActivityPattern::Diurnal);
        for hour in 0..24 {
            let level = clock.activity_level(hour as f32);
            let min = 1.0 - clock.amplitude;
            assert!(
                level >= min - f32::EPSILON && level <= 1.0 + f32::EPSILON,
                "activity out of bounds at hour {hour}: {level}"
            );
        }
    }

    #[test]
    fn rest_drive_inverse_of_activity() {
        let clock = CircadianClock::new(ActivityPattern::Diurnal);
        let active_rest = clock.drive_modifier(12.0, true);
        let inactive_rest = clock.drive_modifier(0.0, true);
        assert!(
            inactive_rest > active_rest,
            "rest drive should be higher when inactive"
        );
    }

    #[test]
    fn zeitgeber_entrains_toward_light() {
        let correction = zeitgeber_correction(2.0, 0.0, 24.0, 0.2);
        assert!(correction < 0.0, "should pull phase backward toward light");

        let correction2 = zeitgeber_correction(0.0, 2.0, 24.0, 0.2);
        assert!(correction2 > 0.0, "should pull phase forward toward light");
    }

    #[test]
    fn zeitgeber_wraps_around_cycle() {
        // Phase at 23, light at 1 → should go forward (2h), not backward (22h)
        let correction = zeitgeber_correction(23.0, 1.0, 24.0, 1.0);
        assert!(
            correction > 0.0 && correction < 5.0,
            "should take short path: {correction}"
        );
    }

    #[test]
    fn custom_period() {
        // A planet with 30-hour days
        let mut clock = CircadianClock::new(ActivityPattern::Diurnal);
        clock.period_hours = 30.0;
        let peak = clock.activity_level(15.0); // noon on 30h planet
        let trough = clock.activity_level(0.0);
        assert!(peak > trough);
    }

    #[test]
    fn zero_period_safe() {
        let mut clock = CircadianClock::new(ActivityPattern::Diurnal);
        clock.period_hours = 0.0;
        assert!((clock.activity_level(12.0) - 1.0).abs() < f32::EPSILON);
        assert!((zeitgeber_correction(0.0, 1.0, 0.0, 0.2)).abs() < f32::EPSILON);
    }

    #[test]
    fn serde_roundtrip_activity_pattern() {
        for p in [
            ActivityPattern::Diurnal,
            ActivityPattern::Nocturnal,
            ActivityPattern::Crepuscular,
            ActivityPattern::Cathemeral,
        ] {
            let json = serde_json::to_string(&p).unwrap();
            let p2: ActivityPattern = serde_json::from_str(&json).unwrap();
            assert_eq!(p, p2);
        }
    }

    #[test]
    fn serde_roundtrip_circadian_clock() {
        let clock = CircadianClock::new(ActivityPattern::Nocturnal);
        let json = serde_json::to_string(&clock).unwrap();
        let clock2: CircadianClock = serde_json::from_str(&json).unwrap();
        assert_eq!(clock.pattern, clock2.pattern);
        assert!((clock.period_hours - clock2.period_hours).abs() < f32::EPSILON);
        assert!((clock.amplitude - clock2.amplitude).abs() < f32::EPSILON);
    }
}
