#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Path, Query, State},
    http::{Method, StatusCode},
    routing::{delete, get, post, put},
    Json, Router,
};
use payloads::{NewTodoRequest, TodoCompletedRequest};
use todos::filter_todos;
use tokio::{net::TcpListener, sync::Mutex};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};
use uuid::Uuid;

use crate::errors::Result;
use crate::payloads::TodosFilterRequest;
use crate::todos::Todo;

mod errors;
mod payloads;
mod todos;

#[derive(Clone, Default)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::default();

    // Middlewares
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let timeout = TimeoutLayer::new(Duration::from_secs(3));

    // Router
    let exposed_router = Router::new().route("/", get(root));

    let protected_router = Router::new()
        .route("/todos", get(todos))
        .route("/todos/new", post(create_todo))
        .route("/todos/:id/set-completion", put(set_todo_completion))
        .route("/todos/:id/remove", delete(remove_todo))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors).layer(timeout));

    let app = Router::new().merge(exposed_router).merge(protected_router);

    // Starting
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello Todos!"
}

async fn todos(
    State(state): State<AppState>,
    todos_filters: Option<Query<TodosFilterRequest>>,
) -> Result<Json<Vec<Todo>>> {
    let todos = state.todos.lock().await;

    Ok(Json(filter_todos(
        &todos,
        todos_filters.unwrap_or_default().filter,
    )))
}

async fn create_todo(
    State(state): State<AppState>,
    Json(new_todo): Json<NewTodoRequest>,
) -> StatusCode {
    let mut todos = state.todos.lock().await;
    todos.push(Todo {
        id: Uuid::new_v4(),
        content: new_todo.content,
        completed: false,
    });
    StatusCode::NO_CONTENT
}

async fn set_todo_completion(
    Path(todo_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(todo_completed): Json<TodoCompletedRequest>,
) -> StatusCode {
    let mut todos = state.todos.lock().await;
    let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) else {
        return StatusCode::NOT_FOUND;
    };

    todo.completed = todo_completed.completed;

    StatusCode::NO_CONTENT
}

async fn remove_todo(Path(todo_id): Path<Uuid>, State(state): State<AppState>) -> StatusCode {
    let mut todos = state.todos.lock().await;
    let original_len = todos.len();
    todos.retain(|todo| todo.id != todo_id);
    if original_len == todos.len() {
        return StatusCode::NOT_FOUND;
    }

    StatusCode::NO_CONTENT
}
