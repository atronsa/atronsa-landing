pub mod app_error;
pub mod auth_error;
pub mod payment_error;

pub use app_error::{AppError, ErrorMessage};

pub type Result<T> = std::result::Result<T, AppError>;
