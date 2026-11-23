use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use show360_core::{
    CreateRoomRequest, JoinRoomRequest, ParticipantId, Role, Room, RoomCode, RoomId, Session,
};
use show360_media::IssuedToken;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

#[derive(Serialize)]
pub struct CreateRoomResponse {
    pub room: RoomSummary,
    pub join_url: String,
}

#[derive(Serialize, Clone)]
pub struct RoomSummary {
    pub id: String,
    pub code: String,
    pub name: String,
    pub mode: show360_core::RoomMode,
    pub max_participants: u32,
    pub participant_count: usize,
    pub presenter_id: Option<String>,
}

impl From<&Room> for RoomSummary {
    fn from(room: &Room) -> Self {
        Self {
            id: room.id.to_string(),
            code: room.code.to_string(),
            name: room.name.clone(),
            mode: room.mode,
            max_participants: room.max_participants,
            participant_count: room.participants.len(),
            presenter_id: room.presenter_id.as_ref().map(|p| p.to_string()),
        }
    }
}

#[derive(Serialize)]
pub struct JoinRoomResponse {
    pub room: RoomSummary,
    pub session: Session,
    pub participant_id: String,
    pub role: Role,
    /// Token LiveKit stub/réel — secrets SFU jamais inclus.
    pub media: IssuedToken,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub participant_id: String,
    pub display_name: String,
    #[serde(default = "default_role")]
    pub role: Role,
}

fn default_role() -> Role {
    Role::Participant
}

#[derive(Deserialize)]
pub struct SetPresenterRequest {
    pub participant_id: String,
}

#[derive(Serialize)]
pub struct SetPresenterResponse {
    pub room: RoomSummary,
}

pub async fn create_room(
    State(state): State<AppState>,
    Json(req): Json<CreateRoomRequest>,
) -> ApiResult<Json<CreateRoomResponse>> {
    if req.name.trim().is_empty() {
        return Err(ApiError::BadRequest("name requis".into()));
    }
    let room = Room::create(req);
    let summary = RoomSummary::from(&room);
    let join_url = format!(
        "{}/join?code={}",
        state.inner.config.public_base_url.trim_end_matches('/'),
        summary.code
    );
    state.insert_room(room).await;
    Ok(Json(CreateRoomResponse {
        room: summary,
        join_url,
    }))
}

pub async fn join_room(
    State(state): State<AppState>,
    Json(req): Json<JoinRoomRequest>,
) -> ApiResult<Json<JoinRoomResponse>> {
    if req.display_name.trim().is_empty() {
        return Err(ApiError::BadRequest("display_name requis".into()));
    }

    let room_id = resolve_room_id(&state, &req).await?;

    let (participant, room_snapshot) = state
        .update_room(&room_id, |room| {
            let p = room.join(&req.display_name, req.role)?.clone();
            Ok::<_, show360_core::CoreError>((p, room.clone()))
        })
        .await
        .ok_or_else(|| ApiError::NotFound(format!("salle introuvable: {room_id}")))??;

    let session = Session::start(room_id.clone(), participant.id.clone());
    let media = state.inner.issuer.issue(
        &room_id,
        &participant.id,
        participant.display_name.0.as_str(),
        participant.role,
    )?;

    Ok(Json(JoinRoomResponse {
        room: RoomSummary::from(&room_snapshot),
        session,
        participant_id: participant.id.to_string(),
        role: participant.role,
        media,
    }))
}

pub async fn get_room(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> ApiResult<Json<RoomSummary>> {
    let id = RoomId(room_id);
    let room = state
        .get_room_by_id(&id)
        .await
        .ok_or_else(|| ApiError::from(show360_core::CoreError::RoomNotFound(id.to_string())))?;
    Ok(Json(RoomSummary::from(&room)))
}

pub async fn issue_token(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<TokenRequest>,
) -> ApiResult<Json<IssuedToken>> {
    let id = RoomId(room_id);
    let room = state
        .get_room_by_id(&id)
        .await
        .ok_or_else(|| ApiError::from(show360_core::CoreError::RoomNotFound(id.to_string())))?;

    let participant_id = ParticipantId(req.participant_id);
    if !room.participants.iter().any(|p| p.id == participant_id) {
        return Err(ApiError::BadRequest(
            "participant inconnu dans cette salle".into(),
        ));
    }

    let issued = state
        .inner
        .issuer
        .issue(&id, &participant_id, &req.display_name, req.role)?;
    Ok(Json(issued))
}

pub async fn set_presenter(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<SetPresenterRequest>,
) -> ApiResult<Json<SetPresenterResponse>> {
    let id = RoomId(room_id);
    let participant_id = ParticipantId(req.participant_id);

    let room = state
        .update_room(&id, |room| {
            room.set_presenter(&participant_id)?;
            Ok::<_, show360_core::CoreError>(room.clone())
        })
        .await
        .ok_or_else(|| ApiError::from(show360_core::CoreError::RoomNotFound(id.to_string())))??;

    Ok(Json(SetPresenterResponse {
        room: RoomSummary::from(&room),
    }))
}

async fn resolve_room_id(state: &AppState, req: &JoinRoomRequest) -> ApiResult<RoomId> {
    if let Some(id) = &req.room_id {
        if state.get_room_by_id(id).await.is_some() {
            return Ok(id.clone());
        }
        return Err(ApiError::from(show360_core::CoreError::RoomNotFound(
            id.to_string(),
        )));
    }
    if let Some(code) = &req.room_code {
        if let Some(room) = state.get_room_by_code(code).await {
            return Ok(room.id);
        }
        return Err(ApiError::BadRequest(format!(
            "code salle inconnu: {}",
            code.as_str()
        )));
    }
    let _ = RoomCode::generate(); // keep type linked for future validation helpers
    Err(ApiError::BadRequest("room_id ou room_code requis".into()))
}
