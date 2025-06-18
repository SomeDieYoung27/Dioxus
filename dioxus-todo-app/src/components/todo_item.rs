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
    let todo = &props.todo;
    
    // Priority color mapping
    let priority_color = match todo.priority {
        Priority::High => "bg-red-100 text-red-800 border-red-200",
        Priority::Medium => "bg-yellow-100 text-yellow-800 border-yellow-200",
        Priority::Low => "bg-green-100 text-green-800 border-green-200",
    };

    let completed_styles = if todo.completed {
        "bg-gray-100 opacity-75"
    } else {
        "bg-white"
    };

    rsx! {
        div { 
            class: "p-4 border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow {completed_styles}",
            
            div { 
                class: "flex items-start justify-between",
                
                // Left side - Checkbox and content
                div { 
                    class: "flex items-start space-x-3 flex-1",
                    
                    // Toggle checkbox
                    input { 
                        r#type: "checkbox",
                        class: "mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded cursor-pointer",
                        checked: todo.completed,
                        onchange: move |_| props.on_toggle.call(todo.id)
                    },
                    
                    // Todo content
                    div { 
                        class: "flex-1 min-w-0",
                        
                        // Title
                        h3 { 
                            class: if todo.completed { 
                                "text-lg font-medium text-gray-500 line-through" 
                            } else { 
                                "text-lg font-medium text-gray-900" 
                            },
                            "{todo.title}"
                        },
                        
                        // Description (if exists)
                        if let Some(ref description) = todo.description {
                            p { 
                                class: if todo.completed { 
                                    "mt-1 text-sm text-gray-400 line-through" 
                                } else { 
                                    "mt-1 text-sm text-gray-600" 
                                },
                                "{description}"
                            }
                        },
                        
                        // Metadata
                        div { 
                            class: "mt-2 flex items-center space-x-4 text-xs text-gray-500",
                            
                            // Priority badge
                            span { 
                                class: "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border {priority_color}",
                                "🔥 {todo.priority}"
                            },
                            
                            // Creation date
                            span { 
                                "Created: {}"
                                todo.created_at.format("%b %d, %Y at %H:%M")
                            },
                            
                            // Updated date (if different from created)
                            if todo.updated_at != todo.created_at {
                                span { 
                                    "Updated: {}"
                                    todo.updated_at.format("%b %d, %Y at %H:%M")
                                }
                            }
                        }
                    }
                },
                
                // Right side - Action buttons
                div { 
                    class: "flex items-center space-x-2",
                    
                    // Edit button
                    button { 
                        class: "p-2 text-gray-400 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors",
                        title: "Edit todo",
                        onclick: move |_| props.on_edit.call(todo.id),
                        "✏️"
                    },
                    
                    // Delete button
                    button { 
                        class: "p-2 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded-md transition-colors",
                        title: "Delete todo",
                        onclick: move |_| props.on_delete.call(todo.id),
                        "🗑️"
                    }
                }
            }
        }
    }
} 