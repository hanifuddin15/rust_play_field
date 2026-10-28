use shared::TodoItem;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state across Axum handlers
#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<RwLock<HashMap<u64, TodoItem>>>,
    pub next_id: Arc<RwLock<u64>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut initial_todos = HashMap::new();
        let now = chrono::Utc::now();

        let sample1 = TodoItem {
            id: 1,
            title: "Configure Cargo Workspace".to_string(),
            description: Some("Set up multi-crate workspace for Axum and Dioxus".to_string()),
            completed: true,
            created_at: now,
        };

        let sample2 = TodoItem {
            id: 2,
            title: "Build Dioxus Frontend UI".to_string(),
            description: Some("Implement interactive reactive components with signals".to_string()),
            completed: false,
            created_at: now,
        };

        let sample3 = TodoItem {
            id: 3,
            title: "Connect Dioxus with Axum REST API".to_string(),
            description: Some("Fetch, create, toggle, and delete items via HTTP endpoints".to_string()),
            completed: false,
            created_at: now,
        };

        initial_todos.insert(1, sample1);
        initial_todos.insert(2, sample2);
        initial_todos.insert(3, sample3);

        Self {
            todos: Arc::new(RwLock::new(initial_todos)),
            next_id: Arc::new(RwLock::new(4)),
        }
    }
}

