//! # Jantu
//!
//! **Jantu** (जन्तु — Sanskrit for "creature, living being") — ethology and creature
//! behavior engine for the AGNOS ecosystem.
//!
//! Provides instinct modeling, survival drives, territorial behavior, social hierarchy,
//! swarm intelligence, pack dynamics, and creature lifecycle. The biological foundation
//! layer that bhava's human personality builds upon — the medulla oblongata of the
//! personality stack.
//!
//! ## Relationship to Bhava
//!
//! Bhava models the human mind (personality, emotion, reasoning). Jantu models the
//! animal brain underneath — the instincts that evolution preserved. Human emotions
//! *are* refined animal instincts:
//!
//! - `bhava::stress` → jantu fight-or-flight (reptilian)
//! - `bhava::relationship` → jantu pair bonding (mammalian)
//! - `bhava::contagion` → jantu emotional contagion (pack behavior)
//! - `bhava::flow` → jantu predatory focus (hunting state)
//! - `bhava::energy` → jantu foraging/resting cycles

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
extern crate alloc;

/// Cross-crate bridges — primitive-value conversions from other AGNOS science crates.
pub mod bridge;
/// Circadian rhythm drive modifiers.
pub mod circadian;
/// Predator-prey co-evolution hooks.
pub mod coevolution;
/// Emotional contagion — affective state spreading through groups.
pub mod contagion;
/// Error types and result alias.
pub mod error;
/// Optimal foraging theory — prey selection, patch departure, and vigilance.
pub mod foraging;
/// Evolutionary game theory — contest strategies and stable equilibria.
pub mod gametheory;
/// Genetic trait inheritance — heritable behavior parameters.
pub mod genetics;
/// Learning through habituation and sensitization (dual-process theory).
pub mod habituation;
/// Core instinct system — drives, priorities, and dominant instinct selection.
pub mod instinct;
/// Integration APIs for downstream consumers (soorat rendering).
pub mod integration;
/// Kin selection and inclusive fitness (Hamilton 1964).
pub mod kin;
/// Landscape of fear — spatial risk perception and fear-mediated behavior.
pub mod landscape;
/// Allometric lifecycle scaling (Kleiber's law).
pub mod lifecycle;
/// Mate selection and courtship behaviors.
pub mod mating;
/// Spatial and social memory — recognition of locations, individuals, and threats.
pub mod memory;
/// Seasonal migration patterns and movement behaviors.
pub mod migration;
/// Pack hunting coordination and food sharing.
pub mod pack;
/// Communication signals — alarm calls, mating calls, territorial displays.
pub mod signals;
/// Social hierarchy, roles, and group cohesion.
pub mod social;
/// Stress accumulation and its effects on drive baselines.
pub mod stress;
/// Survival states and threat response classification.
pub mod survival;
/// Swarm intelligence — pheromone trails, quorum sensing, collective behavior.
pub mod swarm;
/// Territory marking and territorial aggression.
pub mod territory;

/// Optional tracing subscriber initialization. Requires the `logging` feature.
#[cfg(feature = "logging")]
pub mod logging;

pub use error::{JantuError, Result};
pub use instinct::{DriveLevel, Instinct, InstinctType, PriorityWeights};
pub use social::{HierarchyPosition, SocialRole};
pub use survival::{SurvivalState, ThreatResponse};
pub use swarm::SwarmBehavior;
