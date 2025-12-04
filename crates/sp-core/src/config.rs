//! Application configuration with TOML support.

use config::{Config as ConfigBuilder, Environment, File};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::Result;

/// Main application configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Config {
    #[validate(nested)]
    pub server: ServerConfig,
    #[validate(nested)]
    pub panel: PanelConfig,
    pub hardware: HardwareConfig,
    #[validate(nested)]
    pub effects: EffectsConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ServerConfig {
    pub host: String,
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PanelConfig {
    #[validate(range(min = 1, max = 256))]
    pub width: u32,
    #[validate(range(min = 1, max = 256))]
    pub height: u32,
    #[validate(range(min = 0, max = 100))]
    pub brightness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub mock: bool,
    pub gpio_slowdown: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EffectsConfig {
    pub default: String,
    #[validate(range(min = 0, max = 5000))]
    pub transition_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Config {
    /// Load configuration from files and environment.
    ///
    /// Priority (highest to lowest):
    /// 1. Environment variables (SP_*)
    /// 2. config/local.toml (optional, gitignored)
    /// 3. config/default.toml
    pub fn load() -> Result<Self> {
        let config = ConfigBuilder::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("SP").separator("__"))
            .build()?;

        let config: Config = config.try_deserialize()?;

        // Validate configuration
        config
            .validate()
            .map_err(|e| crate::Error::Config(config::ConfigError::Message(e.to_string())))?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            panel: PanelConfig {
                width: 64,
                height: 32,
                brightness: 80,
            },
            hardware: HardwareConfig {
                mock: false,
                gpio_slowdown: 2,
            },
            effects: EffectsConfig {
                default: "fire".to_string(),
                transition_ms: 500,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.panel.width, 64);
        assert_eq!(config.panel.height, 32);
        assert_eq!(config.server.port, 3000);
    }

    #[test]
    fn test_validation() {
        let mut config = Config::default();
        config.panel.width = 0; // Invalid

        let result = config.validate();
        assert!(result.is_err());
    }
}
