//! UI Dioxus partagée — placeholders P0 (salle, présentateur, shell).
//!
//! Les clients `apps/web` et `apps/desktop` consomment ces composants.
//! WebRTC / LiveKit SDK : lots suivants (hors scaffold).

use dioxus::prelude::*;
use show360_core::{Role, RoomMode};

mod presenter_badge;
mod room_panel;
mod shell;

pub use presenter_badge::PresenterBadge;
pub use room_panel::RoomPanel;
pub use shell::AppShell;

/// État UI minimal partagé (placeholder — pas encore branché LiveKit).
#[derive(Debug, Clone, PartialEq)]
pub struct UiSession {
    pub room_name: String,
    pub room_code: String,
    pub display_name: String,
    pub role: Role,
    pub mode: RoomMode,
    pub presenter_name: Option<String>,
}

impl Default for UiSession {
    fn default() -> Self {
        Self {
            room_name: "Show360".into(),
            room_code: "------".into(),
            display_name: "Invité".into(),
            role: Role::Participant,
            mode: RoomMode::SelfHosted,
            presenter_name: None,
        }
    }
}

/// Racine placeholder : shell + panneau salle.
#[component]
pub fn Show360App(session: UiSession) -> Element {
    rsx! {
        AppShell {
            title: "Show360".to_string(),
            RoomPanel { session }
        }
    }
}
