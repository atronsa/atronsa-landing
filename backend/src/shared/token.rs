use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::JwtConfig;
use crate::error::{AppError, ErrorMessage};

// ============================================
// JWT Claims
// ============================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at (timestamp)
    pub iat: usize,
    /// Expiration (timestamp)
    pub exp: usize,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Token type (access/refresh)
    pub token_type: String,
    /// User role
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            TokenType::Access => "access",
            TokenType::Refresh => "refresh",
        }
    }
}

// ============================================
// JWT Configuration — imported from crate::config
// ============================================

// ============================================
// Create Token
// ============================================

/// Create an access token for a user
///
/// # Arguments
/// * `user_id` - UUID of the user
/// * `role` - User role (admin, user, super_admin)
/// * `config` - JWT configuration
///
/// # Returns
/// * `Ok(String)` - JWT token string
/// * `Err(AppError)` - If token creation fails
pub fn create_access_token(
    user_id: Uuid,
    role: &str,
    config: &JwtConfig,
) -> Result<String, AppError> {
    create_token(
        user_id,
        role,
        TokenType::Access,
        config.access_token_maxage,
        config,
    )
}

/// Create a refresh token for a user
///
/// # Arguments
/// * `user_id` - UUID of the user
/// * `role` - User role
/// * `config` - JWT configuration
///
/// # Returns
/// * `Ok(String)` - Refresh token string
pub fn create_refresh_token(
    user_id: Uuid,
    role: &str,
    config: &JwtConfig,
) -> Result<String, AppError> {
    create_token(
        user_id,
        role,
        TokenType::Refresh,
        config.refresh_token_maxage,
        config,
    )
}

/// Core token creation logic
fn create_token(
    user_id: Uuid,
    role: &str,
    token_type: TokenType,
    expires_in_minutes: i64,
    config: &JwtConfig,
) -> Result<String, AppError> {
    if user_id.is_nil() {
        return Err(AppError::bad_request(ErrorMessage::InvalidToken));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_minutes)).timestamp() as usize;

    let claims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
        iss: config.issuer.clone(),
        aud: config.audience.clone(),
        token_type: token_type.as_str().to_string(),
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("Failed to create JWT token: {:?}", e);
        AppError::server_error(ErrorMessage::ServerError)
    })
}

// ============================================
// Decode & Verify Token
// ============================================

/// Decode and verify an access token
///
/// # Arguments
/// * `token` - JWT token string
/// * `config` - JWT configuration
///
/// # Returns
/// * `Ok(TokenClaims)` - Decoded token claims
/// * `Err(AppError)` - If token is invalid or expired
pub fn decode_access_token(token: &str, config: &JwtConfig) -> Result<TokenClaims, AppError> {
    let claims = decode_token(token, config)?;

    // Ensure it's an access token
    if claims.token_type != TokenType::Access.as_str() {
        return Err(AppError::unauthorized(ErrorMessage::InvalidToken));
    }

    Ok(claims)
}

/// Decode and verify a refresh token
pub fn decode_refresh_token(token: &str, config: &JwtConfig) -> Result<TokenClaims, AppError> {
    let claims = decode_token(token, config)?;

    // Ensure it's a refresh token
    if claims.token_type != TokenType::Refresh.as_str() {
        return Err(AppError::unauthorized(ErrorMessage::InvalidToken));
    }

    Ok(claims)
}

/// Core token decoding logic
fn decode_token(token: &str, config: &JwtConfig) -> Result<TokenClaims, AppError> {
    if token.is_empty() {
        return Err(AppError::unauthorized(ErrorMessage::TokenNotProvided));
    }

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[&config.audience]);
    validation.set_issuer(&[&config.issuer]);
    validation.validate_exp = true;

    let decoded = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )
    .map_err(|e| {
        let error_msg = match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => ErrorMessage::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidToken => ErrorMessage::InvalidToken,
            _ => {
                tracing::error!("JWT validation error: {:?}", e);
                ErrorMessage::InvalidToken
            }
        };
        AppError::unauthorized(error_msg)
    })?;

    Ok(decoded.claims)
}

// ============================================
// Extract User ID from Token
// ============================================

/// Extract user ID from decoded claims
pub fn extract_user_id(claims: &TokenClaims) -> Result<Uuid, AppError> {
    Uuid::parse_str(&claims.sub).map_err(|_| AppError::server_error(ErrorMessage::InvalidToken))
}

/// Extract role from decoded claims
pub fn extract_role(claims: &TokenClaims) -> String {
    claims.role.clone()
}

// ============================================
// Token Response Helpers
// ============================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

impl TokenResponse {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        TokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
        }
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> JwtConfig {
        JwtConfig {
            secret: "test-secret-key-for-testing-only".to_string(),
            access_token_maxage: 15,
            refresh_token_maxage: 10080,
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
        }
    }

    #[test]
    fn test_create_and_decode_access_token() {
        let config = get_test_config();
        let user_id = Uuid::new_v4();

        // Create token
        let token = create_access_token(user_id, "user", &config).unwrap();
        assert!(!token.is_empty());

        // Decode token
        let claims = decode_access_token(&token, &config).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.token_type, "access");
        assert_eq!(claims.role, "user");
    }

    #[test]
    fn test_token_type_validation() {
        let config = get_test_config();
        let user_id = Uuid::new_v4();

        // Create refresh token
        let refresh_token = create_refresh_token(user_id, "user", &config).unwrap();

        // Try to decode as access token (should fail)
        let result = decode_access_token(&refresh_token, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_token() {
        let config = get_test_config();
        let result = decode_access_token("invalid-token", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_token() {
        let config = get_test_config();
        let result = decode_access_token("", &config);
        assert!(result.is_err());
    }
}
