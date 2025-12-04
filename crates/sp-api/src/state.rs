//! Application state shared across handlers.

use sp_core::Config;
use sp_effects::EffectManager;
use sp_hub75::Driver;
use sp_renderer::Framebuffer;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Shared application state.
pub struct AppState {
    pub config: Config,
    pub effect_manager: RwLock<EffectManager>,
    pub framebuffer: RwLock<Framebuffer>,
    pub driver: RwLock<Box<dyn Driver>>,
    pub start_time: Instant,
}

impl AppState {
    /// Create new application state.
    pub fn new(
        config: Config,
        effect_manager: EffectManager,
        driver: Box<dyn Driver>,
    ) -> Arc<Self> {
        let fb = Framebuffer::new(config.panel.width, config.panel.height);

        Arc::new(Self {
            config,
            effect_manager: RwLock::new(effect_manager),
            framebuffer: RwLock::new(fb),
            driver: RwLock::new(driver),
            start_time: Instant::now(),
        })
    }

    /// Get uptime in seconds.
    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
