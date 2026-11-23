use serde::{Deserialize, Serialize};

use crate::participant::ParticipantId;

/// Nature d'une track média (LiveKit + logique métier).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    Camera,
    Microphone,
    Screen,
    /// Flux 360° (équirectangulaire) — rendu client en P2.
    Projection360,
    Data,
}

/// Métadonnées de projection (écran classique vs 360°).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionKind {
    /// Partage d'écran 2D classique (P0).
    Screen,
    /// Projection 360° (P2).
    Equirectangular360,
}

/// Publication de track côté métier (miroir léger des tracks LiveKit).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackPublication {
    pub id: String,
    pub kind: TrackKind,
    pub publisher: ParticipantId,
    pub projection: Option<ProjectionKind>,
    /// Spotlight présentateur : la track à mettre en avant côté clients.
    pub spotlight: bool,
}

impl TrackPublication {
    pub fn screen(publisher: ParticipantId, id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: TrackKind::Screen,
            publisher,
            projection: Some(ProjectionKind::Screen),
            spotlight: true,
        }
    }

    pub fn projection_360(publisher: ParticipantId, id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: TrackKind::Projection360,
            publisher,
            projection: Some(ProjectionKind::Equirectangular360),
            spotlight: true,
        }
    }
}
