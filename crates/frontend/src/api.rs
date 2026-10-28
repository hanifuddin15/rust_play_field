use gloo_net::http::Request;
use shared::{ApiResponse, CreateTodoRequest, TodoItem, UpdateTodoRequest};

const API_BASE: &str = "http://127.0.0.1:3000/api";

/// Fetch all todos from the Axum backend
pub async fn fetch_todos() -> Result<Vec<TodoItem>, String> {
    let response = Request::get(&format!("{}/todos", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Server returned HTTP {}", response.status()));
    }

    let payload: ApiResponse<Vec<TodoItem>> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    payload.data.ok_or_else(|| "Empty response data".to_string())
}

/// Create a new todo item
pub async fn create_todo(title: String, description: Option<String>) -> Result<TodoItem, String> {
    let req = CreateTodoRequest { title, description };
    let response = Request::post(&format!("{}/todos", API_BASE))
        .json(&req)
        .map_err(|e| format!("Serialization error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Failed to create item: HTTP {}", response.status()));
    }

    let payload: ApiResponse<TodoItem> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    payload.data.ok_or_else(|| "Empty response data".to_string())
}

/// Toggle or update completion status
pub async fn update_todo_completed(id: u64, completed: bool) -> Result<TodoItem, String> {
    let req = UpdateTodoRequest {
        title: None,
        description: None,
        completed: Some(completed),
    };

    let response = Request::put(&format!("{}/todos/{}", API_BASE, id))
        .json(&req)
        .map_err(|e| format!("Serialization error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Failed to update item: HTTP {}", response.status()));
    }

    let payload: ApiResponse<TodoItem> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    payload.data.ok_or_else(|| "Empty response data".to_string())
}

/// Delete a todo item by id
pub async fn delete_todo(id: u64) -> Result<(), String> {
    let response = Request::delete(&format!("{}/todos/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() && response.status() != 204 {
        return Err(format!("Failed to delete item: HTTP {}", response.status()));
    }

    Ok(())
}

