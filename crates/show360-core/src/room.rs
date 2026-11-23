use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{CoreError, CoreResult};
use crate::participant::{Participant, ParticipantId};
use crate::role::Role;
use crate::track::TrackPublication;
use crate::P0_MAX_PARTICIPANTS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoomId(pub String);

impl RoomId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for RoomId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Code court pour rejoindre une salle (lien + code).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoomCode(pub String);

impl RoomCode {
    pub fn generate() -> Self {
        // 6 caractères alphanumériques — stub déterministe via UUID tronqué.
        let raw = Uuid::new_v4().simple().to_string();
        Self(raw[..6].to_uppercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RoomCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Mode opérationnel de la salle (self-host vs cloud LiveKit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RoomMode {
    #[default]
    SelfHosted,
    Cloud,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    #[serde(default)]
    pub mode: RoomMode,
    /// Capacité max ; défaut P0 = 25.
    pub max_participants: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRoomRequest {
    pub display_name: String,
    #[serde(default = "default_join_role")]
    pub role: Role,
    /// Rejoindre via id ou code (au moins un).
    pub room_id: Option<RoomId>,
    pub room_code: Option<RoomCode>,
}

fn default_join_role() -> Role {
    Role::Participant
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub code: RoomCode,
    pub name: String,
    pub mode: RoomMode,
    pub max_participants: u32,
    pub participants: Vec<Participant>,
    pub presenter_id: Option<ParticipantId>,
    pub active_tracks: Vec<TrackPublication>,
    pub created_at: DateTime<Utc>,
}

impl Room {
    pub fn create(req: CreateRoomRequest) -> Self {
        let max = req.max_participants.unwrap_or(P0_MAX_PARTICIPANTS);
        Self {
            id: RoomId::new(),
            code: RoomCode::generate(),
            name: req.name,
            mode: req.mode,
            max_participants: max,
            participants: Vec::new(),
            presenter_id: None,
            active_tracks: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn join(
        &mut self,
        display_name: impl Into<String>,
        role: Role,
    ) -> CoreResult<&Participant> {
        if self.participants.len() as u32 >= self.max_participants {
            return Err(CoreError::RoomFull {
                max: self.max_participants,
            });
        }

        let mut role = role;
        // Premier entrant devient host + présentateur par défaut (P0).
        if self.participants.is_empty() {
            role = Role::Host;
        }

        if role == Role::Presenter {
            if let Some(current) = &self.presenter_id {
                return Err(CoreError::PresenterAlreadyAssigned {
                    current: current.to_string(),
                });
            }
        }

        let participant = Participant::new(display_name, role);
        if matches!(participant.role, Role::Host | Role::Presenter) && self.presenter_id.is_none() {
            self.presenter_id = Some(participant.id.clone());
            // Host assume aussi le rôle présentateur côté permissions tracks.
        }

        self.participants.push(participant);
        Ok(self.participants.last().expect("just pushed"))
    }

    /// Transfert du mode présentateur (P0 : un seul à la fois).
    pub fn set_presenter(&mut self, participant_id: &ParticipantId) -> CoreResult<()> {
        let target_idx = self
            .participants
            .iter()
            .position(|p| &p.id == participant_id)
            .ok_or_else(|| {
                CoreError::Other(format!("participant introuvable: {participant_id}"))
            })?;

        if !self.participants[target_idx].role.can_become_presenter() {
            return Err(CoreError::InvalidRole(
                self.participants[target_idx].role.to_string(),
            ));
        }

        // Rétrograde l'ancien présentateur (sauf host qui reste host).
        if let Some(old_id) = self.presenter_id.clone() {
            if let Some(old) = self.participants.iter_mut().find(|p| p.id == old_id) {
                if old.role == Role::Presenter {
                    old.role = Role::Participant;
                }
            }
        }

        let p = &mut self.participants[target_idx];
        if p.role != Role::Host {
            p.role = Role::Presenter;
        }
        self.presenter_id = Some(participant_id.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_joiner_is_host_and_presenter() {
        let mut room = Room::create(CreateRoomRequest {
            name: "demo".into(),
            mode: RoomMode::SelfHosted,
            max_participants: Some(10),
        });
        let p = room.join("Alice", Role::Participant).unwrap().clone();
        assert_eq!(p.role, Role::Host);
        assert_eq!(room.presenter_id.as_ref(), Some(&p.id));
    }

    #[test]
    fn transfer_presenter() {
        let mut room = Room::create(CreateRoomRequest {
            name: "demo".into(),
            mode: RoomMode::SelfHosted,
            max_participants: None,
        });
        let host = room.join("Alice", Role::Participant).unwrap().id.clone();
        let bob = room.join("Bob", Role::Participant).unwrap().id.clone();
        room.set_presenter(&bob).unwrap();
        assert_eq!(room.presenter_id.as_ref(), Some(&bob));
        assert_ne!(room.presenter_id.as_ref(), Some(&host));
    }
}
