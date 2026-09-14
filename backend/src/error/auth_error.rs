use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

use super::app_error::ErrorResponse;

#[derive(Debug, Clone, PartialEq)]
pub enum AuthError {
    WrongCredentials,
    InvalidToken,
    TokenExpired,
    TokenNotProvided,
    UserNotAuthenticated,
    PermissionDenied,
    AccountNotVerified,
    AccountSuspended,
    AccountBanned,
    PasswordMismatch,
    InvalidPasswordFormat,
    UserNotFound,
    EmailAlreadyExists,
    InvalidEmailFormat,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
            AuthError::TokenNotProvided => StatusCode::UNAUTHORIZED,
            AuthError::UserNotAuthenticated => StatusCode::UNAUTHORIZED,
            AuthError::PermissionDenied => StatusCode::FORBIDDEN,
            AuthError::AccountNotVerified => StatusCode::FORBIDDEN,
            AuthError::AccountSuspended => StatusCode::FORBIDDEN,
            AuthError::AccountBanned => StatusCode::FORBIDDEN,
            AuthError::PasswordMismatch => StatusCode::BAD_REQUEST,
            AuthError::InvalidPasswordFormat => StatusCode::BAD_REQUEST,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::EmailAlreadyExists => StatusCode::CONFLICT,
            AuthError::InvalidEmailFormat => StatusCode::BAD_REQUEST,
        }
    }

    pub fn message(&self) -> String {
        match self {
            AuthError::WrongCredentials => "Email or password is incorrect".to_string(),
            AuthError::InvalidToken => "Authentication token is invalid or expired".to_string(),
            AuthError::TokenExpired => "Token has expired. Please login again".to_string(),
            AuthError::TokenNotProvided => "Authentication token is required".to_string(),
            AuthError::UserNotAuthenticated => "Please log in to access this resource".to_string(),
            AuthError::PermissionDenied => {
                "You do not have permission to perform this action".to_string()
            }
            AuthError::AccountNotVerified => {
                "Please verify your email before logging in".to_string()
            }
            AuthError::AccountSuspended => {
                "Your account has been suspended. Contact support".to_string()
            }
            AuthError::AccountBanned => "Your account has been banned".to_string(),
            AuthError::PasswordMismatch => "Passwords do not match".to_string(),
            AuthError::InvalidPasswordFormat => {
                "Password must be at least 6 characters".to_string()
            }
            AuthError::UserNotFound => "User not found".to_string(),
            AuthError::EmailAlreadyExists => "A user with this email already exists".to_string(),
            AuthError::InvalidEmailFormat => "Invalid email format".to_string(),
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AuthError {}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(ErrorResponse::new("fail", &self.message()));
        (status, body).into_response()
    }
}
