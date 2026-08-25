use std::path::Path;

#[derive(Debug, Clone)]
pub struct Env;

impl Env {
    pub fn load() {
        if cfg!(debug_assertions) {
            if Path::new(".env").exists() {
                dotenvy::dotenv().ok();
                println!("Loaded .env file");
            } else {
                println!("No .env file found. Create one from .env.example");
            }
        }

        if !cfg!(debug_assertions) {
            println!("Running in production mode - using system environment variables");
        }
    }

    pub fn validate() -> Result<(), Vec<String>> {
        let required_vars = vec!["DATABASE_URL", "JWT_SECRET_KEY"];

        let optional_vars = vec!["SMTP_HOST", "SMTP_USERNAME", "SMTP_PASSWORD"];

        let mut missing = Vec::new();

        for var in required_vars {
            if std::env::var(var).is_err() {
                missing.push(format!("Required variable '{}' is not set", var));
            }
        }

        for var in optional_vars {
            if std::env::var(var).is_err() {
                println!("Optional variable '{}' is not set", var);
            }
        }

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}
