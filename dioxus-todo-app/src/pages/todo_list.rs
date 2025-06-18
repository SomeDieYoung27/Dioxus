use dioxus::prelude::*;
use dioxus_router::prelude::*;
use uuid::Uuid;
use reqwest;

use crate::{
    Route,
    models::{Todo, Priority, TodoForm},
    components::{TodoItem, TodoForm as TodoFormComponent},
};

const API_URL: &str = "http://localhost:3000/api";

#[derive(Debug, Clone, PartialEq)]
enum FilterState {
    All,
    Active,
    Completed,
}

#[derive(Debug, Clone, PartialEq)]
enum ViewState {
    List,
    AddForm,
    EditForm(Uuid),
}

async fn fetch_todos() -> Result<Vec<Todo>, reqwest::Error> {
    let url = format!("{}/todos", API_URL);
    let todos = reqwest::get(&url).await?.json::<Vec<Todo>>().await?;
    Ok(todos)
}

#[component]
pub fn TodoList() -> Element {
    let mut todos = use_resource(fetch_todos);
    let mut filter = use_signal(|| FilterState::All);
    let mut view = use_signal(|| ViewState::List);

    let filtered_todos = use_memo(move || {
        if let Some(Ok(todos_vec)) = todos.read().as_ref() {
            todos_vec.iter().filter(|todo| match *filter.read() {
                FilterState::All => true,
                FilterState::Active => !todo.completed,
                FilterState::Completed => todo.completed,
            }).cloned().collect::<Vec<Todo>>()
        } else {
            vec![]
        }
    });

    let handle_add_todo = move |form: TodoForm| {
        spawn(async move {
            let client = reqwest::Client::new();
            let url = format!("{}/todos", API_URL);
            let mut new_todo = serde_json::json!({
                "title": form.title,
                "priority": form.priority,
            });
            if !form.description.is_empty() {
                new_todo["description"] = serde_json::Value::String(form.description);
            }

            if let Ok(_) = client.post(&url).json(&new_todo).send().await {
                todos.restart();
            }
            view.set(ViewState::List);
        });
    };

    let handle_update_todo = move |(id, form): (Uuid, TodoForm)| {
        spawn(async move {
            let client = reqwest::Client::new();
            let url = format!("{}/todos/{}", API_URL, id);
            let mut updated_todo = serde_json::json!({
                "title": form.title,
                "priority": form.priority,
            });
             if !form.description.is_empty() {
                updated_todo["description"] = serde_json::Value::String(form.description);
            }

            if let Ok(_) = client.put(&url).json(&updated_todo).send().await {
                todos.restart();
            }
            view.set(ViewState::List);
        });
    };

    let handle_delete_todo = move |id: Uuid| {
        spawn(async move {
            let client = reqwest::Client::new();
            let url = format!("{}/todos/{}", API_URL, id);
            if let Ok(_) = client.delete(&url).send().await {
                todos.restart();
            }
        });
    };

    let handle_toggle_todo = move |id: Uuid| {
        if let Some(Ok(todos_vec)) = todos.read().as_ref() {
            if let Some(todo) = todos_vec.iter().find(|t| t.id == id) {
                let new_completed_status = !todo.completed;
                spawn(async move {
                    let client = reqwest::Client::new();
                    let url = format!("{}/todos/{}", API_URL, id);
                    let updated_todo = serde_json::json!({ "completed": new_completed_status });
                    if let Ok(_) = client.put(&url).json(&updated_todo).send().await {
                        todos.restart();
                    }
                });
            }
        }
    };

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 p-4 sm:p-8",
            div {
                class: "max-w-3xl mx-auto",

                // Header
                div {
                    class: "flex justify-between items-center mb-6",
                    h1 {
                        class: "text-3xl font-bold text-gray-900",
                        "My Todos"
                    },
                    Link {
                        to: Route::Home {},
                        class: "text-blue-600 hover:text-blue-800 text-sm font-medium",
                        "← Back to Home"
                    }
                }

                // Main content area
                div {
                    class: "bg-white p-6 rounded-lg shadow-md",
                    match *view.read() {
                        ViewState::List => {
                            match &*todos.read() {
                                Some(Ok(_)) => rsx! {
                                    // Controls: Filter and Add button
                                    div {
                                        class: "flex flex-col sm:flex-row justify-between items-center mb-4 gap-4",
                                        // Filter buttons
                                        div {
                                            class: "flex space-x-2 p-1 bg-gray-100 rounded-lg",
                                            for (f, label) in [
                                                (FilterState::All, "All"),
                                                (FilterState::Active, "Active"),
                                                (FilterState::Completed, "Completed")
                                            ] {
                                                button {
                                                    class: if *filter.read() == f { "bg-white text-blue-600 shadow-sm" } else { "text-gray-500 hover:text-gray-700" },
                                                    class: "px-3 py-1 rounded-md text-sm font-medium transition-colors",
                                                    onclick: move |_| filter.set(f.clone()),
                                                    "{label}"
                                                }
                                            }
                                        }
                                        // Add Todo button
                                        button {
                                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors shadow",
                                            onclick: move |_| view.set(ViewState::AddForm),
                                            "＋ Add Todo"
                                        }
                                    }

                                    // Todo List
                                    if filtered_todos.read().is_empty() {
                                        // Empty state message
                                        div {
                                            class: "text-center py-12",
                                            p {
                                                class: "text-gray-500",
                                                "No todos here. Add one to get started!"
                                            }
                                        }
                                    } else {
                                        // List of todos
                                        ul {
                                            class: "space-y-3",
                                            for todo in filtered_todos.read().iter() {
                                                TodoItem {
                                                    key: "{todo.id}",
                                                    todo: todo.clone(),
                                                    on_toggle: move |_| handle_toggle_todo(todo.id),
                                                    on_edit: move |_| view.set(ViewState::EditForm(todo.id)),
                                                    on_delete: move |_| handle_delete_todo(todo.id),
                                                }
                                            }
                                        }
                                    }
                                },
                                Some(Err(e)) => rsx! { p { class: "text-red-500", "Error loading todos: {e}" } },
                                None => rsx! { p { class: "text-gray-500", "Loading..." } },
                            }
                        },

                        // ViewState::AddForm: Show the form for adding a new todo
                        ViewState::AddForm => rsx! {
                            TodoFormComponent {
                                on_submit: handle_add_todo,
                                on_cancel: move |_| view.set(ViewState::List),
                                submit_text: "Create Todo".to_string()
                            }
                        },

                        // ViewState::EditForm: Show the form for editing a todo
                        ViewState::EditForm(id) => {
                             if let Some(Ok(todos_vec)) = todos.read().as_ref() {
                                if let Some(todo) = todos_vec.iter().find(|t| t.id == id).cloned() {
                                    let initial_value = TodoForm {
                                        title: todo.title.clone(),
                                        description: todo.description.clone().unwrap_or_default(),
                                        priority: todo.priority,
                                    };
                                    rsx! {
                                        TodoFormComponent {
                                            initial_form: Some(initial_value),
                                            on_submit: move |form| handle_update_todo((id, form)),
                                            on_cancel: move |_| view.set(ViewState::List),
                                            submit_text: "Update Todo".to_string()
                                        }
                                    }
                                } else {
                                     rsx! {
                                        p {
                                            class: "text-red-500",
                                            "Error: Todo not found."
                                        }
                                        button {
                                            class: "mt-2 text-blue-600",
                                            onclick: move |_| view.set(ViewState::List),
                                            "Back to list"
                                        }
                                    }
                                }
                            } else {
                                rsx! { p { class: "text-gray-500", "Loading..." } }
                            }
                        }
                    }
                }
            }
        }
    }
} 