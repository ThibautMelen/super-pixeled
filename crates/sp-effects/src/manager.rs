//! Effect manager - handles effect lifecycle and switching.

use sp_core::{Error, Result};
use sp_renderer::Framebuffer;
use std::time::{Duration, Instant};

use crate::{create_effect, Effect, EffectConfig, EffectParams};

/// Manages effect lifecycle and transitions.
pub struct EffectManager {
    current: Option<Box<dyn Effect>>,
    config: EffectConfig,
    last_tick: Instant,
}

impl EffectManager {
    /// Create a new effect manager.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            current: None,
            config: EffectConfig {
                width,
                height,
                params: EffectParams::default(),
            },
            last_tick: Instant::now(),
        }
    }

    /// Get the name of the current effect.
    pub fn current_effect(&self) -> Option<&str> {
        self.current.as_ref().map(|e| e.name())
    }

    /// Switch to a new effect.
    pub fn set_effect(&mut self, name: &str, params: EffectParams) -> Result<()> {
        // Cleanup current effect
        if let Some(ref mut effect) = self.current {
            effect.cleanup();
        }

        // Create new effect
        let mut effect = create_effect(name).ok_or_else(|| Error::EffectNotFound(name.to_string()))?;

        // Initialize with config
        self.config.params = params;
        effect.init(&self.config);

        self.current = Some(effect);
        self.last_tick = Instant::now();

        tracing::info!(effect = name, "Switched to effect");
        Ok(())
    }

    /// Update effect parameters without restart.
    pub fn update_params(&mut self, params: EffectParams) -> Result<()> {
        if let Some(ref mut effect) = self.current {
            if effect.supports_hot_update() {
                effect.update_params(&params);
                self.config.params = params;
                Ok(())
            } else {
                // Restart effect with new params
                let name = effect.name().to_string();
                self.set_effect(&name, params)
            }
        } else {
            Err(Error::Internal("No active effect".to_string()))
        }
    }

    /// Generate the next frame.
    pub fn tick(&mut self, fb: &mut Framebuffer) -> bool {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick);
        self.last_tick = now;

        if let Some(ref mut effect) = self.current {
            effect.tick(fb, dt)
        } else {
            fb.clear();
            true
        }
    }

    /// Stop the current effect.
    pub fn stop(&mut self) {
        if let Some(ref mut effect) = self.current {
            effect.cleanup();
        }
        self.current = None;
        tracing::info!("Effect stopped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager() {
        let mut manager = EffectManager::new(64, 32);

        assert!(manager.current_effect().is_none());

        manager
            .set_effect("fire", EffectParams::default())
            .unwrap();
        assert_eq!(manager.current_effect(), Some("fire"));

        let mut fb = Framebuffer::new(64, 32);
        let continuing = manager.tick(&mut fb);
        assert!(continuing);
    }

    #[test]
    fn test_invalid_effect() {
        let mut manager = EffectManager::new(64, 32);
        let result = manager.set_effect("nonexistent", EffectParams::default());
        assert!(result.is_err());
    }
}
