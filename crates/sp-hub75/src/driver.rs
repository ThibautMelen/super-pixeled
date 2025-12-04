//! Real HUB75 driver implementation.

use sp_core::Result;
use sp_renderer::Framebuffer;

use crate::Driver;

/// Real HUB75 LED driver.
///
/// Uses GPIO to communicate with HUB75 panels.
/// Requires root privileges on Raspberry Pi.
pub struct LedDriver {
    gpio_slowdown: u8,
    initialized: bool,
    brightness: u8,
}

impl LedDriver {
    /// Create a new LED driver.
    pub fn new(gpio_slowdown: u8) -> Self {
        Self {
            gpio_slowdown,
            initialized: false,
            brightness: 80,
        }
    }
}

impl Driver for LedDriver {
    fn init(&mut self) -> Result<()> {
        tracing::info!(
            gpio_slowdown = self.gpio_slowdown,
            "Initializing HUB75 driver"
        );

        // TODO: Initialize rpi-rgb-led-matrix bindings
        // This would use FFI to the C++ library

        self.initialized = true;
        Ok(())
    }

    fn display(&mut self, fb: &Framebuffer) -> Result<()> {
        if !self.initialized {
            return Err(sp_core::Error::hardware("Driver not initialized"));
        }

        // TODO: Send framebuffer data to panel via GPIO
        // For now, just log that we would display
        tracing::trace!(
            width = fb.width(),
            height = fb.height(),
            "Displaying frame"
        );

        Ok(())
    }

    fn set_brightness(&mut self, brightness: u8) -> Result<()> {
        self.brightness = brightness.min(100);
        tracing::debug!(brightness = self.brightness, "Set brightness");
        Ok(())
    }

    fn is_healthy(&self) -> bool {
        self.initialized
    }

    fn shutdown(&mut self) -> Result<()> {
        tracing::info!("Shutting down HUB75 driver");
        self.initialized = false;
        Ok(())
    }
}
