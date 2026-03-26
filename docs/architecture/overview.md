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
