use crate::config::EmailConfig;
use crate::error::AppError;

use super::sender::send_email;

// ============================================
// Template Constants
// ============================================

// Baked into the binary rather than read from `src/shared/email/templates` at
// runtime: that path is relative to the working directory, so a deployed binary
// started from anywhere else failed every send before it reached SMTP.
const WELCOME_TEMPLATE: &str = include_str!("templates/welcome.html");
const RESET_PASSWORD_TEMPLATE: &str = include_str!("templates/reset_password.html");
const VERIFY_EMAIL_TEMPLATE: &str = include_str!("templates/verify_email.html");

// ============================================
// Public Email Functions
// ============================================

/// Send welcome email after successful registration
pub async fn send_welcome_email(
    to_email: &str,
    first_name: &str,
    config: &EmailConfig,
) -> Result<(), AppError> {
    let subject = "Welcome to Atronsa";

    let placeholders = vec![
        ("{{first_name}}".to_string(), first_name.to_string()),
        ("{{app_name}}".to_string(), "Atronsa".to_string()),
        (
            "{{login_url}}".to_string(),
            "https://app.atronsa.com/login".to_string(),
        ),
        ("{{support_email}}".to_string(), config.from_email.clone()),
    ];

    send_email(
        to_email,
        first_name,
        subject,
        WELCOME_TEMPLATE,
        &placeholders,
        config,
    )
    .await
}

/// Send password reset email
pub async fn send_password_reset_email(
    to_email: &str,
    first_name: &str,
    reset_token: &str,
    config: &EmailConfig,
) -> Result<(), AppError> {
    let subject = "Reset Your Password - Atronsa";
    let reset_link = build_reset_link(reset_token, config);

    let placeholders = vec![
        ("{{first_name}}".to_string(), first_name.to_string()),
        ("{{reset_link}}".to_string(), reset_link),
        ("{{app_name}}".to_string(), "Atronsa".to_string()),
        ("{{support_email}}".to_string(), config.from_email.clone()),
        ("{{expiry_minutes}}".to_string(), "30".to_string()),
    ];

    send_email(
        to_email,
        first_name,
        subject,
        RESET_PASSWORD_TEMPLATE,
        &placeholders,
        config,
    )
    .await
}

/// Send an email-verification OTP. `otp` is the plaintext 6-digit code —
/// only its Argon2 hash is ever persisted (see `shared::otp`).
pub async fn send_verification_otp_email(
    to_email: &str,
    first_name: &str,
    otp: &str,
    config: &EmailConfig,
) -> Result<(), AppError> {
    let subject = "Verify Your Email - Atronsa";

    let placeholders = vec![
        ("{{first_name}}".to_string(), first_name.to_string()),
        ("{{otp-code}}".to_string(), otp.to_string()),
        ("{{support_email}}".to_string(), config.from_email.clone()),
    ];

    send_email(
        to_email,
        first_name,
        subject,
        VERIFY_EMAIL_TEMPLATE,
        &placeholders,
        config,
    )
    .await
}

// ============================================
// Helper Functions
// ============================================

fn build_reset_link(token: &str, _config: &EmailConfig) -> String {
    let base_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    format!("{}/reset-password?token={}", base_url, token)
}
