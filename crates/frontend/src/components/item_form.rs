use dioxus::prelude::*;

#[component]
pub fn ItemForm(on_create: EventHandler<(String, Option<String>)>) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut is_submitting = use_signal(|| false);

    let is_empty = title.read().trim().is_empty();

    let handle_submit = move |_| {
        let current_title = title.read().trim().to_string();
        if current_title.is_empty() {
            return;
        }
        let desc = description.read().trim().to_string();
        let current_desc = if desc.is_empty() {
            None
        } else {
            Some(desc)
        };

        is_submitting.set(true);
        on_create.call((current_title, current_desc));
        title.set(String::new());
        description.set(String::new());
        is_submitting.set(false);
    };

    rsx! {
        div { class: "form-card",
            h2 { class: "form-title", "Add a New Task" }
            div { class: "form-group",
                input {
                    class: "form-input",
                    placeholder: "Task title (e.g. Implement user authentication)",
                    value: "{title}",
                    oninput: move |evt| title.set(evt.value()),
                }
            }
            div { class: "form-group",
                textarea {
                    class: "form-textarea",
                    placeholder: "Description or notes (optional)",
                    value: "{description}",
                    oninput: move |evt| description.set(evt.value()),
                }
            }
            button {
                class: "btn btn-primary",
                disabled: is_empty || *is_submitting.read(),
                onclick: handle_submit,
                if *is_submitting.read() {
                    rsx! { span { class: "spinner" } "Adding..." }
                } else {
                    rsx! { "Add Task" }
                }
            }
        }
    }
}

