use axum::Json;
use serde::Serialize;
use show360_media::LiveKitDeployment;

use crate::error::ApiResult;
use crate::state::AppState;
use axum::extract::State;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub livekit_mode: &'static str,
    pub livekit_url: String,
}

pub async fn health(State(state): State<AppState>) -> ApiResult<Json<HealthResponse>> {
    let cfg = state.inner.issuer.config();
    let mode = match cfg.deployment {
        LiveKitDeployment::SelfHosted => "self_hosted",
        LiveKitDeployment::Cloud => "cloud",
    };
    Ok(Json(HealthResponse {
        status: "ok",
        service: "show360-api",
        livekit_mode: mode,
        // URL publique uniquement — jamais api_key / api_secret.
        livekit_url: cfg.url.clone(),
    }))
}
