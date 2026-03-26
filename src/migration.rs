//! Seasonal migration patterns and movement behaviors.
//!
//! Models migratory drives that vary with season, navigation strategies,
//! and the energetic costs of long-distance movement. Supports both
//! obligate (hardwired) and facultative (condition-dependent) migration.

use serde::{Deserialize, Serialize};

/// Migration strategy classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MigrationStrategy {
    /// Fixed annual migration regardless of conditions (e.g., Arctic tern).
    Obligate,
    /// Migrates only when conditions deteriorate (e.g., some songbirds).
    Facultative,
    /// Partial migration — some individuals migrate, others don't (e.g., robins).
    Partial,
    /// No migration — resident year-round.
    Sedentary,
    /// Nomadic wandering without fixed routes (e.g., crossbills following food).
    Nomadic,
}

/// Season of the year (for driving migration triggers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

/// Current phase of a migration cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MigrationPhase {
    /// At breeding/summer grounds.
    Resident,
    /// Preparing to depart (hyperphagia, restlessness).
    PreMigration,
    /// Actively migrating.
    EnRoute,
    /// At wintering/destination grounds.
    Overwintering,
    /// Returning to breeding grounds.
    Returning,
}

/// Navigation method used during migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NavigationMethod {
    /// Sun compass (diurnal migrants).
    SolarCompass,
    /// Star compass (nocturnal migrants).
    StellarCompass,
    /// Magnetic field sensing.
    Magnetoreception,
    /// Landmark following (rivers, coastlines, mountain ranges).
    Landmarks,
    /// Social learning — following experienced individuals.
    SocialLearning,
}

/// Compute the migratory urge based on season and environmental conditions.
///
/// ```
/// use jantu::migration::{MigrationStrategy, migratory_urge};
///
/// // Sedentary creatures never migrate
/// assert_eq!(migratory_urge(180, 0.5, 0.5, MigrationStrategy::Sedentary), 0.0);
///
/// // Bad conditions increase facultative urge
/// let good = migratory_urge(180, 0.9, 0.8, MigrationStrategy::Facultative);
/// let bad = migratory_urge(180, 0.1, 0.2, MigrationStrategy::Facultative);
/// assert!(bad > good);
/// ```
///
/// - `day_of_year`: 0-365, used to compute seasonal drive
/// - `food_availability`: 0.0-1.0 (low food increases facultative urge)
/// - `temperature`: relative temperature 0.0-1.0 (0=freezing, 1=warm)
/// - `strategy`: the creature's migration strategy
///
/// Returns a migration drive (0.0-1.0) where higher = stronger urge to migrate.
#[must_use]
pub fn migratory_urge(
    day_of_year: u16,
    food_availability: f32,
    temperature: f32,
    strategy: MigrationStrategy,
) -> f32 {
    let food_availability = food_availability.clamp(0.0, 1.0);
    let temperature = temperature.clamp(0.0, 1.0);

    match strategy {
        MigrationStrategy::Sedentary => 0.0,
        MigrationStrategy::Obligate => {
            // Pure photoperiod response: peaks in autumn (day ~270) and spring (day ~90)
            let phase = std::f32::consts::TAU * day_of_year as f32 / 365.0;
            // Two peaks per year (outbound and return)
            let seasonal = (2.0 * phase).cos().abs();
            seasonal * 0.8 + 0.2 // never fully zero for obligate
        }
        MigrationStrategy::Facultative => {
            // Condition-dependent: low food + cold → migrate
            let food_stress = 1.0 - food_availability;
            let cold_stress = 1.0 - temperature;
            (food_stress * 0.6 + cold_stress * 0.4).clamp(0.0, 1.0)
        }
        MigrationStrategy::Partial => {
            // Blend of obligate and facultative
            let phase = std::f32::consts::TAU * day_of_year as f32 / 365.0;
            let seasonal = (2.0 * phase).cos().abs() * 0.4;
            let condition = (1.0 - food_availability) * 0.3 + (1.0 - temperature) * 0.3;
            (seasonal + condition).clamp(0.0, 1.0)
        }
        MigrationStrategy::Nomadic => {
            // Purely resource-driven, no seasonal component
            (1.0 - food_availability).clamp(0.0, 1.0)
        }
    }
}

/// Compute the energetic cost of migration per unit distance.
///
/// - `body_mass_kg`: creature mass
/// - `flight_capable`: whether the creature can fly (flight is more efficient per km)
/// - `headwind`: 0.0 = no wind, 1.0 = strong headwind (increases cost)
///
/// Returns relative energy cost multiplier (1.0 = baseline).
#[must_use]
pub fn migration_energy_cost(body_mass_kg: f32, flight_capable: bool, headwind: f32) -> f32 {
    if body_mass_kg <= 0.0 {
        return 0.0;
    }
    let headwind = headwind.clamp(0.0, 1.0);

    // Larger animals: lower per-kg cost (scaling ~M^0.7 / M = M^-0.3)
    let base_cost = body_mass_kg.powf(-0.3);

    // Flight reduces cost by ~60% compared to terrestrial locomotion
    let locomotion = if flight_capable { 0.4 } else { 1.0 };

    base_cost * locomotion * (1.0 + headwind * 0.5)
}

/// Determine the current season from day of year.
#[must_use]
pub fn season_from_day(day_of_year: u16) -> Season {
    match day_of_year % 365 {
        0..=79 => Season::Winter,
        80..=171 => Season::Spring,
        172..=263 => Season::Summer,
        264..=354 => Season::Autumn,
        _ => Season::Winter,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sedentary_never_migrates() {
        for day in [0, 90, 180, 270] {
            let urge = migratory_urge(day, 0.5, 0.5, MigrationStrategy::Sedentary);
            assert_eq!(urge, 0.0);
        }
    }

    #[test]
    fn obligate_always_has_urge() {
        for day in [0, 90, 180, 270] {
            let urge = migratory_urge(day, 1.0, 1.0, MigrationStrategy::Obligate);
            assert!(
                urge > 0.0,
                "obligate should always have some urge, day={day}"
            );
        }
    }

    #[test]
    fn facultative_responds_to_conditions() {
        let good = migratory_urge(180, 0.9, 0.8, MigrationStrategy::Facultative);
        let bad = migratory_urge(180, 0.1, 0.2, MigrationStrategy::Facultative);
        assert!(
            bad > good,
            "bad conditions should increase urge: good={good}, bad={bad}"
        );
    }

    #[test]
    fn nomadic_follows_food() {
        let abundant = migratory_urge(0, 0.9, 0.5, MigrationStrategy::Nomadic);
        let scarce = migratory_urge(0, 0.1, 0.5, MigrationStrategy::Nomadic);
        assert!(scarce > abundant);
    }

    #[test]
    fn flight_cheaper_than_walking() {
        let flying = migration_energy_cost(1.0, true, 0.0);
        let walking = migration_energy_cost(1.0, false, 0.0);
        assert!(flying < walking);
    }

    #[test]
    fn headwind_increases_cost() {
        let calm = migration_energy_cost(1.0, true, 0.0);
        let windy = migration_energy_cost(1.0, true, 0.8);
        assert!(windy > calm);
    }

    #[test]
    fn zero_mass_safe() {
        assert_eq!(migration_energy_cost(0.0, true, 0.0), 0.0);
    }

    #[test]
    fn season_mapping() {
        assert_eq!(season_from_day(15), Season::Winter);
        assert_eq!(season_from_day(100), Season::Spring);
        assert_eq!(season_from_day(200), Season::Summer);
        assert_eq!(season_from_day(300), Season::Autumn);
    }

    #[test]
    fn serde_roundtrip_migration_strategy() {
        for s in [
            MigrationStrategy::Obligate,
            MigrationStrategy::Facultative,
            MigrationStrategy::Partial,
            MigrationStrategy::Sedentary,
            MigrationStrategy::Nomadic,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let s2: MigrationStrategy = serde_json::from_str(&json).unwrap();
            assert_eq!(s, s2);
        }
    }

    #[test]
    fn serde_roundtrip_season() {
        for s in [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let s2: Season = serde_json::from_str(&json).unwrap();
            assert_eq!(s, s2);
        }
    }

    #[test]
    fn serde_roundtrip_migration_phase() {
        for p in [
            MigrationPhase::Resident,
            MigrationPhase::PreMigration,
            MigrationPhase::EnRoute,
            MigrationPhase::Overwintering,
            MigrationPhase::Returning,
        ] {
            let json = serde_json::to_string(&p).unwrap();
            let p2: MigrationPhase = serde_json::from_str(&json).unwrap();
            assert_eq!(p, p2);
        }
    }

    #[test]
    fn serde_roundtrip_navigation_method() {
        for n in [
            NavigationMethod::SolarCompass,
            NavigationMethod::StellarCompass,
            NavigationMethod::Magnetoreception,
            NavigationMethod::Landmarks,
            NavigationMethod::SocialLearning,
        ] {
            let json = serde_json::to_string(&n).unwrap();
            let n2: NavigationMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(n, n2);
        }
    }
}
