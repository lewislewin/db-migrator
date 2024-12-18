use sqlx::{Pool, MySql, Postgres};

pub enum DbPool {
    Postgres(Pool<Postgres>),
    MySql(Pool<MySql>),
}

pub async fn connect_to_database(db_type: &str, db_url: &str) -> DbPool {
    match db_type {
        "postgres" => {
            let pool = sqlx::postgres::PgPool::connect(db_url)
                .await
                .expect("Failed to connect to PostgreSQL database");
            DbPool::Postgres(pool)
        }
        "mysql" => {
            let pool = sqlx::mysql::MySqlPool::connect(db_url)
                .await
                .expect("Failed to connect to MySQL database");
            DbPool::MySql(pool)
        }
        _ => panic!("Unsupported database type: {}", db_type),
    }
}
