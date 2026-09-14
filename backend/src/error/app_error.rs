use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(status: &str, message: &str) -> Self {
        ErrorResponse {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorMessage {
    // Validation errors
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidEmailFormat,
    InvalidPhoneFormat,
    WeakPassword,

    // Authentication errors
    WrongCredentials,
    InvalidToken,
    TokenNotProvided,
    UserNotAuthenticated,
    PermissionDenied,

    // User errors
    UserNotFound,
    UserAlreadyExists,
    EmailAlreadyExists,
    PhoneAlreadyExists,
    UserNoLongerExist,
    NoToken,
    AccountNotVerified,
    AccountInactive,
    AccountSuspended,
    AccountBanned,

    // Wallet errors
    WalletNotFound,

    // Database errors
    DatabaseError,
    UniqueConstraintViolation,
    ForeignKeyViolation,

    // Password errors
    HashingError,
    InvalidHashFormat,
    PasswordMismatch,
    IncorrectCurrentPassword,
    InvalidPasswordFormat,
    SamePassword,

    // Token/OTP errors
    InvalidOrExpiredToken,
    TokenExpired,
    EmailAlreadyVerified,
    OtpNotRequested,
    OtpExpired,
    InvalidOtp(i64),
    OtpResendCooldown(i64),
    OtpVerificationLocked(i64),
    PhoneAlreadyVerified,
    InvalidPhoneOtp,
    PhoneOtpSendLimitReached(i64),

    // Server errors
    ServerError,
    InternalServerError,
    ServiceUnavailable,

    // Input errors
    BadRequest,
    InvalidInput,
    MissingRequiredField(String),

    // Rate limiting
    TooManyRequests,

    // Email errors
    EmailSendError,
    EmailConfigurationError,
}

impl ErrorMessage {
    pub fn to_str(&self) -> String {
        match self {
            // Validation
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max) => {
                format!("Password must not be more than {} characters", max)
            }
            ErrorMessage::InvalidEmailFormat => "Invalid email format".to_string(),
            ErrorMessage::InvalidPhoneFormat => {
                "Invalid phone number format. Use an Ethiopian mobile number".to_string()
            }
            ErrorMessage::WeakPassword => {
                "Password must be at least 8 characters and include an uppercase letter, a lowercase letter, a number, and a special character".to_string()
            }

            // Authentication
            ErrorMessage::WrongCredentials => "Email or password is incorrect".to_string(),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => "Authentication token is required".to_string(),
            ErrorMessage::UserNotAuthenticated => {
                "Please log in to access this resource".to_string()
            }
            ErrorMessage::PermissionDenied => {
                "You do not have permission to perform this action".to_string()
            }

            // User
            ErrorMessage::UserNotFound => "User not found".to_string(),
            ErrorMessage::UserAlreadyExists => "User already exists".to_string(),
            ErrorMessage::EmailAlreadyExists => "A user with this email already exists".to_string(),
            ErrorMessage::PhoneAlreadyExists => {
                "A user with this phone number already exists".to_string()
            }
            ErrorMessage::UserNoLongerExist => {
                "User belonging to this token no longer exists".to_string()
            }
            ErrorMessage::NoToken => {
                "You aren't logged in yet, to logout you must login first".to_string()
            }
            ErrorMessage::AccountNotVerified => {
                "Your account is not verified contact support team".to_string()
            }
            ErrorMessage::AccountInactive => {
                "Your account is not active contact support team".to_string()
            }
            ErrorMessage::AccountSuspended => {
                "Your account has been suspended. Contact support".to_string()
            }
            ErrorMessage::AccountBanned => "Your account has been banned".to_string(),

            // Wallet
            ErrorMessage::WalletNotFound => "Wallet not found".to_string(),

            // Database
            ErrorMessage::DatabaseError => "A database error occurred".to_string(),
            ErrorMessage::UniqueConstraintViolation => {
                "A record with this value already exists".to_string()
            }
            ErrorMessage::ForeignKeyViolation => "Referenced record does not exist".to_string(),

            // Password
            ErrorMessage::HashingError => "Error while processing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::PasswordMismatch => "Passwords do not match".to_string(),
            ErrorMessage::IncorrectCurrentPassword => {
                "Current password is incorrect".to_string()
            }
            ErrorMessage::InvalidPasswordFormat => {
                "Password must be at least 6 characters".to_string()
            }
            ErrorMessage::SamePassword => {
                "New password must be different from current password".to_string()
            }

            // Token/OTP
            ErrorMessage::InvalidOrExpiredToken => "Token is invalid or has expired".to_string(),
            ErrorMessage::TokenExpired => "Token has expired. Please request a new one".to_string(),
            ErrorMessage::EmailAlreadyVerified => "Your email is already verified".to_string(),
            ErrorMessage::OtpNotRequested => {
                "No verification code was requested. Please request one first".to_string()
            }
            ErrorMessage::OtpExpired => {
                "Verification code has expired. Please request a new one".to_string()
            }
            ErrorMessage::InvalidOtp(remaining) => format!(
                "Invalid verification code. {} attempt(s) remaining before you're locked out for 1 hour",
                remaining
            ),
            ErrorMessage::OtpResendCooldown(seconds) => format!(
                "Please wait {} seconds before requesting a new code",
                seconds
            ),
            ErrorMessage::OtpVerificationLocked(minutes) => format!(
                "Too many failed attempts. Please try again in {} minute(s)",
                minutes
            ),
            ErrorMessage::PhoneAlreadyVerified => {
                "Your phone number is already verified".to_string()
            }
            ErrorMessage::InvalidPhoneOtp => "Invalid verification code".to_string(),
            ErrorMessage::PhoneOtpSendLimitReached(minutes) => format!(
                "Maximum verification codes sent. Please try again in {} minute(s)",
                minutes
            ),

            // Server
            ErrorMessage::ServerError => "Server error. Please try again later".to_string(),
            ErrorMessage::InternalServerError => "Internal server error occurred".to_string(),
            ErrorMessage::ServiceUnavailable => "Service temporarily unavailable".to_string(),

            // Input
            ErrorMessage::BadRequest => "Bad request".to_string(),
            ErrorMessage::InvalidInput => "Invalid input provided".to_string(),
            ErrorMessage::MissingRequiredField(field) => format!("{} is required", field),

            // Rate limiting
            ErrorMessage::TooManyRequests => {
                "Too many requests. Please try again later".to_string()
            }

            // Email
            ErrorMessage::EmailSendError => "Failed to send email".to_string(),
            ErrorMessage::EmailConfigurationError => {
                "Email service is not configured properly".to_string()
            }
        }
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub message: String,
    pub status: StatusCode,
    pub error_type: ErrorMessage,
}

impl AppError {
    pub fn new(message: impl Into<String>, status: StatusCode, error_type: ErrorMessage) -> Self {
        AppError {
            message: message.into(),
            status,
            error_type,
        }
    }

    pub fn bad_request(error_type: ErrorMessage) -> Self {
        AppError {
            message: error_type.to_str(),
            status: StatusCode::BAD_REQUEST,
            error_type,
        }
    }

    pub fn unauthorized(error_type: ErrorMessage) -> Self {
        AppError {
            message: error_type.to_str(),
            status: StatusCode::UNAUTHORIZED,
            error_type,
        }
    }

    pub fn forbidden(error_type: ErrorMessage) -> Self {
        AppError {
            message: error_type.to_str(),
            status: StatusCode::FORBIDDEN,
            error_type,
        }
    }

    pub fn not_found(error_type: ErrorMessage) -> Self {
        AppError {
            message: error_type.to_str(),
            status: StatusCode::NOT_FOUND,
            error_type,
        }
    }

    pub fn conflict(error_type: ErrorMessage) -> Self {
        AppError {
            message: error_type.to_str(),
            status: StatusCode::CONFLICT,
            error_type,
        }
    }

    pub fn server_error(error_type: ErrorMessage) -> Self {
        AppError {
            message: error_type.to_str(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error_type,
        }
    }

    pub fn too_many_requests(error_type: ErrorMessage) -> Self {
        AppError {
            message: error_type.to_str(),
            status: StatusCode::TOO_MANY_REQUESTS,
            error_type,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppError: type={:?}, message={}, status={}",
            self.error_type, self.message, self.status
        )
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::not_found(ErrorMessage::UserNotFound),
            sqlx::Error::Database(db_error) => {
                if let Some(code) = db_error.code() {
                    match code.as_ref() {
                        "23505" => AppError::conflict(ErrorMessage::UniqueConstraintViolation),
                        "23503" => AppError::bad_request(ErrorMessage::ForeignKeyViolation),
                        _ => AppError::server_error(ErrorMessage::DatabaseError),
                    }
                } else {
                    AppError::server_error(ErrorMessage::DatabaseError)
                }
            }
            _ => AppError::server_error(ErrorMessage::DatabaseError),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let message = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let messages: Vec<String> = errors
                    .iter()
                    .map(|e| e.message.as_ref().unwrap_or(&"invalid".into()).to_string())
                    .collect();
                format!("{}: {}", field, messages.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");

        AppError::new(message, StatusCode::BAD_REQUEST, ErrorMessage::InvalidInput)
    }
}
