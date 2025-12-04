//! Effect system for LED panel animations.
//!
//! Provides a trait-based system for creating and managing visual effects.

mod effects;
mod manager;
mod traits;

pub use effects::*;
pub use manager::EffectManager;
pub use traits::{Effect, EffectConfig, EffectParams};
