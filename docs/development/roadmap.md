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
- [x] Learning/habituation — dual-process model (Groves & Thompson 1970)
- [x] Circadian rhythms — sinusoidal activity patterns with zeitgeber entrainment
- [x] Emotional contagion — proximity-based fear/aggression spreading with rank modulation
- [x] Migration patterns — obligate/facultative/nomadic strategies with energy costs
- [x] Mate selection — fitness evaluation, handicap principle, sexual selection pressure
- [x] Predator-prey co-evolution — trait pressure, Red Queen dynamics, Holling Type II
- [x] Stress accumulation — acute/chronic/resilience three-axis model
- [x] Memory/familiarity — reinforcement-protected forgetting, neophobia, social recognition
- [x] Communication signals — modality-specific range, SNR detection, honest signaling
- [x] Genetic trait inheritance — polygenic behavioral genome, crossover, fitness evaluation

## Future

- [ ] WASM target support
- [ ] `no_std` compatibility for embedded simulation
- [ ] Benchmark-driven SIMD optimizations for batch creature updates
- [ ] Integration guides for kiran and joshua
- [ ] Doc-tests / examples for all public API items

## v1.0 Criteria

- All public API documented with examples
- Performance baselines established and defended by CI
- At least one consumer (kiran or joshua) running in production
