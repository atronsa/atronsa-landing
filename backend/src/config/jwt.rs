use chrono::Duration;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_maxage: i64,
    pub refresh_token_maxage: i64,
    pub issuer: String,
    pub audience: String,
}

impl JwtConfig {
    pub fn init() -> Self {
        let secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

        let access_token_maxage = std::env::var("JWT_ACCESS_TOKEN_MAXAGE")
            .unwrap_or_else(|_| "15".to_string()) // Default 15 minutes
            .parse::<i64>()
            .expect("JWT_ACCESS_TOKEN_MAXAGE must be a valid number");

        let refresh_token_maxage = std::env::var("JWT_REFRESH_TOKEN_MAXAGE")
            .unwrap_or_else(|_| "10080".to_string()) // Default 7 days
            .parse::<i64>()
            .expect("JWT_REFRESH_TOKEN_MAXAGE must be a valid number");

        let issuer = std::env::var("JWT_ISSUER").unwrap_or_else(|_| "atronsa".to_string());

        let audience =
            std::env::var("JWT_AUDIENCE").unwrap_or_else(|_| "atronsa_users".to_string());

        JwtConfig {
            secret,
            access_token_maxage,
            refresh_token_maxage,
            issuer,
            audience,
        }
    }

    pub fn access_token_duration(&self) -> Duration {
        Duration::minutes(self.access_token_maxage)
    }

    pub fn refresh_token_duration(&self) -> Duration {
        Duration::minutes(self.refresh_token_maxage)
    }
}
