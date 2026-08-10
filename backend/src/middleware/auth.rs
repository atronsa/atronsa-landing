use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    database::UserExt,
    error::{AppError, ErrorMessage},
    modules::users::model::{User, UserRole},
    shared::token as jwt,
};

// ============================================
// JWT Auth Middleware Extension
// ============================================

/// Stores authenticated user info in request extensions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub user: User,
    pub user_id: Uuid,
    pub role: UserRole,
}

impl JWTAuthMiddleware {
    pub fn new(user: User) -> Self {
        JWTAuthMiddleware {
            user_id: user.id,
            role: user.role.clone(),
            user,
        }
    }
}

// ============================================
// Main Authentication Middleware
// ============================================

/// Middleware that validates JWT token from cookie or Authorization header
pub async fn auth_middleware(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    // Extract token from cookie or Authorization header
    let token = extract_token(&cookie_jar, &req)?;

    // Decode and verify JWT token
    let claims = jwt::decode_access_token(&token, &state.config.jwt)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    // Extract user ID from claims
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    // Fetch user from database
    let user = state
        .db_client
        .get_user(Some(user_id), None, None)
        .await
        .map_err(|_| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?
        .ok_or_else(|| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?;

    // Check if account is active
    if !user.is_active() {
        return Err(AppError::forbidden(ErrorMessage::AccountSuspended));
    }

    // fix for upper code
    //     match user.status {
    //     UserStatus::Inactive => {
    //         return Err(AppError::forbidden(ErrorMessage::AccountNotVerified));
    //     }
    //     UserStatus::Suspended => {
    //         return Err(AppError::forbidden(ErrorMessage::AccountSuspended));
    //     }
    //     UserStatus::Banned => {
    //         return Err(AppError::forbidden(ErrorMessage::AccountBanned));
    //     }
    //     UserStatus::Active => {} // allowed
    // }

    // Store authenticated user in request extensions
    let auth_user = JWTAuthMiddleware::new(user);
    req.extensions_mut().insert(auth_user);

    // Continue to next middleware/handler
    Ok(next.run(req).await)
}

// ============================================
// Optional Authentication Middleware
// ============================================

/// Middleware that optionally authenticates (doesn't fail if no token)
pub async fn optional_auth_middleware(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    // Try to extract token
    if let Ok(token) = extract_token(&cookie_jar, &req) {
        // Try to decode token
        if let Ok(claims) = jwt::decode_access_token(&token, &state.config.jwt) {
            // Try to parse user ID
            if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                // Try to fetch user
                if let Ok(Some(user)) = state.db_client.get_user(Some(user_id), None, None).await {
                    // Store user if found
                    let auth_user = JWTAuthMiddleware::new(user);
                    req.extensions_mut().insert(auth_user);
                }
            }
        }
    }

    // Always continue, even without auth
    Ok(next.run(req).await)
}

// ============================================
// Role-Based Authorization Middleware Factory
// ============================================

/// Creates a middleware that checks for specific roles
pub fn require_roles(
    required_roles: Vec<UserRole>,
) -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<axum::response::Response, AppError>> + Send>,
> {
    move |req: Request, next: Next| {
        let required_roles = required_roles.clone();
        Box::pin(async move {
            role_check_middleware(req, next, &required_roles)
                .await
                .map(IntoResponse::into_response)
        })
    }
}

/// Middleware that checks if user has required roles
pub async fn role_check_middleware(
    req: Request,
    next: Next,
    required_roles: &[UserRole],
) -> Result<impl IntoResponse, AppError> {
    // Get authenticated user from request extensions
    let auth_user = req
        .extensions()
        .get::<JWTAuthMiddleware>()
        .ok_or_else(|| AppError::unauthorized(ErrorMessage::UserNotAuthenticated))?;

    // Check if user has required role
    if !required_roles.contains(&auth_user.role) {
        return Err(AppError::forbidden(ErrorMessage::PermissionDenied));
    }

    // Continue to handler
    Ok(next.run(req).await)
}

// ============================================
// Specific Role Middlewares (Convenience)
// ============================================

/// Middleware that requires Admin or SuperAdmin role
pub async fn require_admin(req: Request, next: Next) -> Result<impl IntoResponse, AppError> {
    role_check_middleware(req, next, &[UserRole::Admin, UserRole::SuperAdmin]).await
}

/// Middleware that requires SuperAdmin role only
pub async fn require_super_admin(req: Request, next: Next) -> Result<impl IntoResponse, AppError> {
    role_check_middleware(req, next, &[UserRole::SuperAdmin]).await
}

// ============================================
// Token Extraction Helper
// ============================================

/// Extract JWT token from cookie or Authorization header
fn extract_token(cookie_jar: &CookieJar, req: &Request) -> Result<String, AppError> {
    // First, try to get token from cookie
    if let Some(cookie) = cookie_jar.get("atronsa_access_token") {
        let token = cookie.value().to_string();
        if !token.is_empty() {
            return Ok(token);
        }
    }

    // Then, try to get token from Authorization header
    if let Some(auth_header) = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if !token.is_empty() {
                return Ok(token.to_string());
            }
        }
    }

    // No token found
    Err(AppError::unauthorized(ErrorMessage::TokenNotProvided))
}

// ============================================
// Extractor for Handlers
// ============================================

/// Extractor to get authenticated user in handlers
pub async fn get_auth_user(req: &Request) -> Result<&JWTAuthMiddleware, AppError> {
    req.extensions()
        .get::<JWTAuthMiddleware>()
        .ok_or_else(|| AppError::unauthorized(ErrorMessage::UserNotAuthenticated))
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_token_from_header() {
        // Test implementation here
    }
}
