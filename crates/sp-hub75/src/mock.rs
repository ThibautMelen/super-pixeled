//! Mock driver for development without hardware.

use sp_core::Result;
use sp_renderer::Framebuffer;

use crate::Driver;

/// Mock LED driver for development.
///
/// Logs operations without requiring hardware.
/// Useful for testing and development on non-Pi machines.
pub struct MockDriver {
    initialized: bool,
    brightness: u8,
    frame_count: u64,
}

impl MockDriver {
    /// Create a new mock driver.
    pub fn new() -> Self {
        Self {
            initialized: false,
            brightness: 80,
            frame_count: 0,
        }
    }

    /// Get the number of frames displayed.
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}

impl Default for MockDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for MockDriver {
    fn init(&mut self) -> Result<()> {
        tracing::info!("Mock driver initialized");
        self.initialized = true;
        Ok(())
    }

    fn display(&mut self, fb: &Framebuffer) -> Result<()> {
        if !self.initialized {
            return Err(sp_core::Error::hardware("Mock driver not initialized"));
        }

        self.frame_count += 1;

        // Log every 60 frames (1 second at 60 FPS)
        if self.frame_count % 60 == 0 {
            tracing::debug!(
                frame = self.frame_count,
                width = fb.width(),
                height = fb.height(),
                "Mock: displayed frame"
            );
        }

        Ok(())
    }

    fn set_brightness(&mut self, brightness: u8) -> Result<()> {
        self.brightness = brightness.min(100);
        tracing::debug!(brightness = self.brightness, "Mock: set brightness");
        Ok(())
    }

    fn is_healthy(&self) -> bool {
        self.initialized
    }

    fn shutdown(&mut self) -> Result<()> {
        tracing::info!(frames = self.frame_count, "Mock driver shutdown");
        self.initialized = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_driver() {
        let mut driver = MockDriver::new();

        assert!(!driver.is_healthy());
        driver.init().unwrap();
        assert!(driver.is_healthy());

        let fb = Framebuffer::default();
        driver.display(&fb).unwrap();
        assert_eq!(driver.frame_count(), 1);

        driver.shutdown().unwrap();
        assert!(!driver.is_healthy());
    }
}
