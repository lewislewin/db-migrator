use async_trait::async_trait;
use sqlx::{Row, MySql, Pool, Postgres};

#[async_trait]
pub trait DatabaseHandler {
    async fn ensure_table_exists(&self, table_name: &str);
    async fn fetch_rows(&self, table_name: &str) -> Vec<Vec<String>>;
    async fn insert_batch(&self, table_name: &str, batch: &[Vec<String>]);
}

pub struct PostgresHandler {
    pub pool: Pool<Postgres>,
}

#[async_trait]
impl DatabaseHandler for PostgresHandler {
    async fn ensure_table_exists(&self, table_name: &str) {
        let table_schema_query = format!(
            "SELECT column_name, data_type 
             FROM information_schema.columns 
             WHERE table_name = '{}'",
            table_name
        );

        let rows = sqlx::query(&table_schema_query)
            .fetch_all(&self.pool)
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

        sqlx::query(&create_table_query)
            .execute(&self.pool)
            .await
            .expect("Failed to create table in target database");
    }

    async fn fetch_rows(&self, table_name: &str) -> Vec<Vec<String>> {
        let query = format!("SELECT * FROM {}", table_name);
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .expect("Failed to fetch rows from source database");

        rows.iter()
            .map(|row| {
                (0..row.len())
                    .map(|i| row.try_get::<String, _>(i).unwrap_or_default())
                    .collect()
            })
            .collect()
    }

    async fn insert_batch(&self, table_name: &str, batch: &[Vec<String>]) {
        let mut query = format!("INSERT INTO {} VALUES ", table_name);
        let values: Vec<String> = batch
            .iter()
            .map(|row| format!("({})", row.join(", ")))
            .collect();

        query.push_str(&values.join(", "));

        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .expect("Failed to insert batch into target database");
    }
}

pub struct MySqlHandler {
    pub pool: Pool<MySql>,
}

#[async_trait]
impl DatabaseHandler for MySqlHandler {
    async fn ensure_table_exists(&self, table_name: &str) {
        let table_schema_query = format!(
            "SELECT COLUMN_NAME, COLUMN_TYPE 
             FROM INFORMATION_SCHEMA.COLUMNS 
             WHERE TABLE_NAME = '{}'",
            table_name
        );

        let rows = sqlx::query(&table_schema_query)
            .fetch_all(&self.pool)
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

        sqlx::query(&create_table_query)
            .execute(&self.pool)
            .await
            .expect("Failed to create table in target database");
    }

    async fn fetch_rows(&self, table_name: &str) -> Vec<Vec<String>> {
        let query = format!("SELECT * FROM {}", table_name);
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .expect("Failed to fetch rows from source database");

        rows.iter()
            .map(|row| {
                (0..row.len())
                    .map(|i| row.try_get::<String, _>(i).unwrap_or_default())
                    .collect()
            })
            .collect()
    }

    async fn insert_batch(&self, table_name: &str, batch: &[Vec<String>]) {
        let mut query = format!("INSERT INTO {} VALUES ", table_name);
        let values: Vec<String> = batch
            .iter()
            .map(|row| format!("({})", row.join(", ")))
            .collect();

        query.push_str(&values.join(", "));

        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .expect("Failed to insert batch into target database");
    }
}
