use serde::{Deserialize, Serialize};

/// Drive level (0.0 = sated, 1.0 = critical need).
///
/// ```
/// use jantu::instinct::DriveLevel;
///
/// let drive = DriveLevel::new(0.75);
/// assert!(!drive.is_critical());
/// assert!(!drive.is_sated());
///
/// // Values are clamped to [0.0, 1.0]
/// let clamped = DriveLevel::new(1.5);
/// assert_eq!(clamped.value(), 1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DriveLevel(f32);

impl DriveLevel {
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }
    #[must_use]
    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
    #[must_use]
    #[inline]
    pub fn is_critical(&self) -> bool {
        self.0 > 0.8
    }
    #[must_use]
    #[inline]
    pub fn is_sated(&self) -> bool {
        self.0 < 0.2
    }

    pub fn increase(&mut self, amount: f32) {
        self.0 = (self.0 + amount).clamp(0.0, 1.0);
    }
    pub fn decrease(&mut self, amount: f32) {
        self.0 = (self.0 - amount).clamp(0.0, 1.0);
    }
}

/// Core instinct types (Tinbergen's four questions mapped to drives).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InstinctType {
    Hunger,       // foraging drive
    Thirst,       // water seeking
    Fear,         // threat avoidance
    Aggression,   // territorial/competitive
    Reproduction, // mate seeking
    Nurturing,    // offspring care
    Curiosity,    // exploration/play
    Social,       // group affiliation
    Rest,         // energy conservation
}

/// An active instinct with its current drive level.
///
/// ```
/// use jantu::instinct::{Instinct, InstinctType, DriveLevel};
///
/// let mut fear = Instinct::new(InstinctType::Fear);
/// fear.drive = DriveLevel::new(0.4);
/// fear.update_priority();
/// // Fear has a 2.0x multiplier, so priority = 0.4 * 2.0 = 0.8
/// assert!((fear.priority - 0.8).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instinct {
    pub instinct_type: InstinctType,
    pub drive: DriveLevel,
    pub priority: f32, // 0.0-1.0, how urgently this instinct demands behavior
}

impl Instinct {
    #[must_use]
    pub fn new(instinct_type: InstinctType) -> Self {
        Self {
            instinct_type,
            drive: DriveLevel::new(0.5),
            priority: 0.5,
        }
    }

    /// Update priority based on drive level and time since last satisfied.
    pub fn update_priority(&mut self) {
        self.priority = self.drive.value()
            * match self.instinct_type {
                InstinctType::Fear => 2.0,   // fear overrides everything
                InstinctType::Hunger => 1.5, // hunger is urgent
                InstinctType::Thirst => 1.8, // thirst even more urgent
                InstinctType::Rest => 1.0,
                InstinctType::Reproduction => 0.8, // can wait
                InstinctType::Curiosity => 0.5,    // lowest priority
                _ => 1.0,
            };
        self.priority = self.priority.clamp(0.0, 1.0);
    }
}

/// Select the dominant instinct from a set (highest priority).
///
/// ```
/// use jantu::instinct::{Instinct, InstinctType, DriveLevel, dominant_instinct};
///
/// let mut hunger = Instinct::new(InstinctType::Hunger);
/// hunger.priority = 0.8;
/// let mut rest = Instinct::new(InstinctType::Rest);
/// rest.priority = 0.3;
///
/// let instincts = [hunger, rest];
/// let dom = dominant_instinct(&instincts).unwrap();
/// assert_eq!(dom.instinct_type, InstinctType::Hunger);
/// ```
#[must_use]
pub fn dominant_instinct(instincts: &[Instinct]) -> Option<&Instinct> {
    instincts.iter().max_by(|a, b| {
        a.priority
            .partial_cmp(&b.priority)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_level_clamps() {
        assert_eq!(DriveLevel::new(1.5).value(), 1.0);
        assert_eq!(DriveLevel::new(-0.5).value(), 0.0);
    }

    #[test]
    fn drive_critical() {
        assert!(DriveLevel::new(0.9).is_critical());
        assert!(!DriveLevel::new(0.5).is_critical());
    }

    #[test]
    fn drive_sated() {
        assert!(DriveLevel::new(0.1).is_sated());
        assert!(!DriveLevel::new(0.5).is_sated());
    }

    #[test]
    fn fear_highest_priority() {
        let mut fear = Instinct::new(InstinctType::Fear);
        fear.drive = DriveLevel::new(0.9);
        fear.update_priority();

        let mut curiosity = Instinct::new(InstinctType::Curiosity);
        curiosity.drive = DriveLevel::new(0.9);
        curiosity.update_priority();

        assert!(fear.priority > curiosity.priority);
    }

    #[test]
    fn dominant_instinct_selection() {
        let mut hunger = Instinct::new(InstinctType::Hunger);
        hunger.priority = 0.8;
        let mut rest = Instinct::new(InstinctType::Rest);
        rest.priority = 0.3;

        let instincts = [hunger.clone(), rest];
        let dom = dominant_instinct(&instincts).unwrap();
        assert_eq!(dom.instinct_type, InstinctType::Hunger);
    }

    #[test]
    fn serde_roundtrip_drive_level() {
        let d = DriveLevel::new(0.75);
        let json = serde_json::to_string(&d).unwrap();
        let d2: DriveLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(d, d2);
    }

    #[test]
    fn serde_roundtrip_instinct_type() {
        for it in [
            InstinctType::Hunger,
            InstinctType::Thirst,
            InstinctType::Fear,
            InstinctType::Aggression,
            InstinctType::Reproduction,
            InstinctType::Nurturing,
            InstinctType::Curiosity,
            InstinctType::Social,
            InstinctType::Rest,
        ] {
            let json = serde_json::to_string(&it).unwrap();
            let it2: InstinctType = serde_json::from_str(&json).unwrap();
            assert_eq!(it, it2);
        }
    }

    #[test]
    fn serde_roundtrip_instinct() {
        let mut i = Instinct::new(InstinctType::Fear);
        i.drive = DriveLevel::new(0.8);
        i.update_priority();
        let json = serde_json::to_string(&i).unwrap();
        let i2: Instinct = serde_json::from_str(&json).unwrap();
        assert_eq!(i.instinct_type, i2.instinct_type);
        assert_eq!(i.drive, i2.drive);
        assert!((i.priority - i2.priority).abs() < f32::EPSILON);
    }

    #[test]
    fn drive_increase_decrease() {
        let mut d = DriveLevel::new(0.5);
        d.increase(0.3);
        assert!((d.value() - 0.8).abs() < f32::EPSILON);
        d.decrease(0.5);
        assert!((d.value() - 0.3).abs() < f32::EPSILON);
    }
}
