pub mod sender;
pub mod templates;

pub use templates::{send_password_reset_email, send_verification_otp_email, send_welcome_email};
