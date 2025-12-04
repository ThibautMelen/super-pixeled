//! HTTP request handlers.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sp_effects::{available_effects, EffectParams};
use std::sync::Arc;
use validator::Validate;

use crate::{state::AppState, validation::ApiError};

// ============================================================================
// Health Check
// ============================================================================

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub uptime_secs: u64,
    pub panel: PanelInfo,
}

#[derive(Serialize)]
pub struct PanelInfo {
    pub width: u32,
    pub height: u32,
    pub brightness: u8,
}

pub async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        uptime_secs: state.uptime_secs(),
        panel: PanelInfo {
            width: state.config.panel.width,
            height: state.config.panel.height,
            brightness: state.config.panel.brightness,
        },
    })
}

// ============================================================================
// Effects
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct EffectRequest {
    pub name: String,
    #[serde(default)]
    pub params: EffectParams,
}

#[derive(Serialize)]
pub struct EffectResponse {
    pub success: bool,
    pub effect: String,
}

pub async fn set_effect(
    State(state): State<Arc<AppState>>,
    Json(req): Json<EffectRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut manager = state.effect_manager.write().await;

    manager.set_effect(&req.name, req.params)?;

    Ok((
        StatusCode::OK,
        Json(EffectResponse {
            success: true,
            effect: req.name,
        }),
    ))
}

pub async fn get_current_effect(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let manager = state.effect_manager.read().await;
    let current = manager.current_effect().map(String::from);

    Json(serde_json::json!({
        "current": current,
        "available": available_effects(),
    }))
}

pub async fn stop_effect(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let mut manager = state.effect_manager.write().await;
    manager.stop();

    Json(serde_json::json!({
        "success": true,
        "message": "Effect stopped"
    }))
}

// ============================================================================
// Text Display
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct TextRequest {
    #[validate(length(min = 1, max = 256))]
    pub text: String,
    #[serde(default = "default_color")]
    pub color: [u8; 3],
    #[serde(default)]
    pub scroll: bool,
    #[serde(default = "default_speed")]
    #[validate(range(min = 1, max = 200))]
    pub speed: u32,
}

fn default_color() -> [u8; 3] {
    [255, 255, 255]
}

fn default_speed() -> u32 {
    50
}

pub async fn display_text(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<TextRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement text rendering with embedded font
    tracing::info!(
        text = %req.text,
        color = ?req.color,
        scroll = req.scroll,
        "Display text requested"
    );

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "text": req.text
        })),
    ))
}

// ============================================================================
// Brightness
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct BrightnessRequest {
    #[validate(range(min = 0, max = 100))]
    pub brightness: u8,
}

pub async fn set_brightness(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BrightnessRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut driver = state.driver.write().await;
    driver
        .set_brightness(req.brightness)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "brightness": req.brightness
        })),
    ))
}
