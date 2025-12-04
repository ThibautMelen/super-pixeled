//! API route definitions.

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{handlers, state::AppState};

/// Create the API router with all routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Health check
        .route("/health", get(handlers::health))
        // Effects
        .route("/api/effect", post(handlers::set_effect))
        .route("/api/effect", get(handlers::get_current_effect))
        .route("/api/effect/stop", post(handlers::stop_effect))
        // Text
        .route("/api/text", post(handlers::display_text))
        // Brightness
        .route("/api/brightness", post(handlers::set_brightness))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
