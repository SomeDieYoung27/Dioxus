use dioxus::prelude::*;
use crate::models::{Todo, Priority};

#[derive(Props, Clone, PartialEq)]
pub struct TodoItemProps {
    pub todo: Todo,
    pub on_toggle: EventHandler<uuid::Uuid>,
    pub on_delete: EventHandler<uuid::Uuid>,
    pub on_edit: EventHandler<uuid::Uuid>,
}

#[component]
pub fn TodoItem(props: TodoItemProps) -> Element {
    let priority_color = match props.todo.priority {
        Priority::High => "text-red-500",
        Priority::Medium => "text-yellow-500",
        Priority::Low => "text-green-500",
    };

    rsx! {
        li {
            class: "flex items-center justify-between p-4 bg-white rounded-lg shadow-sm my-2",
            div {
                class: "flex items-center",
                input {
                    r#type: "checkbox",
                    checked: props.todo.completed,
                    onchange: move |_| props.on_toggle.call(props.todo.id),
                },
                div {
                    class: "ml-4",
                    p {
                        class: if props.todo.completed { "line-through text-gray-500" } else { "" },
                        "{props.todo.title}"
                    }
                    if let Some(desc) = &props.todo.description {
                        p { class: "text-sm text-gray-600", "{desc}" }
                    }
                }
            }
            div {
                class: "flex items-center",
                span { class: "mr-4 {priority_color}", "{props.todo.priority}" },
                button {
                    onclick: move |_| props.on_edit.call(props.todo.id),
                    class: "p-2 text-gray-400 hover:text-blue-600",
                    "✏️"
                }
                button {
                    onclick: move |_| props.on_delete.call(props.todo.id),
                    class: "p-2 text-gray-400 hover:text-red-600",
                    "🗑️"
                }
            }
        }
    }
} 