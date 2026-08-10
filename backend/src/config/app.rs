use super::database::DatabaseConfig;
use super::email::EmailConfig;
use super::jwt::JwtConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub email: EmailConfig,
    pub port: u16,
    pub environment: Environment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Staging,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::Staging => "staging",
        }
    }

    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }

    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }
}

impl AppConfig {
    pub fn init() -> Self {
        if cfg!(debug_assertions) {
            dotenvy::dotenv().ok();
        }

        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        let environment = match environment.as_str() {
            "production" => Environment::Production,
            "staging" => Environment::Staging,
            _ => Environment::Development,
        };

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");

        AppConfig {
            database: DatabaseConfig::init(),
            jwt: JwtConfig::init(),
            email: EmailConfig::init(),
            port,
            environment,
        }
    }

    pub fn server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
