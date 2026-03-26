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
    /// Create a new drive level, clamped to [0.0, 1.0].
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }
    /// Get the raw drive value.
    #[must_use]
    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
    /// Whether this drive is at critical level (> 0.8).
    #[must_use]
    #[inline]
    pub fn is_critical(&self) -> bool {
        self.0 > 0.8
    }
    /// Whether this drive is sated (< 0.2).
    #[must_use]
    #[inline]
    pub fn is_sated(&self) -> bool {
        self.0 < 0.2
    }

    /// Increase the drive level by `amount`, clamped to 1.0.
    ///
    /// ```
    /// use jantu::instinct::DriveLevel;
    ///
    /// let mut drive = DriveLevel::new(0.5);
    /// drive.increase(0.3);
    /// assert!((drive.value() - 0.8).abs() < f32::EPSILON);
    /// ```
    pub fn increase(&mut self, amount: f32) {
        self.0 = (self.0 + amount).clamp(0.0, 1.0);
    }

    /// Decrease the drive level by `amount`, clamped to 0.0.
    ///
    /// ```
    /// use jantu::instinct::DriveLevel;
    ///
    /// let mut drive = DriveLevel::new(0.5);
    /// drive.decrease(0.4);
    /// assert!((drive.value() - 0.1).abs() < f32::EPSILON);
    /// ```
    pub fn decrease(&mut self, amount: f32) {
        self.0 = (self.0 - amount).clamp(0.0, 1.0);
    }
}

/// Core instinct types (Tinbergen's four questions mapped to drives).
///
/// # Examples
///
/// ```
/// use jantu::instinct::InstinctType;
///
/// let drive = InstinctType::Hunger;
/// assert_ne!(drive, InstinctType::Fear);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InstinctType {
    /// Foraging drive.
    Hunger,
    /// Water-seeking drive.
    Thirst,
    /// Threat avoidance.
    Fear,
    /// Territorial/competitive drive.
    Aggression,
    /// Mate-seeking drive.
    Reproduction,
    /// Offspring care.
    Nurturing,
    /// Exploration and play.
    Curiosity,
    /// Group affiliation.
    Social,
    /// Energy conservation.
    Rest,
}

/// Species-specific priority multipliers for instinct types.
///
/// Default weights reflect general mammalian priorities:
/// fear (2.0) > thirst (1.8) > hunger (1.5) > rest/social/nurturing (1.0)
/// > reproduction (0.8) > aggression (0.7) > curiosity (0.5).
///
/// # Examples
///
/// ```
/// use jantu::instinct::{PriorityWeights, InstinctType};
///
/// let w = PriorityWeights::default();
/// assert!(w.for_type(InstinctType::Fear) > w.for_type(InstinctType::Hunger));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityWeights {
    /// Fear priority multiplier (default 2.0).
    pub fear: f32,
    /// Hunger priority multiplier (default 1.5).
    pub hunger: f32,
    /// Thirst priority multiplier (default 1.8).
    pub thirst: f32,
    /// Aggression priority multiplier (default 0.7).
    pub aggression: f32,
    /// Reproduction priority multiplier (default 0.8).
    pub reproduction: f32,
    /// Nurturing priority multiplier (default 1.0).
    pub nurturing: f32,
    /// Curiosity priority multiplier (default 0.5).
    pub curiosity: f32,
    /// Social priority multiplier (default 1.0).
    pub social: f32,
    /// Rest priority multiplier (default 1.0).
    pub rest: f32,
}

impl Default for PriorityWeights {
    fn default() -> Self {
        Self {
            fear: 2.0,
            hunger: 1.5,
            thirst: 1.8,
            aggression: 0.7,
            reproduction: 0.8,
            nurturing: 1.0,
            curiosity: 0.5,
            social: 1.0,
            rest: 1.0,
        }
    }
}

impl PriorityWeights {
    /// Look up the weight for an instinct type.
    #[must_use]
    pub fn for_type(&self, instinct_type: InstinctType) -> f32 {
        match instinct_type {
            InstinctType::Fear => self.fear,
            InstinctType::Hunger => self.hunger,
            InstinctType::Thirst => self.thirst,
            InstinctType::Aggression => self.aggression,
            InstinctType::Reproduction => self.reproduction,
            InstinctType::Nurturing => self.nurturing,
            InstinctType::Curiosity => self.curiosity,
            InstinctType::Social => self.social,
            InstinctType::Rest => self.rest,
        }
    }
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
    /// Which instinct this represents.
    pub instinct_type: InstinctType,
    /// Current drive level (0.0 = sated, 1.0 = critical).
    pub drive: DriveLevel,
    /// Computed priority (0.0–1.0) — how urgently this instinct demands behavior.
    pub priority: f32,
}

impl Instinct {
    /// Create a new instinct with default drive (0.5) and priority (0.5).
    #[must_use]
    pub fn new(instinct_type: InstinctType) -> Self {
        Self {
            instinct_type,
            drive: DriveLevel::new(0.5),
            priority: 0.5,
        }
    }

    /// Update priority using default weights.
    ///
    /// For species-specific tuning, use [`update_priority_with`](Self::update_priority_with).
    pub fn update_priority(&mut self) {
        self.update_priority_with(&PriorityWeights::default());
    }

    /// Update priority using custom weights.
    ///
    /// ```
    /// use jantu::instinct::{Instinct, InstinctType, DriveLevel, PriorityWeights};
    ///
    /// let mut hunger = Instinct::new(InstinctType::Hunger);
    /// hunger.drive = DriveLevel::new(0.6);
    ///
    /// let mut weights = PriorityWeights::default();
    /// weights.hunger = 3.0; // herbivore: hunger dominates
    /// hunger.update_priority_with(&weights);
    /// assert!(hunger.priority > 0.5);
    /// ```
    pub fn update_priority_with(&mut self, weights: &PriorityWeights) {
        self.priority = self.drive.value() * weights.for_type(self.instinct_type);
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
            .unwrap_or(core::cmp::Ordering::Equal)
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
