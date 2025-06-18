use dioxus::prelude::*;
use crate::models::{Priority, TodoForm};
use crate::utils::validate_todo_title;

#[derive(Props, Clone, PartialEq)]
pub struct TodoFormProps {
    pub initial_form: Option<TodoForm>,
    pub on_submit: EventHandler<TodoForm>,
    pub on_cancel: EventHandler<()>,
    pub submit_text: String,
}

#[component]
pub fn TodoForm(props: TodoFormProps) -> Element {
    let mut form = use_signal(TodoForm::default);
    let mut title_error = use_signal(|| None::<String>);
    let mut is_submitting = use_signal(|| false);
    let api_client = reqwest::Client::new();
    let user_id = "some-user-id"; // TODO: get from auth context

    // Form handlers
    let handle_title_change = {
        let mut form = form.clone();
        let mut title_error = title_error.clone();
        move |evt: FormEvent| {
            let value = evt.value();
            form.with_mut(|f| f.title = value.clone());
            
            // Real-time validation
            match validate_todo_title(&value) {
                Ok(_) => title_error.set(None),
                Err(err) => title_error.set(Some(err)),
            }
        }
    };

    let handle_description_change = {
        let mut form = form.clone();
        move |evt: FormEvent| {
            form.with_mut(|f| f.description = evt.value());
        }
    };

    let handle_priority_change = {
        let mut form = form.clone();
        move |evt: FormEvent| {
            let priority = match evt.value().as_str() {
                "High" => Priority::High,
                "Medium" => Priority::Medium,
                "Low" => Priority::Low,
                _ => Priority::Medium,
            };
            form.with_mut(|f| f.priority = priority);
        }
    };

    let mut validate_form = move || -> bool {
        let mut valid = true;
        let form_data = form.read();
        if form_data.title.is_empty() {
            *title_error.write() = Some("Title is required".to_string());
            valid = false;
        } else {
            *title_error.write() = None;
        }
        valid
    };

    let submit_handler = move |_evt: FormEvent| {
        if !validate_form() || *is_submitting.read() {
            return;
        }
        spawn({
            let new_todo = form();
            async move {
                *is_submitting.write() = true;
                // ... api call ...
                *is_submitting.write() = false;
                form.set(TodoForm::default());
            }
        });
        let new_todo = form();
        props.on_submit.call(new_todo);
    };

    rsx! {
        div { 
            class: "bg-white p-6 rounded-lg shadow-md border border-gray-200",
            
            h2 { 
                class: "text-xl font-semibold text-gray-900 mb-4",
                if props.initial_form.is_some() {
                    "Edit Todo"
                } else {
                    "Add New Todo"
                }
            },
            
            form { 
                class: "space-y-4",
                onsubmit: submit_handler,
                
                // Title field
                div {
                    label { 
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        r#for: "title",
                        "Title *"
                    },
                    input { 
                        r#type: "text",
                        id: "title",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                        placeholder: "Enter todo title...",
                        value: "{form.read().title}",
                        oninput: handle_title_change,
                        required: true,
                        maxlength: 100
                    },
                    
                    // Title validation error
                    if let Some(ref error) = title_error.read().clone() {
                        p { 
                            class: "mt-1 text-sm text-red-600",
                            "{error}"
                        }
                    }
                },
                
                // Description field
                div {
                    label { 
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        r#for: "description",
                        "Description"
                    },
                    textarea { 
                        id: "description",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                        placeholder: "Enter todo description (optional)...",
                        rows: "3",
                        value: "{form.read().description}",
                        oninput: handle_description_change
                    }
                },
                
                // Priority field
                div {
                    label { 
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        r#for: "priority",
                        "Priority"
                    },
                    select { 
                        id: "priority",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                        value: "{form.read().priority}",
                        onchange: handle_priority_change,
                        
                        option { value: "Low", "🟢 Low Priority" },
                        option { value: "Medium", "🟡 Medium Priority" },
                        option { value: "High", "🔴 High Priority" }
                    }
                },
                
                // Action buttons
                div { 
                    class: "flex items-center justify-end space-x-3 pt-4",
                    
                    button { 
                        r#type: "button",
                        class: "px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500",
                        onclick: move |_| props.on_cancel.call(()),
                        "Cancel"
                    },
                    
                    button { 
                        r#type: "submit",
                        class: "px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: is_submitting.read().clone(),
                        
                        if *is_submitting.read() {
                            "Saving..."
                        } else {
                            "{props.submit_text}"
                        }
                    }
                }
            }
        }
    }
} 