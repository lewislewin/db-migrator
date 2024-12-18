mod config;
mod db_connector;
mod schema_manager;
mod data_migrator;

use config::AppConfig;
use db_connector::{connect_to_database, DbPool};
use schema_manager::{ensure_table_exists, ensure_table_exists_mysql};
use data_migrator::{copy_table_data, copy_table_data_mysql};

#[tokio::main]
async fn main() {
    // Load the configuration from the config file
    let config = AppConfig::from_file("config.toml");

    // Connect to the source and target databases
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

    // Ensure the connection pools are for the same type of database
    match (source_pool, target_pool) {
        (DbPool::Postgres(source), DbPool::Postgres(target)) => {
            let table_name = "example_table"; // Replace with the actual table name

            ensure_table_exists(&source, &target, table_name).await;

            println!("Copying data...");
            copy_table_data(&source, &target, table_name).await;
        }
        (DbPool::MySql(source), DbPool::MySql(target)) => {
            let table_name = "example_table"; // Replace with the actual table name

            ensure_table_exists_mysql(&source, &target, table_name).await;

            println!("Copying data...");
            copy_table_data_mysql(&source, &target, table_name).await;
        }
        _ => panic!("Source and target databases must be of the same type"),
    }

    println!("Data migration completed!");
}
