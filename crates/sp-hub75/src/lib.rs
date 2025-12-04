//! HUB75 LED driver abstraction.
//!
//! Provides a trait-based abstraction for LED panel drivers,
//! allowing easy mocking for tests and development.

mod driver;
mod mock;

pub use driver::LedDriver;
pub use mock::MockDriver;

use sp_core::Result;
use sp_renderer::Framebuffer;

/// Trait for LED panel drivers.
///
/// Implementors handle the low-level communication with HUB75 panels.
/// Use `MockDriver` for development without hardware.
#[cfg_attr(test, mockall::automock)]
pub trait Driver: Send + Sync {
    /// Initialize the driver.
    fn init(&mut self) -> Result<()>;

    /// Display a framebuffer on the panel.
    fn display(&mut self, fb: &Framebuffer) -> Result<()>;

    /// Set panel brightness (0-100).
    fn set_brightness(&mut self, brightness: u8) -> Result<()>;

    /// Check if driver is healthy.
    fn is_healthy(&self) -> bool;

    /// Shutdown the driver gracefully.
    fn shutdown(&mut self) -> Result<()>;
}

/// Create a driver based on configuration.
pub fn create_driver(mock: bool, gpio_slowdown: u8) -> Box<dyn Driver> {
    if mock {
        tracing::info!("Using mock LED driver");
        Box::new(MockDriver::new())
    } else {
        tracing::info!(gpio_slowdown, "Using real LED driver");
        Box::new(LedDriver::new(gpio_slowdown))
    }
}
