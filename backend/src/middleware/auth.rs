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
    database::{AdminExt, SuperAdminExt, UserExt},
    error::{AppError, ErrorMessage},
    modules::super_admin::model::SuperAdmin,
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
            role: user.role,
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
    let token = extract_token(&cookie_jar, &req, ACCESS_COOKIE_NAME)?;

    let claims = jwt::decode_access_token(&token, &state.config.jwt)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    let user = state
        .db_client
        .get_user(Some(user_id), None)
        .await
        .map_err(|_| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?
        .ok_or_else(|| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?;

    // Only moderation states (Suspended/Banned) block access. `Inactive` is
    // the registration default (pending future email verification) and must
    // not lock users out.
    if user.is_blocked() {
        let message = match user.status {
            crate::modules::users::model::UserStatus::Banned => ErrorMessage::AccountBanned,
            _ => ErrorMessage::AccountSuspended,
        };
        return Err(AppError::forbidden(message));
    }

    let auth_user = JWTAuthMiddleware::new(user);
    req.extensions_mut().insert(auth_user);

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
    if let Ok(token) = extract_token(&cookie_jar, &req, ACCESS_COOKIE_NAME) {
        if let Ok(claims) = jwt::decode_access_token(&token, &state.config.jwt) {
            if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                if let Ok(Some(user)) = state.db_client.get_user(Some(user_id), None).await {
                    if !user.is_blocked() {
                        let auth_user = JWTAuthMiddleware::new(user);
                        req.extensions_mut().insert(auth_user);
                    }
                }
            }
        }
    }

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
    let auth_user = req
        .extensions()
        .get::<JWTAuthMiddleware>()
        .ok_or_else(|| AppError::unauthorized(ErrorMessage::UserNotAuthenticated))?;

    if !required_roles.contains(&auth_user.role) {
        return Err(AppError::forbidden(ErrorMessage::PermissionDenied));
    }

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
// Super Admin Authentication Middleware
// ============================================
//
// Separate from `auth_middleware` because super admins live in their own
// table (`super_admins`), not `users` — `auth_middleware` resolves a JWT's
// `sub` against `users` only, and would 404 on every super-admin token.
// This is the "super-admin-only route/middleware" CLAUDE.md's "Super admin"
// section calls out as a natural follow-up once one is actually needed —
// it's needed now, to gate `POST /api/v1/super-admin/admins`.

const SUPER_ADMIN_ACCESS_COOKIE_NAME: &str = "atronsa_super_admin_access_token";

/// Stores the authenticated super admin in request extensions, mirroring
/// `JWTAuthMiddleware`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuperAdminAuthMiddleware {
    pub super_admin: SuperAdmin,
    pub super_admin_id: Uuid,
}

impl SuperAdminAuthMiddleware {
    pub fn new(super_admin: SuperAdmin) -> Self {
        SuperAdminAuthMiddleware {
            super_admin_id: super_admin.super_admin_id,
            super_admin,
        }
    }
}

/// Middleware that validates a super-admin JWT (cookie or Bearer header)
/// and re-resolves it against a live `super_admins` row on every request.
pub async fn super_admin_auth_middleware(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = extract_token(&cookie_jar, &req, SUPER_ADMIN_ACCESS_COOKIE_NAME)?;

    let claims = jwt::decode_access_token(&token, &state.config.jwt)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    if claims.role != "super_admin" {
        return Err(AppError::unauthorized(ErrorMessage::InvalidToken));
    }

    let super_admin_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    let super_admin = state
        .db_client
        .get_super_admin_by_id(super_admin_id)
        .await
        .map_err(|_| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?
        .ok_or_else(|| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?;

    if super_admin.is_blocked() {
        let message = match super_admin.status {
            crate::modules::users::model::UserStatus::Banned => ErrorMessage::AccountBanned,
            _ => ErrorMessage::AccountSuspended,
        };
        return Err(AppError::forbidden(message));
    }

    req.extensions_mut()
        .insert(SuperAdminAuthMiddleware::new(super_admin));

    Ok(next.run(req).await)
}

// ============================================
// Admin (staff) Authentication Middleware
// ============================================
//
// Separate from both `auth_middleware` and `super_admin_auth_middleware`
// because admins live in their own table (`admins`), not `users` or
// `super_admins` — resolving an admin JWT's `sub` against either of those
// would 404 on every admin token. Gates the `/api/v1/admin` routes that
// manage `User` accounts (list, get one, ban/suspend/reactivate, change
// status) — see CLAUDE.md's "Admins (staff accounts)" section.

const ADMIN_ACCESS_COOKIE_NAME: &str = "atronsa_admin_access_token";

/// Stores the authenticated admin in request extensions, mirroring
/// `SuperAdminAuthMiddleware`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminAuthMiddleware {
    pub admin: crate::modules::admin::model::Admin,
    pub admin_id: Uuid,
}

impl AdminAuthMiddleware {
    pub fn new(admin: crate::modules::admin::model::Admin) -> Self {
        AdminAuthMiddleware {
            admin_id: admin.admin_id,
            admin,
        }
    }
}

/// Middleware that validates an admin JWT (cookie or Bearer header) and
/// re-resolves it against a live `admins` row on every request.
pub async fn admin_auth_middleware(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let token = extract_token(&cookie_jar, &req, ADMIN_ACCESS_COOKIE_NAME)?;

    let claims = jwt::decode_access_token(&token, &state.config.jwt)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    if claims.role != "admin" {
        return Err(AppError::unauthorized(ErrorMessage::InvalidToken));
    }

    let admin_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::unauthorized(ErrorMessage::InvalidToken))?;

    let admin = state
        .db_client
        .get_admin_by_id(admin_id)
        .await
        .map_err(|_| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?
        .ok_or_else(|| AppError::unauthorized(ErrorMessage::UserNoLongerExist))?;

    if admin.is_blocked() {
        let message = match admin.status {
            crate::modules::users::model::UserStatus::Banned => ErrorMessage::AccountBanned,
            _ => ErrorMessage::AccountSuspended,
        };
        return Err(AppError::forbidden(message));
    }

    req.extensions_mut().insert(AdminAuthMiddleware::new(admin));

    Ok(next.run(req).await)
}

// ============================================
// Token Extraction Helper
// ============================================

const ACCESS_COOKIE_NAME: &str = "atronsa_access_token";

/// Extract JWT token from cookie or Authorization header. `cookie_name`
/// lets callers look for a different cookie per identity type (regular
/// user vs. super admin), since each is issued under its own cookie name.
fn extract_token(
    cookie_jar: &CookieJar,
    req: &Request,
    cookie_name: &str,
) -> Result<String, AppError> {
    if let Some(cookie) = cookie_jar.get(cookie_name) {
        let token = cookie.value().to_string();
        if !token.is_empty() {
            return Ok(token);
        }
    }

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
