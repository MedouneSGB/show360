//! Types métier partagés Show360 — sans dépendance UI.
//!
//! Prépare A/V, screen share, mode présentateur et trajectoire 360° (P2).

mod error;
mod participant;
mod role;
mod room;
mod session;
mod track;

pub use error::{CoreError, CoreResult};
pub use participant::{DisplayName, Participant, ParticipantId};
pub use role::Role;
pub use room::{CreateRoomRequest, JoinRoomRequest, Room, RoomCode, RoomId, RoomMode};
pub use session::{Session, SessionId, SessionState};
pub use track::{ProjectionKind, TrackKind, TrackPublication};

/// Capacité cible P0 (self-host mono-nœud). Voir plan produit.
pub const P0_MAX_PARTICIPANTS: u32 = 25;
