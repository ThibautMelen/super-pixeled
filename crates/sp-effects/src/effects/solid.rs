//! Solid color effect.

use sp_core::Color;
use sp_renderer::Framebuffer;
use std::time::Duration;

use crate::{Effect, EffectConfig, EffectParams};

/// Solid color effect - fills panel with a single color.
pub struct SolidEffect {
    color: Color,
}

impl SolidEffect {
    /// Create a new solid effect with default color (white).
    pub fn new() -> Self {
        Self {
            color: Color::WHITE,
        }
    }

    /// Create an "off" effect (black/no light).
    pub fn off() -> Self {
        Self {
            color: Color::BLACK,
        }
    }

    /// Create with a specific color.
    pub fn with_color(color: Color) -> Self {
        Self { color }
    }
}

impl Default for SolidEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl Effect for SolidEffect {
    fn name(&self) -> &'static str {
        "solid"
    }

    fn init(&mut self, config: &EffectConfig) {
        if let Some(rgb) = config.params.color {
            self.color = Color::from(rgb);
        }

        tracing::debug!(
            r = self.color.r,
            g = self.color.g,
            b = self.color.b,
            "Solid effect initialized"
        );
    }

    fn tick(&mut self, fb: &mut Framebuffer, _dt: Duration) -> bool {
        fb.fill(self.color);
        true
    }

    fn supports_hot_update(&self) -> bool {
        true
    }

    fn update_params(&mut self, params: &EffectParams) {
        if let Some(rgb) = params.color {
            self.color = Color::from(rgb);
        }
    }
}
