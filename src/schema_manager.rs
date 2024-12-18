use sqlx::{Executor, Pool, Postgres, MySql, Row};

/// Ensure a table exists in the target PostgreSQL database by replicating the schema from the source PostgreSQL database.
pub async fn ensure_table_exists(
    source_pool: &Pool<Postgres>,
    target_pool: &Pool<Postgres>,
    table_name: &str,
) {
    let table_schema_query = format!(
        "SELECT column_name, data_type 
         FROM information_schema.columns 
         WHERE table_name = '{}'",
        table_name
    );

    let rows = sqlx::query(&table_schema_query)
        .fetch_all(source_pool)
        .await
        .expect("Failed to fetch table schema");

    let columns: Vec<String> = rows
        .iter()
        .map(|row| {
            let col_name: &str = row.get("column_name");
            let data_type: &str = row.get("data_type");
            format!("{} {}", col_name, data_type)
        })
        .collect();

    let create_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({})",
        table_name,
        columns.join(", ")
    );

    target_pool
        .execute(&*create_table_query)
        .await
        .expect("Failed to create table in target database");
}

/// Ensure a table exists in the target MySQL database by replicating the schema from the source MySQL database.
pub async fn ensure_table_exists_mysql(
    source_pool: &Pool<MySql>,
    target_pool: &Pool<MySql>,
    table_name: &str,
) {
    let table_schema_query = format!(
        "SELECT COLUMN_NAME, COLUMN_TYPE 
         FROM INFORMATION_SCHEMA.COLUMNS 
         WHERE TABLE_NAME = '{}'",
        table_name
    );

    let rows = sqlx::query(&table_schema_query)
        .fetch_all(source_pool)
        .await
        .expect("Failed to fetch table schema");

    let columns: Vec<String> = rows
        .iter()
        .map(|row| {
            let col_name: &str = row.get("COLUMN_NAME");
            let col_type: &str = row.get("COLUMN_TYPE");
            format!("{} {}", col_name, col_type)
        })
        .collect();

    let create_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({})",
        table_name,
        columns.join(", ")
    );

    target_pool
        .execute(&*create_table_query)
        .await
        .expect("Failed to create table in target database");
}
