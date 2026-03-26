# Jantu

**Jantu** (Sanskrit: जन्तु — creature, living being) — ethology and creature behavior engine for the AGNOS ecosystem.

Provides instinct modeling, survival drives, territorial behavior, social hierarchy, swarm intelligence, pack dynamics, and creature lifecycle scaling. The biological foundation layer that [bhava](https://github.com/MacCracken/bhava)'s human personality builds upon.

## Modules

| Module | Description |
|--------|-------------|
| `instinct` | Drive levels, instinct types (Tinbergen), priority-based dominant instinct selection |
| `survival` | Survival states, threat response classification (fight/flight/freeze/fawn) |
| `territory` | Territory marking with decay, territorial aggression response |
| `social` | Social roles, dominance hierarchy, contests, group cohesion |
| `swarm` | Swarm behaviors, pheromone trails, path selection, quorum sensing |
| `pack` | Pack hunting phases, success probability (sigmoid), rank-based food sharing |
| `lifecycle` | Life stages, Kleiber's law metabolic scaling (BMR, lifespan, heart rate) |
| `habituation` | Dual-process learning: habituation, sensitization, dishabituation, stimulus generalization |
| `circadian` | Circadian rhythms: diurnal/nocturnal/crepuscular activity patterns, zeitgeber entrainment |
| `contagion` | Emotional contagion: fear/aggression spreading through groups, susceptibility profiles |
| `migration` | Seasonal migration: obligate/facultative/nomadic strategies, energy costs, navigation |
| `mating` | Mate selection: fitness evaluation, courtship displays, sexual selection pressure |
| `coevolution` | Predator-prey arms race: trait pressure, Red Queen dynamics, Holling Type II functional response |
| `stress` | Stress accumulation: acute/chronic model, allostatic load, immune suppression |
| `memory` | Spatial and social memory: reinforcement-protected forgetting, neophobia, recognition |
| `signals` | Communication signals: modality-specific range, detection probability, honest signaling costs |
| `genetics` | Heritable behavior: polygenic traits, phenotype expression, crossover, genome fitness |

## Usage

```rust
use jantu::instinct::{DriveLevel, Instinct, InstinctType, dominant_instinct};
use jantu::survival::select_threat_response;

// Model a creature's instincts
let mut hunger = Instinct::new(InstinctType::Hunger);
hunger.drive = DriveLevel::new(0.7);
hunger.update_priority();

let mut fear = Instinct::new(InstinctType::Fear);
fear.drive = DriveLevel::new(0.3);
fear.update_priority();

// Fear multiplier (2.0x) can override hunger (1.5x) even at lower drive
let instincts = [hunger, fear];
let dominant = dominant_instinct(&instincts);

// Threat response based on creature traits
let response = select_threat_response(
    0.8,  // aggression
    0.6,  // speed
    1.3,  // relative size (>1 = bigger than threat)
    0.9,  // social rank
);
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| *(none)* | yes | `no_std + alloc` — works on embedded targets and WASM |
| `std` | no | Enables standard library (required for `logging`) |
| `logging` | no | Tracing subscriber with `JANTU_LOG` env-filter |
| `personality` | no | [bhava](https://github.com/MacCracken/bhava) personality integration |

### Platform Support

- Native (all tier 1 targets)
- `wasm32-unknown-unknown` (verified)
- `no_std` environments with `alloc`

## Consumers

| Crate | Usage |
|-------|-------|
| [kiran](https://github.com/MacCracken/kiran) | Game engine — creature AI behaviors, NPC instincts |
| [joshua](https://github.com/MacCracken/joshua) | Simulation — population dynamics, predator-prey modeling |
| [bhava](https://github.com/MacCracken/bhava) | Human personality — builds on jantu's animal instinct layer |
| AGNOS science stack | Ethology research, behavioral modeling |

## Design Principles

- `no_std` by default — zero I/O, zero networking, minimal attack surface
- `#[non_exhaustive]` on all public enums for forward compatibility
- `#[must_use]` on all pure functions
- `#[warn(missing_docs)]` enforced at compile time
- All types implement `Serialize` + `Deserialize` (serde)
- Zero `unsafe` code, zero `unwrap`/`panic` in library code
- Feature-gated optional modules — consumers pull only what they need

## License

GPL-3.0-only
