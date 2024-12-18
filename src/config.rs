pub struct DbConfig {
    pub source_url: String,
    pub target_url: String,
}

impl DbConfig {
    pub fn new() -> Self {
        Self {
            source_url: std::env::var("SOURCE_DB_URL").expect("SOURCE_DB_URL not set"),
            target_url: std::env::var("TARGET_DB_URL").expect("TARGET_DB_URL not set"),
        }
    }
}
