use std::time::Duration;

use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Row, Sqlite, SqliteConnection,
};
use tracing::debug;

pub async fn create_db_pool(filename: &str) -> Result<Pool<Sqlite>> {
    let pool_opts = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(1))
        .connect_with(pool_opts)
        .await?;

    let mut conn = pool.acquire().await?;
    perform_migrations(conn.as_mut()).await?;

    Ok(pool)
}

async fn perform_migrations(conn: &mut SqliteConnection) -> Result<()> {
    debug!("performing migrations");
    sqlx::query("CREATE TABLE IF NOT EXISTS migrations (version INTEGER NOT NULL)")
        .execute(&mut *conn)
        .await?;
    let migration_row = sqlx::query("SELECT version FROM migrations")
        .fetch_optional(&mut *conn)
        .await?;
    let mut current_version = if let Some(row) = migration_row {
        row.try_get("version")?
    } else {
        debug!("migrations are pristine, initializing the table");
        sqlx::query("INSERT INTO migrations VALUES (0)")
            .execute(&mut *conn)
            .await?;
        0
    };

    #[allow(unused_assignments)]
    if current_version == 0 {
        current_version = create_todos_table(&mut *conn).await?;
    }

    debug!("migrations done");
    Ok(())
}

async fn create_todos_table(conn: &mut SqliteConnection) -> Result<i64> {
    debug!("creating the todos table");
    sqlx::query("CREATE TABLE IF NOT EXISTS todos (id BLOB PRIMARY KEY NOT NULL, content TEXT NOT NULL, completed BOOLEAN NOT NULL)").execute(&mut *conn).await?;
    sqlx::query("UPDATE migrations SET version = 1")
        .execute(&mut *conn)
        .await?;

    Ok(1)
}
