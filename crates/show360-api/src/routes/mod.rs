mod health;
mod rooms;

use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/v1/rooms", post(rooms::create_room))
        .route("/v1/rooms/join", post(rooms::join_room))
        .route("/v1/rooms/{room_id}/token", post(rooms::issue_token))
        .route("/v1/rooms/{room_id}/presenter", post(rooms::set_presenter))
        .route("/v1/rooms/{room_id}", get(rooms::get_room))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
