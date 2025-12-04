//! Built-in effects.

mod fire;
mod solid;

pub use fire::FireEffect;
pub use solid::SolidEffect;

use crate::Effect;

/// Create an effect by name.
pub fn create_effect(name: &str) -> Option<Box<dyn Effect>> {
    match name {
        "fire" => Some(Box::new(FireEffect::new())),
        "solid" => Some(Box::new(SolidEffect::new())),
        "off" => Some(Box::new(SolidEffect::off())),
        _ => None,
    }
}

/// List all available effect names.
pub fn available_effects() -> &'static [&'static str] {
    &["fire", "solid", "off"]
}
