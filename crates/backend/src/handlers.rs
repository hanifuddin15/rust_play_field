use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use shared::{ApiResponse, CreateTodoRequest, TodoItem, UpdateTodoRequest};

use crate::state::AppState;

/// GET /api/health
pub async fn health_check() -> impl IntoResponse {
    Json(ApiResponse::success("Axum backend is healthy!"))
}

/// GET /api/todos
pub async fn list_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = state.todos.read().await;
    let mut items: Vec<TodoItem> = todos.values().cloned().collect();
    items.sort_by_key(|item| item.id);
    Json(ApiResponse::success(items))
}

/// GET /api/todos/:id
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<TodoItem>>, StatusCode> {
    let todos = state.todos.read().await;
    match todos.get(&id) {
        Some(item) => Ok(Json(ApiResponse::success(item.clone()))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /api/todos
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TodoItem>>), (StatusCode, Json<ApiResponse<()>>)> {
    if payload.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Title cannot be empty")),
        ));
    }

    let mut next_id = state.next_id.write().await;
    let id = *next_id;
    *next_id += 1;

    let item = TodoItem {
        id,
        title: payload.title.trim().to_string(),
        description: payload
            .description
            .map(|d| d.trim().to_string())
            .filter(|d| !d.is_empty()),
        completed: false,
        created_at: Utc::now(),
    };

    let mut todos = state.todos.write().await;
    todos.insert(id, item.clone());

    Ok((StatusCode::CREATED, Json(ApiResponse::success(item))))
}

/// PUT /api/todos/:id
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<ApiResponse<TodoItem>>, StatusCode> {
    let mut todos = state.todos.write().await;
    let item = todos.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        let trimmed = title.trim();
        if !trimmed.is_empty() {
            item.title = trimmed.to_string();
        }
    }
    if let Some(desc) = payload.description {
        let trimmed = desc.trim();
        item.description = if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        };
    }
    if let Some(completed) = payload.completed {
        item.completed = completed;
    }

    Ok(Json(ApiResponse::success(item.clone())))
}

/// DELETE /api/todos/:id
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, StatusCode> {
    let mut todos = state.todos.write().await;
    if todos.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

