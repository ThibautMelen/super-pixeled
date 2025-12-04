//! HTTP API for Super Pixeled.
//!
//! Provides REST endpoints for controlling the LED panel.

mod handlers;
mod routes;
mod state;
mod validation;

pub use routes::create_router;
pub use state::AppState;
