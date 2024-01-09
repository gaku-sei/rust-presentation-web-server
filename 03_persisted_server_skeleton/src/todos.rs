use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, Sqlite};
use uuid::Uuid;

use crate::errors::Result;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
    pub completed: bool,
}

#[allow(clippy::module_name_repetitions)]
pub async fn select_todos(
    mut conn: PoolConnection<Sqlite>,
    filter: TodosFilter,
) -> Result<Vec<Todo>> {
    let query = match filter {
        TodosFilter::All => "SELECT id, content, completed FROM todos",
        TodosFilter::Completed => "SELECT id, content, completed FROM todos WHERE completed = 1",
        TodosFilter::NotCompleted => "SELECT id, content, completed FROM todos WHERE completed = 0",
    };

    let todos = sqlx::query_as(query).fetch_all(conn.as_mut()).await?;

    Ok(todos)
}

pub async fn create_todo(mut conn: PoolConnection<Sqlite>, content: &str) -> Result<()> {
    sqlx::query("INSERT INTO todos VALUES (?, ?, ?)")
        .bind(Uuid::new_v4())
        .bind(content)
        .bind(false)
        .execute(conn.as_mut())
        .await?;

    Ok(())
}

pub async fn set_todo_completion(
    mut conn: PoolConnection<Sqlite>,
    todo_id: &Uuid,
    completed: bool,
) -> Result<()> {
    sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
        .bind(completed)
        .bind(todo_id)
        .execute(conn.as_mut())
        .await?;

    Ok(())
}

pub async fn remove_todo(mut conn: PoolConnection<Sqlite>, todo_id: &Uuid) -> Result<()> {
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(todo_id)
        .execute(conn.as_mut())
        .await?;

    Ok(())
}
