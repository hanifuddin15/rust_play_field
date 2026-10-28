use dioxus::prelude::*;
use shared::TodoItem;

#[component]
pub fn ItemCard(
    item: TodoItem,
    on_toggle: EventHandler<u64>,
    on_delete: EventHandler<u64>,
) -> Element {
    let card_class = if item.completed {
        "item-card completed"
    } else {
        "item-card"
    };

    let id = item.id;
    let created = item.created_at.format("%Y-%m-%d %H:%M UTC").to_string();

    rsx! {
        div { class: "{card_class}",
            div { class: "item-left",
                input {
                    r#type: "checkbox",
                    class: "checkbox",
                    checked: item.completed,
                    onchange: move |_| on_toggle.call(id),
                }
                div { class: "item-content",
                    h3 { class: "item-title", "{item.title}" }
                    if let Some(desc) = &item.description {
                        p { class: "item-desc", "{desc}" }
                    }
                    span { class: "item-meta", "Created: {created} • ID: #{item.id}" }
                }
            }
            button {
                class: "btn btn-danger",
                onclick: move |_| on_delete.call(id),
                "Delete"
            }
        }
    }
}

