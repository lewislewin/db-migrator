use sqlx::{Executor, Pool, Postgres, Row};

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
