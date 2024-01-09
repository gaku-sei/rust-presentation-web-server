#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// 0. Remove me
#![allow(unused, unused_imports)]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    extract::{Path, Query, State},
    http::{Method, StatusCode},
    middleware,
    routing::{delete, get, post, put},
    Json, Router,
};
use db::create_db_pool;
use middlewares::auth;
use payloads::{NewTodoRequest, TodoCompletedRequest};
use sqlx::{Pool, Sqlite};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};
use uuid::Uuid;

use crate::errors::Result;
use crate::payloads::TodosFilterRequest;
use crate::todos::Todo;

mod db;
mod errors;
mod middlewares;
mod payloads;
mod todos;

static DB_FILENAME: &str = "todos.db";

#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<Vec<Todo>>>,
    // pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 1. Replace the State with the new one, drop the todos, add the pool
    let state = AppState {
        todos: Arc::new(Mutex::new(Vec::new())),
    };
    // let pool = create_db_pool(DB_FILENAME).await?;
    // let state = AppState { pool };

    // Middlewares
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let timeout = TimeoutLayer::new(Duration::from_secs(3));

    // 2. Add the new auth layer
    // let auth = middleware::from_fn(auth);

    // Router
    let exposed_router = Router::new().route("/", get(root));

    let protected_router = Router::new()
        .route("/todos", get(todos))
        .route("/todos/new", post(create_todo))
        .route("/todos/:id/set-completion", put(set_todo_completion))
        .route("/todos/:id/remove", delete(remove_todo))
        .with_state(state)
        // 3. Add the auth layer to the middlewares
        // .layer(ServiceBuilder::new().layer(cors).layer(timeout).layer(auth));
        .layer(ServiceBuilder::new().layer(cors).layer(timeout));

    let app = Router::new().merge(exposed_router).merge(protected_router);

    // Starting
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello Todos!"
}

// 4. Reimplement the `todos` handler
async fn todos(
    State(state): State<AppState>,
    todos_filters: Option<Query<TodosFilterRequest>>,
) -> Result<Json<Vec<Todo>>> {
    // let conn = state.pool.acquire().await?;
    // let todos = todos::select_todos(conn, todos_filters.unwrap_or_default().filter).await?;
    // Ok(Json(todos))
    todo!()
}

// 5. Reimplement the `create_todo` handler
async fn create_todo(
    State(state): State<AppState>,
    Json(new_todo): Json<NewTodoRequest>,
) -> Result<StatusCode> {
    // let conn = state.pool.acquire().await?;
    // todos::create_todo(conn, &new_todo.content).await?;
    // Ok(StatusCode::NO_CONTENT)
    todo!()
}

// 6. Reimplement the `set_todo_completion` handler
async fn set_todo_completion(
    Path(todo_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(todo_completed): Json<TodoCompletedRequest>,
) -> Result<StatusCode> {
    // let conn = state.pool.acquire().await?;
    // todos::set_todo_completion(conn, &todo_id, todo_completed.completed).await?;
    // Ok(StatusCode::NO_CONTENT)
    todo!()
}

// 7. Reimplement the `remove_todo` handler
async fn remove_todo(
    Path(todo_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode> {
    // let conn = state.pool.acquire().await?;
    // todos::remove_todo(conn, &todo_id).await?;
    // Ok(StatusCode::NO_CONTENT)
    todo!()
}
