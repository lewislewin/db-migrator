use crate::schema_manager::DatabaseHandler;

pub async fn copy_table_data<H: DatabaseHandler>(
    source_handler: &H,
    target_handler: &H,
    table_name: &str,
) {
    let batch_size = 1000;

    // Fetch all rows from the source table
    let rows = source_handler.fetch_rows(table_name).await;

    let mut batch = Vec::new();

    for row in rows {
        batch.push(row);

        if batch.len() >= batch_size {
            target_handler.insert_batch(table_name, &batch).await;
            batch.clear();
        }
    }

    // Insert remaining rows
    if !batch.is_empty() {
        target_handler.insert_batch(table_name, &batch).await;
    }
}
