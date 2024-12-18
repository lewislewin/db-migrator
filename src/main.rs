mod config;
mod db_connector;
mod schema_manager;
mod data_migrator;

use db_connector::connect_to_database;
use schema_manager::ensure_table_exists;
use data_migrator::copy_table_data;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = config::DbConfig::new();

    let source_pool = connect_to_database(&config.source_url).await;
    let target_pool = connect_to_database(&config.target_url).await;

    let table_name = "example_table";

    ensure_table_exists(&source_pool, &target_pool, table_name).await;

    println!("Copying data...");
    copy_table_data(&source_pool, &target_pool, table_name).await;

    println!("Data migration completed!");
}
