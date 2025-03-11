use sqlx::{Pool, Postgres};

pub async fn connect() -> Pool<Postgres> {
    sqlx::PgPool::connect("postgres://user:password@localhost/drought_db")
        .await
        .expect("Failed to connect to database")
}

