use sqlx::{Pool, Postgres, Row};
use futures_util::stream::TryStreamExt;

pub async fn copy_table_data(
    source_pool: &Pool<Postgres>,
    target_pool: &Pool<Postgres>,
    table_name: &str,
) {
    let batch_size = 1000;
    let query = format!("SELECT * FROM {}", table_name);

    let mut rows_stream = sqlx::query(&query).fetch(source_pool);
    let mut batch = Vec::new();

    while let Some(row) = rows_stream.try_next().await.unwrap() {
        let values: Vec<String> = (0..row.len())
            .map(|i| row.try_get::<String, _>(i).unwrap_or_default())
            .collect();

        batch.push(values);

        if batch.len() >= batch_size {
            insert_batch(target_pool, table_name, &batch).await;
            batch.clear();
        }
    }

    if !batch.is_empty() {
        insert_batch(target_pool, table_name, &batch).await;
    }
}

async fn insert_batch(
    target_pool: &Pool<Postgres>,
    table_name: &str,
    batch: &Vec<Vec<String>>,
) {
    let mut query = format!("INSERT INTO {} VALUES ", table_name);
    let values: Vec<String> = batch
        .iter()
        .map(|row| format!("({})", row.join(", ")))
        .collect();

    query.push_str(&values.join(", "));

    sqlx::query(&query)
        .execute(target_pool)
        .await
        .expect("Failed to insert batch into target database");
}
