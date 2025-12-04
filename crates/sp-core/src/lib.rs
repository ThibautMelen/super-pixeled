//! Core types and configuration for Super Pixeled.
//!
//! This crate provides fundamental types used throughout the project:
//! - `Color`: RGB color representation
//! - `Point`: 2D coordinate
//! - `Config`: Application configuration
//! - `Error`: Unified error type

mod color;
mod config;
mod error;
mod point;

pub use color::Color;
pub use config::{Config, EffectsConfig, HardwareConfig, LoggingConfig, PanelConfig, ServerConfig};
pub use error::{Error, Result};
pub use point::Point;
