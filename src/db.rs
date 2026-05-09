use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://tasks.db")
        .await
        .expect("Failed to connect to DB");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    pool
}