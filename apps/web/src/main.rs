#![allow(non_snake_case)]

use dioxus::prelude::*;
use show360_core::{Role, RoomMode};
use show360_ui::{Show360App, UiSession};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let session = UiSession {
        room_name: "Salle démo".into(),
        room_code: "ABC123".into(),
        display_name: "Visiteur web".into(),
        role: Role::Participant,
        mode: RoomMode::SelfHosted,
        presenter_name: Some("Présentateur".into()),
    };

    rsx! {
        Show360App { session }
    }
}
