# Usage Guide

Jantu models creature behavior through composable modules. Each module handles one ethological domain (instinct, survival, territory, etc.) and exposes pure functions that take creature state as input and return drive modifiers, probabilities, or state updates.

## Quick Start

Add jantu to your `Cargo.toml`:

```toml
[dependencies]
jantu = "1"
```

For `no_std` environments (embedded, WASM), jantu works out of the box with no default features. Enable `std` only if you need the `logging` feature:

```toml
jantu = { version = "1", features = ["std", "logging"] }
```

## Core Pattern: Drive-Based Behavior

Creatures are driven by instincts with priority levels. The dominant instinct determines behavior each tick.

```rust
use jantu::instinct::{Instinct, InstinctType, DriveLevel, dominant_instinct};

// Create a creature's instinct set
let mut hunger = Instinct::new(InstinctType::Hunger);
hunger.drive = DriveLevel::new(0.7);
hunger.update_priority();

let mut fear = Instinct::new(InstinctType::Fear);
fear.drive = DriveLevel::new(0.3);
fear.update_priority();

let instincts = [hunger, fear];
let dominant = dominant_instinct(&instincts).unwrap();
// Hunger wins — creature forages
assert_eq!(dominant.instinct_type, InstinctType::Hunger);
```

## Threat Response

When a creature detects danger, its traits determine the response strategy:

```rust
use jantu::survival::select_threat_response;

let response = select_threat_response(
    0.2,  // aggression (low)
    0.9,  // speed (high)
    0.5,  // relative_size (smaller than threat)
    0.5,  // social_rank
);
// Fast, timid creature flees
```

## Circadian Rhythms

Modulate drives based on time of day:

```rust
use jantu::circadian::{CircadianClock, ActivityPattern};

let clock = CircadianClock::new(ActivityPattern::Nocturnal);

// At midnight: high activity, low rest drive
let activity = clock.activity_level(0.0);
let rest_mod = clock.drive_modifier(0.0, true);

// At noon: low activity, high rest drive
let noon_rest = clock.drive_modifier(12.0, true);
assert!(noon_rest > rest_mod);
```

## Stress and Recovery

Stress accumulates from stressors and affects drive baselines:

```rust
use jantu::stress::{StressState, stress_drive_modifier};

let mut stress = StressState::new();

// Predator encounter
stress.apply_stressor(0.8);
assert!(stress.acute > 0.0);

// Chronic stress amplifies anxiety drives
let fear_mod = stress_drive_modifier(stress.chronic, true);
assert!(fear_mod >= 1.0);

// Safe environment allows recovery
stress.recover(10.0, 0.9);
```

## Genetics and Inheritance

Breed creatures with heritable behavioral traits:

```rust
use jantu::genetics::{BehavioralGenome, crossover, genome_fitness};

let parent_a = BehavioralGenome::default_genome();
let parent_b = BehavioralGenome::default_genome();

// Crossover with small mutations
let offspring = crossover(&parent_a, &parent_b, &[0.05, -0.02, 0.0, 0.03, -0.01]);

// Evaluate fitness in a specific environment
let fitness = genome_fitness(
    &offspring,
    &[0.3, 0.5, 0.2, 0.4, 0.1],  // trait weights
    &[0.5, 0.5, 0.5, 0.5, 0.5],  // environment
);
assert!(fitness > 0.0);
```

## Serialization

All types implement `Serialize` and `Deserialize`. Save and restore creature state:

```rust
use jantu::instinct::{Instinct, InstinctType, DriveLevel};

let mut creature = Instinct::new(InstinctType::Hunger);
creature.drive = DriveLevel::new(0.8);

let json = serde_json::to_string(&creature).unwrap();
let restored: Instinct = serde_json::from_str(&json).unwrap();
assert_eq!(restored.instinct_type, InstinctType::Hunger);
```

## Module Reference

| Module | Domain | Key Functions |
|--------|--------|---------------|
| `instinct` | Core drives | `dominant_instinct()` |
| `survival` | Threat response | `select_threat_response()` |
| `territory` | Marking & defense | `territorial_response()` |
| `social` | Hierarchy & groups | `group_cohesion()` |
| `swarm` | Collective behavior | `path_selection_probability()`, `quorum_reached()` |
| `pack` | Hunting & sharing | `hunt_success_probability()`, `food_share()` |
| `lifecycle` | Allometric scaling | `basal_metabolic_rate()` |
| `circadian` | Day/night rhythms | `activity_level()`, `zeitgeber_correction()` |
| `contagion` | Emotional spreading | `emotional_influence()`, `aggregate_pressure()` |
| `habituation` | Learning | `StimulusResponse`, `generalized_habituation()` |
| `migration` | Seasonal movement | `migratory_urge()`, `migration_energy_cost()` |
| `mating` | Sexual selection | `mate_acceptance()`, `display_cost()` |
| `coevolution` | Arms races | `trait_pressure()`, `functional_response_type2()` |
| `stress` | Allostatic load | `stress_drive_modifier()`, `immune_function()` |
| `memory` | Spatial & social | `neophobia_modifier()`, `social_recognition()` |
| `signals` | Communication | `signal_range()`, `detection_probability()` |
| `genetics` | Inheritance | `crossover()`, `genome_fitness()` |

## Feature Flags

| Feature | Effect |
|---------|--------|
| *(none)* | `no_std + alloc` — works everywhere including WASM |
| `std` | Enables standard library (required for `logging`) |
| `logging` | Tracing subscriber with `JANTU_LOG` env filter |
| `personality` | Bhava personality system integration |
