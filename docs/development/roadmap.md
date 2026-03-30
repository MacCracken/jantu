# Roadmap

## Cross-Crate Bridges

- [ ] `bridge.rs` module — primitive-value conversions for cross-crate ethology/behavior
- [ ] **raasta bridge**: target position [f32; 3], obstacle positions → pathfinding waypoints; group centroid → crowd destination
- [ ] **vanaspati bridge**: canopy cover (0-1) → concealment factor; food source availability → foraging attraction weight
- [ ] **badal bridge**: temperature (°C) → activity level scaling; precipitation rate → shelter-seeking urgency
- [ ] **garjan bridge**: creature speed (m/s), size → footstep synthesis parameters; alarm call trigger → vocalization event

## Soorat Integration

- [ ] `integration/soorat.rs` module — feature-gated `soorat-compat`
- [ ] **Agent positions**: creature positions, headings, and behavioral state for instanced particle/sprite rendering
- [ ] **Territory boundaries**: territorial region polygons with ownership for colored overlay rendering
- [ ] **Migration paths**: waypoint sequences with timing for animated line rendering
- [ ] **Social graph**: dominance/affiliation edges between agents for node-link visualization
- [ ] **Swarm field**: density/velocity field from swarm simulation for heatmap/flow rendering

## Future

- [ ] Boids/flocking (Reynolds 1986) — vertebrate herding, schooling, flocking
- [ ] Play behavior — locomotor/object/social play, context-dependent activation
- [ ] Thermoregulation — ectotherm shuttling, endotherm energy costs, huddling
- [ ] Parasite behavioral manipulation — host behavior modification by parasites
- [ ] Ideal free distribution (Fretwell & Lucas 1970) — habitat selection model
- [ ] Producer-scrounger dynamics — frequency-dependent foraging strategies
- [ ] Sleep/torpor (Borbely two-process) — sleep pressure, hibernation, ultradian rhythms
- [ ] Coalition formation and reciprocal altruism (Trivers 1971)
- [ ] Behavioral inertia / hysteresis in instinct switching
- [ ] Benchmark-driven SIMD optimizations for batch creature updates
- [ ] Integration guides for kiran and joshua
- [ ] Performance baselines defended by CI
