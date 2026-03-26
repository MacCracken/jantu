# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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
