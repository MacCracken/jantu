# Roadmap

## Completed

- [x] Core instinct system (DriveLevel, InstinctType, priority-based selection)
- [x] Survival and threat response (4F model)
- [x] Territory marking with decay
- [x] Social hierarchy (roles, contests, group cohesion)
- [x] Swarm intelligence (pheromones, path selection, quorum sensing)
- [x] Pack hunting (success probability, food sharing)
- [x] Lifecycle scaling (Kleiber's law: BMR, lifespan, heart rate)
- [x] Error types with thiserror
- [x] Serde on all types
- [x] Optional logging (tracing)
- [x] Optional bhava personality integration
- [x] Criterion benchmarks for all core functions
- [x] Serde roundtrip tests for all types
- [x] Integration tests

## Backlog

- [ ] Learning/habituation — creatures adapt to repeated stimuli
- [ ] Circadian rhythms — time-of-day drive modifiers
- [ ] Emotional contagion — fear/aggression spreading through groups
- [ ] Migration patterns — seasonal movement behaviors
- [ ] Mate selection — fitness evaluation and courtship behaviors
- [ ] Predator-prey co-evolution hooks — arms race dynamics
- [ ] Stress accumulation — chronic stress affecting drive baselines
- [ ] Memory/familiarity — recognition of locations, individuals, threats
- [ ] Communication signals — alarm calls, mating calls, territorial displays
- [ ] Genetic trait inheritance — heritable behavior parameters

## Future

- [ ] WASM target support
- [ ] `no_std` compatibility for embedded simulation
- [ ] Benchmark-driven SIMD optimizations for batch creature updates
- [ ] Integration guides for kiran and joshua

## v1.0 Criteria

- All backlog items completed or explicitly deferred with ADR
- 100% serde roundtrip coverage
- All public API documented with examples
- Performance baselines established and defended by CI
- At least one consumer (kiran or joshua) running in production
