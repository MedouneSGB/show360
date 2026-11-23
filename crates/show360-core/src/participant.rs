use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::role::Role;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParticipantId(pub String);

impl ParticipantId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for ParticipantId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for ParticipantId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ParticipantId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for ParticipantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DisplayName(pub String);

impl DisplayName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl std::fmt::Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Participant {
    pub id: ParticipantId,
    pub display_name: DisplayName,
    pub role: Role,
    pub muted: bool,
    pub camera_enabled: bool,
}

impl Participant {
    pub fn new(display_name: impl Into<String>, role: Role) -> Self {
        Self {
            id: ParticipantId::new(),
            display_name: DisplayName::new(display_name),
            role,
            muted: false,
            camera_enabled: role.can_publish_av(),
        }
    }
}
