use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

use super::app_error::ErrorResponse;

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentError {
    PaymentFailed,
    InsufficientFunds,
    InvalidPaymentMethod,
    PaymentExpired,
    PaymentNotFound,
    RefundFailed,
    AmountMismatch,
    CurrencyNotSupported,
    ProviderError(String),
}

impl PaymentError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            PaymentError::PaymentFailed => StatusCode::PAYMENT_REQUIRED,
            PaymentError::InsufficientFunds => StatusCode::PAYMENT_REQUIRED,
            PaymentError::InvalidPaymentMethod => StatusCode::BAD_REQUEST,
            PaymentError::PaymentExpired => StatusCode::GONE,
            PaymentError::PaymentNotFound => StatusCode::NOT_FOUND,
            PaymentError::RefundFailed => StatusCode::INTERNAL_SERVER_ERROR,
            PaymentError::AmountMismatch => StatusCode::BAD_REQUEST,
            PaymentError::CurrencyNotSupported => StatusCode::BAD_REQUEST,
            PaymentError::ProviderError(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    pub fn message(&self) -> String {
        match self {
            PaymentError::PaymentFailed => "Payment processing failed".to_string(),
            PaymentError::InsufficientFunds => "Insufficient funds".to_string(),
            PaymentError::InvalidPaymentMethod => "Invalid payment method".to_string(),
            PaymentError::PaymentExpired => "Payment has expired".to_string(),
            PaymentError::PaymentNotFound => "Payment not found".to_string(),
            PaymentError::RefundFailed => "Refund processing failed".to_string(),
            PaymentError::AmountMismatch => "Payment amount mismatch".to_string(),
            PaymentError::CurrencyNotSupported => "Currency not supported".to_string(),
            PaymentError::ProviderError(msg) => format!("Payment provider error: {}", msg),
        }
    }
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for PaymentError {}

impl IntoResponse for PaymentError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(ErrorResponse::new("fail", &self.message()));
        (status, body).into_response()
    }
}
