#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// 0. Remove me
#![allow(unused, unused_imports)]

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

    // let state = AppState::default();

    // Middlewares

    // 1. Adding the CorsLayer middleware
    // let cors = CorsLayer::new()
    //     .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    //     .allow_origin(Any);

    // 1b. (optional) Adding the TimeouLayer middleware
    // let timeout = TimeoutLayer::new(Duration::from_secs(3));

    // Router
    // 2. Creating the routers, an "exposed" one that everyone can access and a protected one (for the next exercise)
    // let exposed_router = Router::new().route("/", get(root));

    // let protected_router = Router::new()
    //     .route("/todos", get(todos))
    //     .route("/todos/new", post(create_todo))
    //     .route("/todos/:id/set-completion", put(set_todo_completion))
    //     .route("/todos/:id/remove", delete(remove_todo));

    // 3. Injecting the global state
    // .with_state(state)

    // 4. Adding the middlewares using a service
    // .layer(ServiceBuilder::new().layer(cors).layer(timeout));

    // 5. Creating the app itself
    // let app = Router::new().merge(exposed_router).merge(protected_router);

    // 6. Starting the app
    // let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

// 7. The barebone `root` handler
// async fn root() -> &'static str {
//     "Hello Todos!"
// }

// 8. Adding the `todos` handler that takes a `TodosFilterRequest` and return the todos filtered
// async fn todos(
//     State(state): State<AppState>,
//     todos_filters: Option<Query<TodosFilterRequest>>,
// ) -> Result<Json<Vec<Todo>>> {
//     let todos = state.todos.lock().await;

//     Ok(Json(filter_todos(
//         &todos,
//         todos_filters.unwrap_or_default().filter,
//     )))
// }

// 9. Adding the `create_todo` handler
// async fn create_todo(
//     State(state): State<AppState>,
//     Json(new_todo): Json<NewTodoRequest>,
// ) -> StatusCode {
//     let mut todos = state.todos.lock().await;
//     todos.push(Todo {
//         id: Uuid::new_v4(),
//         content: new_todo.content,
//         completed: false,
//     });
//     StatusCode::NO_CONTENT
// }

// 10. Adding the `set_todo_completion` handler
// async fn set_todo_completion(
//     Path(todo_id): Path<Uuid>,
//     State(state): State<AppState>,
//     Json(todo_completed): Json<TodoCompletedRequest>,
// ) -> StatusCode {
//     let mut todos = state.todos.lock().await;
//     let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) else {
//         return StatusCode::NOT_FOUND;
//     };

//     todo.completed = todo_completed.completed;

//     StatusCode::NO_CONTENT
// }

// 11. Adding the `remove_todo` function
// async fn remove_todo(Path(todo_id): Path<Uuid>, State(state): State<AppState>) -> StatusCode {
//     let mut todos = state.todos.lock().await;
//     let original_len = todos.len();
//     todos.retain(|todo| todo.id != todo_id);
//     if original_len == todos.len() {
//         return StatusCode::NOT_FOUND;
//     }

//     StatusCode::NO_CONTENT
// }
