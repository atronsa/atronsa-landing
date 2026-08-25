pub mod app;
pub mod database;
pub mod email;
pub mod env;
pub mod jwt;

pub use app::AppConfig;
pub use database::DatabaseConfig;
pub use email::EmailConfig;
pub use env::Env;
pub use jwt::JwtConfig;

use std::sync::OnceLock;

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| AppConfig::init())
}
