//! Effect trait definition.

use serde::{Deserialize, Serialize};
use sp_renderer::Framebuffer;
use std::time::Duration;

/// Configuration passed to effects during initialization.
#[derive(Debug, Clone)]
pub struct EffectConfig {
    pub width: u32,
    pub height: u32,
    pub params: EffectParams,
}

/// Dynamic parameters for effects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EffectParams {
    /// Effect intensity (0.0 - 1.0)
    #[serde(default = "default_intensity")]
    pub intensity: f32,

    /// Speed multiplier
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Primary color (optional)
    pub color: Option<[u8; 3]>,

    /// Additional parameters as JSON
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

fn default_intensity() -> f32 {
    0.8
}

fn default_speed() -> f32 {
    1.0
}

/// Trait for visual effects.
///
/// Effects generate frames that are displayed on the LED panel.
/// They maintain internal state and update each tick.
pub trait Effect: Send + Sync {
    /// Get the unique name of this effect.
    fn name(&self) -> &'static str;

    /// Initialize the effect with configuration.
    fn init(&mut self, config: &EffectConfig);

    /// Generate the next frame.
    ///
    /// Returns `true` if the effect should continue, `false` if finished.
    fn tick(&mut self, fb: &mut Framebuffer, dt: Duration) -> bool;

    /// Clean up resources when effect stops.
    fn cleanup(&mut self) {}

    /// Check if effect supports hot parameter updates.
    fn supports_hot_update(&self) -> bool {
        false
    }

    /// Update parameters without restart (if supported).
    fn update_params(&mut self, _params: &EffectParams) {}
}
