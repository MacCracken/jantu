# Architecture Overview

## Module Map

```
jantu (lib.rs)
├── error        — JantuError enum, Result<T> alias
├── instinct     — DriveLevel, InstinctType, Instinct, dominant_instinct()
├── survival     — SurvivalState, ThreatResponse, select_threat_response()
├── territory    — TerritoryMark, territorial_response()
├── social       — SocialRole, HierarchyPosition, group_cohesion()
├── swarm        — SwarmBehavior, Pheromone, PheromoneType, path/quorum functions
├── pack         — HuntPhase, hunt_success_probability(), food_share()
├── lifecycle    — LifeStage, Kleiber's law scaling (BMR, lifespan, heart rate)
├── habituation  — StimulusResponse, HabituationParams, dual-process learning
├── circadian    — CircadianClock, ActivityPattern, zeitgeber entrainment
├── contagion    — EmotionalState, Susceptibility, group emotional dynamics
├── migration    — MigrationStrategy, Season, migratory urge, energy costs
├── mating       — FitnessTraits, MatingSystem, courtship, sexual selection
├── coevolution  — TraitMatchup, predator-prey arms race, Holling Type II
├── stress       — StressState, acute/chronic model, drive modifiers
├── memory       — MemoryTrace, MemoryType, spatial/social recognition
├── signals      — Signal, SignalModality, detection/response functions
├── genetics     — BehavioralGenome, HeritableTrait, crossover, fitness
├── foraging     — Optimal diet, marginal value theorem, giving-up density, vigilance
├── gametheory   — Hawk-dove ESS, bourgeois strategy, war of attrition, producer-scrounger
├── kin          — Hamilton's rule, relatedness, inclusive fitness, alarm calling
├── landscape    — Perceived risk, fear-foraging tradeoff, group dilution, habitat value
└── logging      — [feature: logging] tracing-subscriber init
```

## Data Flow

Jantu is a stateless computation library. It provides types and pure functions — callers own the state and decide when to invoke behavior calculations.

Typical consumer flow:

1. **Create** creature state (instincts, drives, position, social role)
2. **Update** drives each tick (hunger increases, fear spikes on threat detection)
3. **Query** jantu functions for behavioral decisions:
   - `dominant_instinct()` → what should the creature do?
   - `select_threat_response()` → how should it react to danger?
   - `hunt_success_probability()` → should the pack attack?
   - `territorial_response()` → fight or retreat from intruder?
   - `circadian::activity_level()` → how active should the creature be now?
   - `contagion::emotional_influence()` → is fear spreading through the group?
   - `stress::stress_drive_modifier()` → how is chronic stress shifting priorities?
   - `memory::social_recognition()` → does the creature recognize this individual?
   - `genetics::genome_fitness()` → how well-adapted is this creature?
4. **Apply** decisions in the consumer's simulation loop

## Consumers

| Consumer | Usage |
|----------|-------|
| **kiran** (game engine) | Creature AI behaviors, NPC instincts |
| **joshua** (simulation) | Population dynamics, predator-prey modeling |
| **AGNOS science stack** | Ethology research, behavioral modeling |

## Design Decisions

- **Flat library crate**: no nested module hierarchy — every module is one level deep from lib.rs for discoverability
- **Pure functions over stateful objects**: callers manage state, jantu computes. This makes jantu trivially parallelizable and testable
- **f32 throughout**: game/simulation use case prioritizes throughput over precision. All clamped wrapper types enforce valid ranges at construction
- **Feature gates**: `logging` and `personality` are optional to keep the base dependency tree minimal
