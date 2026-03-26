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

pub mod error;
pub mod instinct;
pub mod lifecycle;
pub mod pack;
pub mod social;
pub mod survival;
pub mod swarm;
pub mod territory;

#[cfg(feature = "logging")]
pub mod logging;

pub use error::{JantuError, Result};
pub use instinct::{DriveLevel, Instinct, InstinctType};
pub use social::{HierarchyPosition, SocialRole};
pub use survival::{SurvivalState, ThreatResponse};
pub use swarm::SwarmBehavior;
