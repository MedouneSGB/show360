use dioxus::prelude::*;
use show360_core::RoomMode;

use crate::presenter_badge::PresenterBadge;
use crate::UiSession;

#[component]
pub fn RoomPanel(session: UiSession) -> Element {
    let mode_label = match session.mode {
        RoomMode::SelfHosted => "LiveKit self-hosted",
        RoomMode::Cloud => "LiveKit Cloud",
    };

    rsx! {
        section {
            style: "max-width: 42rem;",
            h2 {
                style: "margin: 0 0 0.5rem; font-size: 1.35rem;",
                "{session.room_name}"
            }
            p {
                style: "margin: 0 0 1rem; opacity: 0.85;",
                "Code salle : "
                strong { "{session.room_code}" }
                " · Mode : {mode_label}"
            }
            PresenterBadge {
                role: session.role,
                presenter_name: session.presenter_name.clone(),
            }
            div {
                style: "margin-top: 1.5rem; padding: 1.25rem; border: 1px dashed rgba(248,250,252,0.35); border-radius: 0.5rem;",
                p {
                    style: "margin: 0 0 0.5rem;",
                    "Connecté en tant que "
                    strong { "{session.display_name}" }
                    " ({session.role})"
                }
                p {
                    style: "margin: 0; opacity: 0.75; font-size: 0.95rem;",
                    "Placeholder UI — A/V, screen share et spotlight présentateur arriveront en P0. "
                    "Le 360° fonctionnel est hors scaffold (P2)."
                }
            }
        }
    }
}
