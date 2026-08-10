#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
    pub enable_tls: bool,
}

impl EmailConfig {
    pub fn init() -> Self {
        let smtp_host = std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());

        let smtp_port = std::env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse::<u16>()
            .expect("SMTP_PORT must be a valid number");

        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");

        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        let from_email = std::env::var("EMAIL_FROM_ADDRESS")
            .unwrap_or_else(|_| "noreply@atronsa.com".to_string());

        let from_name = std::env::var("EMAIL_FROM_NAME").unwrap_or_else(|_| "Atronsa".to_string());

        let enable_tls = std::env::var("SMTP_ENABLE_TLS")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .expect("SMTP_ENABLE_TLS must be true or false");

        EmailConfig {
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            from_email,
            from_name,
            enable_tls,
        }
    }
}
