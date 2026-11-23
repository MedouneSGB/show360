use dioxus::prelude::*;

#[component]
pub fn AppShell(title: String, children: Element) -> Element {
    rsx! {
        div {
            class: "show360-shell",
            style: "min-height: 100vh; font-family: 'Segoe UI', system-ui, sans-serif; background: linear-gradient(160deg, #0f172a 0%, #1e293b 45%, #0f766e 100%); color: #f8fafc;",
            header {
                style: "padding: 1.25rem 1.75rem; display: flex; align-items: baseline; gap: 1rem; border-bottom: 1px solid rgba(248,250,252,0.12);",
                h1 {
                    style: "margin: 0; font-size: 1.75rem; letter-spacing: 0.04em; font-weight: 700;",
                    "{title}"
                }
                span {
                    style: "opacity: 0.7; font-size: 0.9rem;",
                    "visioconférence · projection · présentateur"
                }
            }
            main {
                style: "padding: 1.75rem;",
                {children}
            }
        }
    }
}
