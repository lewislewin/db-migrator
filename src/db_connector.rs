use sqlx::{Pool, Postgres};

pub async fn connect_to_database(db_url: &str) -> Pool<Postgres> {
    sqlx::postgres::PgPool::connect(db_url)
        .await
        .expect("Failed to connect to the database")
}
