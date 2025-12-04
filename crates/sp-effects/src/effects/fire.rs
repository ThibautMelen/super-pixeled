//! Fire effect - realistic flame animation.

use sp_core::{Color, Point};
use sp_renderer::Framebuffer;
use std::time::Duration;

use crate::{Effect, EffectConfig, EffectParams};

/// Fire effect with realistic flame animation.
///
/// Uses a heat map with cooling and propagation to simulate flames.
pub struct FireEffect {
    width: u32,
    height: u32,
    heat: Vec<u8>,
    intensity: f32,
    time: f32,
}

impl FireEffect {
    /// Create a new fire effect.
    pub fn new() -> Self {
        Self {
            width: 64,
            height: 32,
            heat: Vec::new(),
            intensity: 0.8,
            time: 0.0,
        }
    }

    /// Get heat value at position.
    fn get_heat(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.heat[(y as usize) * (self.width as usize) + (x as usize)]
        } else {
            0
        }
    }

    /// Set heat value at position.
    fn set_heat(&mut self, x: i32, y: i32, value: u8) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.heat[(y as usize) * (self.width as usize) + (x as usize)] = value;
        }
    }

    /// Convert heat to color (fire palette).
    fn heat_to_color(heat: u8) -> Color {
        // Fire palette: black -> red -> orange -> yellow -> white
        let t = heat as f32 / 255.0;

        if t < 0.25 {
            // Black to dark red
            let s = t * 4.0;
            Color::new((s * 128.0) as u8, 0, 0)
        } else if t < 0.5 {
            // Dark red to orange
            let s = (t - 0.25) * 4.0;
            Color::new(128 + (s * 127.0) as u8, (s * 128.0) as u8, 0)
        } else if t < 0.75 {
            // Orange to yellow
            let s = (t - 0.5) * 4.0;
            Color::new(255, 128 + (s * 127.0) as u8, 0)
        } else {
            // Yellow to white
            let s = (t - 0.75) * 4.0;
            Color::new(255, 255, (s * 255.0) as u8)
        }
    }

    /// Simple pseudo-random based on position and time.
    fn random(&self, seed: u32) -> u8 {
        let x = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((x >> 16) & 0xFF) as u8
    }
}

impl Default for FireEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl Effect for FireEffect {
    fn name(&self) -> &'static str {
        "fire"
    }

    fn init(&mut self, config: &EffectConfig) {
        self.width = config.width;
        self.height = config.height;
        self.intensity = config.params.intensity;
        self.heat = vec![0u8; (self.width * self.height) as usize];
        self.time = 0.0;

        tracing::debug!(
            width = self.width,
            height = self.height,
            intensity = self.intensity,
            "Fire effect initialized"
        );
    }

    fn tick(&mut self, fb: &mut Framebuffer, dt: Duration) -> bool {
        self.time += dt.as_secs_f32();
        let frame = (self.time * 30.0) as u32; // ~30 updates per second

        // Step 1: Seed the bottom row with random heat
        for x in 0..self.width as i32 {
            let seed = (frame.wrapping_mul(1000) + x as u32) as u32;
            let base_heat = (self.intensity * 255.0) as u8;
            let variation = (self.random(seed) as i32 - 128) / 4;
            let heat = (base_heat as i32 + variation).clamp(0, 255) as u8;
            self.set_heat(x, self.height as i32 - 1, heat);
        }

        // Step 2: Propagate heat upward with cooling
        for y in 0..(self.height as i32 - 1) {
            for x in 0..self.width as i32 {
                // Average of pixels below
                let below = self.get_heat(x, y + 1) as i32;
                let below_left = self.get_heat(x - 1, y + 1) as i32;
                let below_right = self.get_heat(x + 1, y + 1) as i32;
                let below_two = self.get_heat(x, y + 2) as i32;

                let avg = (below + below_left + below_right + below_two) / 4;

                // Cooling factor (more at top)
                let cooling = (self.height as i32 - y) * 2;
                let new_heat = (avg - cooling).max(0) as u8;

                self.set_heat(x, y, new_heat);
            }
        }

        // Step 3: Render heat map to framebuffer
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let heat = self.get_heat(x, y);
                let color = Self::heat_to_color(heat);
                fb.set(Point::new(x, y), color);
            }
        }

        true // Continue running
    }

    fn cleanup(&mut self) {
        self.heat.clear();
        tracing::debug!("Fire effect cleaned up");
    }

    fn supports_hot_update(&self) -> bool {
        true
    }

    fn update_params(&mut self, params: &EffectParams) {
        self.intensity = params.intensity;
        tracing::debug!(intensity = self.intensity, "Fire effect params updated");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_effect() {
        let mut effect = FireEffect::new();
        let config = EffectConfig {
            width: 64,
            height: 32,
            params: EffectParams::default(),
        };

        effect.init(&config);

        let mut fb = Framebuffer::new(64, 32);
        let continuing = effect.tick(&mut fb, Duration::from_millis(16));

        assert!(continuing);
        // Bottom row should have some heat (non-black pixels)
        let bottom_pixel = fb.get(Point::new(32, 31)).unwrap();
        // Fire should have some red in it
        assert!(bottom_pixel.r > 0 || bottom_pixel.g > 0);
    }

    #[test]
    fn test_heat_to_color() {
        // Black at 0
        let black = FireEffect::heat_to_color(0);
        assert_eq!(black, Color::BLACK);

        // Should have some red at mid values
        let mid = FireEffect::heat_to_color(128);
        assert!(mid.r > 0);

        // White-ish at max
        let hot = FireEffect::heat_to_color(255);
        assert_eq!(hot.r, 255);
        assert_eq!(hot.g, 255);
    }
}
