use crate::config::EmailConfig;
use crate::error::AppError;

use super::sender::send_email;

// ============================================
// Template Constants
// ============================================

const TEMPLATE_DIR: &str = "src/shared/email/templates";

// Template paths
const VERIFICATION_TEMPLATE: &str = "verification.html";
const WELCOME_TEMPLATE: &str = "welcome.html";
const RESET_PASSWORD_TEMPLATE: &str = "reset_password.html";

// ============================================
// Public Email Functions
// ============================================

/// Send email verification email
pub async fn send_verification_email(
    to_email: &str,
    first_name: &str,
    token: &str,
    config: &EmailConfig,
) -> Result<(), AppError> {
    let subject = "Verify Your Email Address - Atronsa";
    let template_path = format!("{}/{}", TEMPLATE_DIR, VERIFICATION_TEMPLATE);
    let verification_link = build_verification_link(token, config);

    let placeholders = vec![
        ("{{first_name}}".to_string(), first_name.to_string()),
        ("{{verification_link}}".to_string(), verification_link),
        ("{{app_name}}".to_string(), "Atronsa".to_string()),
        ("{{support_email}}".to_string(), config.from_email.clone()),
    ];

    send_email(
        to_email,
        first_name,
        subject,
        &template_path,
        &placeholders,
        config,
    )
    .await
}

/// Send welcome email after successful verification
pub async fn send_welcome_email(
    to_email: &str,
    first_name: &str,
    config: &EmailConfig,
) -> Result<(), AppError> {
    let subject = "Welcome to Atronsa";
    let template_path = format!("{}/{}", TEMPLATE_DIR, WELCOME_TEMPLATE);

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
        &template_path,
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
    let template_path = format!("{}/{}", TEMPLATE_DIR, RESET_PASSWORD_TEMPLATE);
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
        &template_path,
        &placeholders,
        config,
    )
    .await
}

// ============================================
// Helper Functions
// ============================================

fn build_verification_link(token: &str, _config: &EmailConfig) -> String {
    let base_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    format!("{}/api/v1/auth/verify-email?token={}", base_url, token)
}

fn build_reset_link(token: &str, _config: &EmailConfig) -> String {
    let base_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    format!("{}/reset-password?token={}", base_url, token)
}
