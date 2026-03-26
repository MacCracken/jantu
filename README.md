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

- **`logging`** — Enables tracing-subscriber for `JANTU_LOG` env-filter logging
- **`personality`** — Enables [bhava](https://github.com/MacCracken/bhava) personality integration

## Design Principles

- `#[non_exhaustive]` on all public enums for forward compatibility
- `#[must_use]` on all pure functions
- All types implement `Serialize` + `Deserialize` (serde)
- Zero `unwrap`/`panic` in library code
- Feature-gated optional modules

## License

GPL-3.0-only
