use dioxus::prelude::*;
use show360_core::Role;

#[component]
pub fn PresenterBadge(role: Role, presenter_name: Option<String>) -> Element {
    let is_presenter = matches!(role, Role::Host | Role::Presenter);
    let label = presenter_name
        .clone()
        .unwrap_or_else(|| "non assigné".into());

    rsx! {
        div {
            style: "display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.35rem 0.75rem; background: rgba(15, 23, 42, 0.55); border-radius: 0.35rem; font-size: 0.9rem;",
            span { "Présentateur : {label}" }
            if is_presenter {
                span {
                    style: "color: #5eead4; font-weight: 600;",
                    "· vous"
                }
            }
        }
    }
}
