//! Super Pixeled - LED Panel Controller
//!
//! A high-performance LED panel controller built with Rust.
//!
//! # Usage
//!
//! ```bash
//! # Run with mock hardware (for development)
//! super-pixeled --mock
//!
//! # Run with real hardware (requires GPIO access)
//! sudo super-pixeled
//! ```

use anyhow::{Context, Result};
use sp_api::{create_router, AppState};
use sp_core::Config;
use sp_effects::EffectManager;
use sp_hub75::create_driver;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::watch;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line args
    let args: Vec<String> = std::env::args().collect();
    let mock_mode = args.iter().any(|a| a == "--mock");

    // Load configuration
    let mut config = Config::default();
    if mock_mode {
        config.hardware.mock = true;
    }

    // Initialize logging
    init_logging(&config);

    info!(
        version = env!("CARGO_PKG_VERSION"),
        mock = config.hardware.mock,
        "Starting Super Pixeled"
    );

    // Create LED driver
    let mut driver = create_driver(config.hardware.mock, config.hardware.gpio_slowdown);
    driver.init().context("Failed to initialize LED driver")?;

    // Create effect manager
    let mut effect_manager = EffectManager::new(config.panel.width, config.panel.height);

    // Start default effect
    if let Err(e) = effect_manager.set_effect(
        &config.effects.default,
        sp_effects::EffectParams::default(),
    ) {
        error!(error = %e, "Failed to start default effect");
    }

    // Create application state
    let state = AppState::new(config.clone(), effect_manager, driver);

    // Create shutdown signal
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // Start render loop in background
    let render_state = Arc::clone(&state);
    let render_shutdown = shutdown_rx.clone();
    let render_handle = tokio::spawn(async move {
        render_loop(render_state, render_shutdown).await;
    });

    // Create HTTP router
    let app = create_router(state);

    // Bind to address
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .context("Invalid server address")?;

    info!(%addr, "Starting HTTP server");

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind to address")?;

    // Serve with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_tx))
        .await
        .context("Server error")?;

    // Wait for render loop to finish
    render_handle.await?;

    info!("Super Pixeled shutdown complete");
    Ok(())
}

/// Initialize the logging system.
fn init_logging(config: &Config) {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.logging.level));

    let subscriber = tracing_subscriber::registry().with(filter);

    if config.logging.format == "json" {
        subscriber.with(fmt::layer().json()).init();
    } else {
        subscriber.with(fmt::layer().pretty()).init();
    }
}

/// Handle shutdown signals (Ctrl+C, SIGTERM).
async fn shutdown_signal(shutdown_tx: watch::Sender<bool>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C"),
        _ = terminate => info!("Received SIGTERM"),
    }

    info!("Initiating graceful shutdown...");
    let _ = shutdown_tx.send(true);
}

/// Main render loop - runs at ~60 FPS.
async fn render_loop(state: Arc<AppState>, mut shutdown_rx: watch::Receiver<bool>) {
    let frame_duration = Duration::from_millis(16); // ~60 FPS

    loop {
        // Check for shutdown
        if *shutdown_rx.borrow() {
            break;
        }

        // Generate frame
        {
            let mut manager = state.effect_manager.write().await;
            let mut fb = state.framebuffer.write().await;
            manager.tick(&mut fb);
        }

        // Display frame
        {
            let fb = state.framebuffer.read().await;
            let mut driver = state.driver.write().await;
            if let Err(e) = driver.display(&fb) {
                error!(error = %e, "Failed to display frame");
            }
        }

        // Wait for next frame
        tokio::select! {
            _ = tokio::time::sleep(frame_duration) => {}
            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    break;
                }
            }
        }
    }

    // Cleanup
    info!("Render loop stopped");
    let mut driver = state.driver.write().await;
    if let Err(e) = driver.shutdown() {
        error!(error = %e, "Error during driver shutdown");
    }
}
