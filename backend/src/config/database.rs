#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

impl DatabaseConfig {
    pub fn init() -> Self {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .expect("DATABASE_MAX_CONNECTIONS must be a valid number");

        let min_connections = std::env::var("DATABASE_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u32>()
            .expect("DATABASE_MIN_CONNECTIONS must be a valid number");

        let acquire_timeout_seconds = std::env::var("DATABASE_ACQUIRE_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .expect("DATABASE_ACQUIRE_TIMEOUT must be a valid number");

        let idle_timeout_seconds = std::env::var("DATABASE_IDLE_TIMEOUT")
            .unwrap_or_else(|_| "300".to_string())
            .parse::<u64>()
            .expect("DATABASE_IDLE_TIMEOUT must be a valid number");

        let max_lifetime_seconds = std::env::var("DATABASE_MAX_LIFETIME")
            .unwrap_or_else(|_| "1800".to_string())
            .parse::<u64>()
            .expect("DATABASE_MAX_LIFETIME must be a valid number");

        DatabaseConfig {
            url,
            max_connections,
            min_connections,
            acquire_timeout_seconds,
            idle_timeout_seconds,
            max_lifetime_seconds,
        }
    }
}
