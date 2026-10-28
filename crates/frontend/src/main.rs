use dioxus::prelude::*;
use shared::TodoItem;

mod api;
mod components;

use components::{Header, ItemCard, ItemForm};

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut todos = use_signal(Vec::<TodoItem>::new);
    let mut error_msg = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| true);

    // Initial fetch on component mount
    use_effect(move || {
        spawn(async move {
            match api::fetch_todos().await {
                Ok(items) => {
                    todos.set(items);
                    error_msg.set(None);
                    is_loading.set(false);
                }
                Err(err) => {
                    error_msg.set(Some(err));
                    is_loading.set(false);
                }
            }
        });
    });

    // Handle creating a new task
    let handle_create = move |(title, desc): (String, Option<String>)| {
        spawn(async move {
            match api::create_todo(title, desc).await {
                Ok(new_item) => {
                    todos.write().push(new_item);
                    error_msg.set(None);
                }
                Err(err) => {
                    error_msg.set(Some(err));
                }
            }
        });
    };

    // Handle toggling complete status
    let handle_toggle = move |id: u64| {
        let current_status = todos.read().iter().find(|t| t.id == id).map(|t| t.completed);
        if let Some(completed) = current_status {
            let new_completed = !completed;
            spawn(async move {
                match api::update_todo_completed(id, new_completed).await {
                    Ok(updated) => {
                        if let Some(item) = todos.write().iter_mut().find(|t| t.id == id) {
                            *item = updated;
                        }
                        error_msg.set(None);
                    }
                    Err(err) => {
                        error_msg.set(Some(err));
                    }
                }
            });
        }
    };

    // Handle deleting a task
    let handle_delete = move |id: u64| {
        spawn(async move {
            match api::delete_todo(id).await {
                Ok(_) => {
                    todos.write().retain(|t| t.id != id);
                    error_msg.set(None);
                }
                Err(err) => {
                    error_msg.set(Some(err));
                }
            }
        });
    };

    let todo_count = todos.read().len();
    let completed_count = todos.read().iter().filter(|t| t.completed).count();

    rsx! {
        div { class: "app-container",
            Header {}

            if let Some(err) = error_msg.read().as_ref() {
                div { class: "alert alert-error",
                    strong { "Connection Error: " }
                    "{err}"
                    p { style: "font-size: 0.8rem; margin-top: 0.25rem;",
                        "Ensure the Axum backend is running on http://127.0.0.1:3000 (cargo run -p backend)"
                    }
                }
            }

            ItemForm { on_create: handle_create }

            section { class: "items-section",
                div { class: "section-header",
                    h2 { "Tasks" }
                    span { "{completed_count} of {todo_count} completed" }
                }

                if *is_loading.read() {
                    div { class: "empty-state",
                        span { class: "spinner" }
                        "Connecting to Axum backend..."
                    }
                } else if todos.read().is_empty() {
                    div { class: "empty-state",
                        p { "No tasks found. Use the form above to create one!" }
                    }
                } else {
                    for item in todos.read().iter().cloned() {
                        ItemCard {
                            key: "{item.id}",
                            item: item,
                            on_toggle: handle_toggle,
                            on_delete: handle_delete,
                        }
                    }
                }
            }
        }
    }
}

