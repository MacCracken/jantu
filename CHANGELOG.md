# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- **habituation** module: `StimulusResponse` with dual-process (Groves & Thompson 1970) habituation/sensitization, `HabituationParams` config, `dishabituation_boost()`, `generalized_habituation()` with quadratic similarity falloff. Intensity-dampened habituation rate ensures strong stimuli sensitize rather than habituate
- **circadian** module: `CircadianClock` with sinusoidal activity oscillator, `ActivityPattern` enum (Diurnal/Nocturnal/Crepuscular/Cathemeral), `drive_modifier()` for rest-inverse scaling, `zeitgeber_correction()` for light-cycle entrainment with wraparound. Supports custom periods for non-Earth environments
- **contagion** module: `EmotionalState` enum (Fear/Aggression/Calm/Excitement), `Susceptibility` profile with rank/arousal modulation, `emotional_influence()` with inverse-square proximity falloff and rank boost, `aggregate_pressure()` for group-level emotional dominance, `contagion_transfer()` with state-match amplification
- **migration** module: `MigrationStrategy` enum (Obligate/Facultative/Partial/Sedentary/Nomadic), `Season`, `MigrationPhase`, `NavigationMethod` enums, `migratory_urge()` with strategy-specific seasonal/condition triggers, `migration_energy_cost()` with allometric scaling and flight efficiency, `season_from_day()`
- **mating** module: `MatingSystem` enum (Monogamous/Polygynous/Polyandrous/Promiscuous/Lek), `CourtshipPhase` enum (6 phases), `FitnessTraits` with weighted attractiveness scoring, `mate_acceptance()` with competition-adjusted thresholds, `display_cost()` implementing Zahavi's handicap principle (quadratic cost), `selection_pressure()` from operational sex ratio
- **coevolution** module: `EcologicalRole` enum, `ArmsRaceTrait` enum (6 trait categories), `TraitMatchup` with predator/prey advantage calculation, `trait_pressure()` with deficit-driven selection and diminishing returns, `red_queen_balance()`, `encounter_rate()` (Lotka-Volterra), `functional_response_type2()` (Holling Type II saturation)
- **stress** module: `StressorType` enum (6 categories), `StressState` with acute/chronic/resilience three-axis model (McEwen & Stellar 1993), `stress_drive_modifier()` amplifying anxiety drives and suppressing luxury drives, `immune_function()` computing immunosuppression from chronic stress
- **memory** module: `MemoryType` enum (7 categories), `MemoryTrace` with reinforcement-protected power-law forgetting, `neophobia_modifier()`, `spatial_reliability()` accounting for environment stability, `social_recognition()` with asymptotic encounter buildup and exponential decay
- **signals** module: `SignalModality` enum (6 channels), `SignalFunction` enum (8 purposes), `Signal` struct with honesty scoring, `signal_range()` with modality-specific propagation, `signal_cost()` implementing handicap principle, `detection_probability()` with sigmoid SNR function, `receiver_response()` with trust/familiarity/relevance weighting
- **genetics** module: `HeritableTrait` with genotype/heritability split and `phenotype()` expression, `BehavioralGenome` (5-trait personality: aggression/boldness/sociability/activity/exploration), `inherit_trait()` midparent blending with mutation, `crossover()` for full genome reproduction, `genome_fitness()` with weighted environment interaction
- 40 criterion benchmarks covering all modules (all sub-60ns)
- 166 unit tests + 6 integration tests, all passing

### Fixed

- Removed `unwrap()` in `survival::select_threat_response()` — replaced with `map_or` fallback
- Fixed deprecated license identifier `GPL-3.0` → `GPL-3.0-only` in `Cargo.toml`
- Fixed unnecessary parentheses clippy lint in `pack::food_share()`

## [0.1.0] - 2026-03-26

### Added

- **instinct** module: `DriveLevel` (clamped 0-1 wrapper), `InstinctType` enum (9 drive types based on Tinbergen's ethological framework), `Instinct` struct with priority-weighted update system, `dominant_instinct()` selector
- **survival** module: `SurvivalState` enum (7 states), `ThreatResponse` enum implementing the 4F model (fight/flight/freeze/fawn), `select_threat_response()` with trait-based scoring
- **territory** module: `TerritoryMark` struct with decay and activity detection, `territorial_response()` aggression modifier
- **social** module: `SocialRole` enum (10 roles), `HierarchyPosition` (clamped 0-1 wrapper) with dominance contests, `group_cohesion()` distance-based calculation
- **swarm** module: `SwarmBehavior` enum, `Pheromone` struct with evaporation, `PheromoneType` enum, `path_selection_probability()` for ant-colony-style routing, `quorum_reached()` for collective decision-making
- **pack** module: `HuntPhase` enum (6 phases), `hunt_success_probability()` sigmoid with diminishing returns (capped at 95%), `food_share()` rank-based distribution
- **lifecycle** module: `LifeStage` enum (6 stages), Kleiber's law implementations for `basal_metabolic_rate()` (M^0.75), `estimated_lifespan_years()` (M^0.25), `heart_rate_bpm()` (M^-0.25)
- **error** module: `JantuError` enum with `thiserror` integration
- **logging** feature: optional tracing-subscriber with `JANTU_LOG` env filter
- **personality** feature: optional bhava personality system integration
- Serde `Serialize`/`Deserialize` on all types
- `#[non_exhaustive]` on all public enums
- `#[must_use]` on all pure functions
- Criterion benchmarks for all core functions
- Serde roundtrip tests for all types
- Integration tests covering cross-module creature behavior scenarios
