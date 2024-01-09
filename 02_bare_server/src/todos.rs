use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum TodosFilter {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "not-completed")]
    NotCompleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
    pub completed: bool,
}

#[allow(clippy::module_name_repetitions)]
pub fn filter_todos(todos: &[Todo], filter: TodosFilter) -> Vec<Todo> {
    match filter {
        TodosFilter::All => todos.to_vec(),
        TodosFilter::Completed => {
            let mut filtered_todos = Vec::with_capacity(todos.len());
            for todo in todos {
                if todo.completed {
                    filtered_todos.push(todo.clone());
                }
            }
            filtered_todos
        }
        TodosFilter::NotCompleted => {
            let mut filtered_todos = Vec::with_capacity(todos.len());
            for todo in todos {
                if !todo.completed {
                    filtered_todos.push(todo.clone());
                }
            }
            filtered_todos
        }
    }
}
