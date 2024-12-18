mod config;
mod db_connector;
mod schema_manager;
mod data_migrator;

use config::AppConfig;
use db_connector::{connect_to_database, DbPool};
use schema_manager::{DatabaseHandler, MySqlHandler, PostgresHandler};
use data_migrator::copy_table_data;

#[tokio::main]
async fn main() {
    // Load configuration from the config file
    let config = AppConfig::from_file("config.toml");

    // Connect to source and target databases
    let source_pool = connect_to_database(
        &config.databases.source.r#type,
        &config.databases.source.url,
    )
    .await;

    let target_pool = connect_to_database(
        &config.databases.target.r#type,
        &config.databases.target.url,
    )
    .await;

    let table_name = "example_table"; // Replace with your actual table name

    // Dynamically determine database type and execute migration
    match (source_pool, target_pool) {
        (DbPool::Postgres(source), DbPool::Postgres(target)) => {
            let source_handler = PostgresHandler { pool: source };
            let target_handler = PostgresHandler { pool: target };

            println!("Ensuring table exists in target database...");
            target_handler.ensure_table_exists(table_name).await;

            println!("Copying data from source to target...");
            copy_table_data(&source_handler, &target_handler, table_name).await;
        }
        (DbPool::MySql(source), DbPool::MySql(target)) => {
            let source_handler = MySqlHandler { pool: source };
            let target_handler = MySqlHandler { pool: target };

            println!("Ensuring table exists in target database...");
            target_handler.ensure_table_exists(table_name).await;

            println!("Copying data from source to target...");
            copy_table_data(&source_handler, &target_handler, table_name).await;
        }
        _ => panic!("Source and target databases must be of the same type"),
    }

    println!("Data migration completed!");
}
