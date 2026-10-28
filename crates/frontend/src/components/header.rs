use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "app-header",
            div { class: "brand-row",
                h1 { class: "brand-title", "🦀 Fullstack Rust Workspace" }
                div { class: "badges",
                    span { class: "badge badge-axum", "Axum 0.7" }
                    span { class: "badge badge-dioxus", "Dioxus 0.6" }
                }
            }
            p { class: "subtitle",
                "Unified Cargo workspace featuring shared domain types, Axum async backend, and reactive Dioxus frontend."
            }
        }
    }
}

