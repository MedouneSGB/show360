#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Shell Tauri 2 — charge l'UI web/Dioxus (placeholder P0).
//!
//! Les commandes Rust exposent les types core (rôles, salle) pour la trajectoire
//! présentateur / screen share. Le branchement LiveKit SDK arrive en P0 suivant.

use serde::Serialize;
use show360_core::{Role, RoomMode, P0_MAX_PARTICIPANTS};

#[derive(Serialize)]
struct DesktopInfo {
    app: &'static str,
    role_model: Vec<&'static str>,
    room_mode_default: &'static str,
    p0_max_participants: u32,
    notes: &'static str,
}

#[tauri::command]
fn desktop_info() -> DesktopInfo {
    DesktopInfo {
        app: "show360-desktop",
        role_model: vec![
            Role::Host.as_str(),
            Role::Presenter.as_str(),
            Role::Participant.as_str(),
            Role::Spectator.as_str(),
        ],
        room_mode_default: match RoomMode::SelfHosted {
            RoomMode::SelfHosted => "self_hosted",
            RoomMode::Cloud => "cloud",
        },
        p0_max_participants: P0_MAX_PARTICIPANTS,
        notes: "Stub Tauri 2 — UI Dioxus via webview ; A/V + présentateur en lots P0.",
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![desktop_info])
        .run(tauri::generate_context!())
        .expect("erreur démarrage Show360 desktop");
}
