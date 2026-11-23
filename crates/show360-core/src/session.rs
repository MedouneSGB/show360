use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::participant::ParticipantId;
use crate::room::RoomId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Lobby,
    Live,
    Ended,
}

/// Session applicative liée à une salle (auth/JWT app vs token LiveKit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub room_id: RoomId,
    pub participant_id: ParticipantId,
    pub state: SessionState,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn start(room_id: RoomId, participant_id: ParticipantId) -> Self {
        Self {
            id: SessionId::new(),
            room_id,
            participant_id,
            state: SessionState::Live,
            created_at: Utc::now(),
        }
    }
}
