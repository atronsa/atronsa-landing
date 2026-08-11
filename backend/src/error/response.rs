use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

impl ErrorResponse {
    pub fn new(status: impl Into<String>, message: impl Into<String>) -> Self {
        ErrorResponse {
            status: status.into(),
            message: message.into(),
            code: None,
            details: None,
        }
    }
    
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    
    pub fn with_details(mut self, details: Vec<String>) -> Self {
        self.details = Some(details);
        self
    }
}

// error_kind.rs - Single source of truth for error variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    // Authentication (401)
    WrongCredentials,
    InvalidToken,
    TokenExpired,
    TokenNotProvided,
    UserNotAuthenticated,
    
    // Authorization (403)
    PermissionDenied,
    AccountNotVerified,
    AccountSuspended,
    AccountBanned,
    
    // Validation (400)
    InvalidInput,
    InvalidEmailFormat,
    InvalidPasswordFormat,
    PasswordMismatch,
    MissingRequiredField,
    EmptyPassword,
    
    // Resource (404)
    UserNotFound,
    
    // Conflict (409)
    EmailAlreadyExists,
    PhoneAlreadyExists,
    UserAlreadyExists,
    
    // Server (500)
    InternalError,
    DatabaseError,
    HashingError,
    EmailSendError,
    
    // Rate Limiting (429)
    TooManyRequests,
}

impl ErrorKind {
    pub fn status_code(&self) -> StatusCode {
        match self {
            // 401
            Self::WrongCredentials | Self::InvalidToken | Self::TokenExpired |
            Self::TokenNotProvided | Self::UserNotAuthenticated => StatusCode::UNAUTHORIZED,
            
            // 403
            Self::PermissionDenied | Self::AccountNotVerified | 
            Self::AccountSuspended | Self::AccountBanned => StatusCode::FORBIDDEN,
            
            // 400
            Self::InvalidInput | Self::InvalidEmailFormat | Self::InvalidPasswordFormat |
            Self::PasswordMismatch | Self::MissingRequiredField | Self::EmptyPassword => {
                StatusCode::BAD_REQUEST
            }
            
            // 404
            Self::UserNotFound => StatusCode::NOT_FOUND,
            
            // 409
            Self::EmailAlreadyExists | Self::PhoneAlreadyExists | 
            Self::UserAlreadyExists => StatusCode::CONFLICT,
            
            // 500
            Self::InternalError | Self::DatabaseError | Self::HashingError |
            Self::EmailSendError => StatusCode::INTERNAL_SERVER_ERROR,
            
            // 429
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }
    
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::WrongCredentials => "AUTH_001",
            Self::InvalidToken => "AUTH_002",
            Self::TokenExpired => "AUTH_003",
            Self::TokenNotProvided => "AUTH_004",
            Self::UserNotAuthenticated => "AUTH_005",
            Self::PermissionDenied => "AUTH_006",
            Self::AccountNotVerified => "AUTH_007",
            Self::AccountSuspended => "AUTH_008",
            Self::AccountBanned => "AUTH_009",
            Self::InvalidInput => "VAL_001",
            Self::InvalidEmailFormat => "VAL_002",
            Self::InvalidPasswordFormat => "VAL_003",
            Self::PasswordMismatch => "VAL_004",
            Self::MissingRequiredField => "VAL_005",
            Self::EmptyPassword => "VAL_006",
            Self::UserNotFound => "RES_001",
            Self::EmailAlreadyExists => "CON_001",
            Self::PhoneAlreadyExists => "CON_002",
            Self::UserAlreadyExists => "CON_003",
            Self::InternalError => "SRV_001",
            Self::DatabaseError => "SRV_002",
            Self::HashingError => "SRV_003",
            Self::EmailSendError => "SRV_004",
            Self::TooManyRequests => "RAT_001",
        }
    }
    
    pub fn user_message(&self) -> &'static str {
        match self {
            Self::WrongCredentials => "Email or password is incorrect",
            Self::InvalidToken => "Authentication token is invalid",
            Self::TokenExpired => "Token has expired. Please login again",
            Self::TokenNotProvided => "Authentication token is required",
            Self::UserNotAuthenticated => "Please log in to access this resource",
            Self::PermissionDenied => "You do not have permission to perform this action",
            Self::AccountNotVerified => "Please verify your email before logging in",
            Self::AccountSuspended => "Your account has been suspended. Contact support",
            Self::AccountBanned => "Your account has been banned",
            Self::InvalidInput => "Invalid input provided",
            Self::InvalidEmailFormat => "Invalid email format",
            Self::InvalidPasswordFormat => "Password does not meet requirements",
            Self::PasswordMismatch => "Passwords do not match",
            Self::MissingRequiredField => "Required field is missing",
            Self::EmptyPassword => "Password cannot be empty",
            Self::UserNotFound => "Resource not found",
            Self::EmailAlreadyExists => "This email is already registered",
            Self::PhoneAlreadyExists => "This phone number is already registered",
            Self::UserAlreadyExists => "User already exists",
            Self::InternalError => "An unexpected error occurred",
            Self::DatabaseError => "A database error occurred",
            Self::HashingError => "Error processing request",
            Self::EmailSendError => "Failed to send email",
            Self::TooManyRequests => "Too many requests. Please try again later",
        }
    }
}

// app_error.rs - Clean, single error type
#[derive(Debug, Clone)]
pub struct AppError {
    kind: ErrorKind,
    message: Option<String>,
    details: Option<Vec<String>>,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl AppError {
    pub fn new(kind: ErrorKind) -> Self {
        AppError {
            kind,
            message: None,
            details: None,
            source: None,
        }
    }
    
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    
    pub fn with_details(mut self, details: Vec<String>) -> Self {
        self.details = Some(details);
        self
    }
    
    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
    
    // Convenience constructors
    pub fn unauthorized(kind: ErrorKind) -> Self {
        debug_assert_eq!(kind.status_code(), StatusCode::UNAUTHORIZED);
        Self::new(kind)
    }
    
    pub fn forbidden(kind: ErrorKind) -> Self {
        debug_assert_eq!(kind.status_code(), StatusCode::FORBIDDEN);
        Self::new(kind)
    }
    
    pub fn bad_request(kind: ErrorKind) -> Self {
        debug_assert_eq!(kind.status_code(), StatusCode::BAD_REQUEST);
        Self::new(kind)
    }
    
    pub fn not_found(kind: ErrorKind) -> Self {
        debug_assert_eq!(kind.status_code(), StatusCode::NOT_FOUND);
        Self::new(kind)
    }
    
    // Accessors
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    
    pub fn status_code(&self) -> StatusCode {
        self.kind.status_code()
    }
    
    pub fn error_code(&self) -> &'static str {
        self.kind.error_code()
    }
    
    pub fn user_message(&self) -> String {
        self.message.clone().unwrap_or_else(|| self.kind.user_message().to_string())
    }
    
    pub fn internal_message(&self) -> String {
        format!(
            "AppError[kind={:?}, code={}, message={}, details={:?}]",
            self.kind,
            self.error_code(),
            self.user_message(),
            self.details
        )
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.internal_message())
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &dyn std::error::Error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        
        // Log internal details server-side
        tracing::error!(
            error_code = %self.error_code(),
            error_kind = ?self.kind,
            details = ?self.details,
            source = ?self.source,
            "{}", self.user_message()
        );
        
        let mut response = ErrorResponse::new("fail", self.user_message())
            .with_code(self.error_code());
        
        // Only include details in non-production environments
        if cfg!(debug_assertions) {
            if let Some(details) = self.details {
                response = response.with_details(details);
            }
        }
        
        (status, Json(response)).into_response()
    }
}

// Database error conversion
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        let app_error = match &error {
            sqlx::Error::RowNotFound => {
                AppError::not_found(ErrorKind::UserNotFound)
            }
            sqlx::Error::Database(db_error) => {
                match db_error.code().as_deref() {
                    Some("23505") => AppError::bad_request(ErrorKind::EmailAlreadyExists)
                        .with_message("A record with this value already exists"),
                    Some("23503") => AppError::bad_request(ErrorKind::InvalidInput)
                        .with_message("Referenced record does not exist"),
                    _ => AppError::new(ErrorKind::DatabaseError),
                }
            }
            _ => AppError::new(ErrorKind::DatabaseError),
        };
        
        app_error
            .with_source(error)
            .with_details(vec![format!("Database error: {:?}", error)])
    }
}

// Validation error conversion
impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let details: Vec<String> = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let messages: Vec<String> = errors
                    .iter()
                    .map(|e| e.message.as_ref().unwrap_or(&"invalid value".into()).to_string())
                    .collect();
                format!("{}: {}", field, messages.join(", "))
            })
            .collect();
        
        AppError::bad_request(ErrorKind::InvalidInput)
            .with_message("Validation failed")
            .with_details(details)
    }
}