use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, SinglePart, header},
    transport::smtp::authentication::Credentials,
};
use std::fs;
use tracing::{error, info};

use crate::config::EmailConfig;
use crate::error::{AppError, ErrorMessage};

// ============================================
// Email Sender
// ============================================

/// Core email sending function
pub async fn send_email(
    to_email: &str,
    to_name: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)],
    config: &EmailConfig,
) -> Result<(), AppError> {
    // Read HTML template
    let mut html_template = fs::read_to_string(template_path).map_err(|e| {
        error!("Failed to read email template {}: {}", template_path, e);
        AppError::server_error(ErrorMessage::EmailSendError)
    })?;

    // Replace placeholders
    for (key, value) in placeholders {
        html_template = html_template.replace(key.as_str(), value.as_str());
    }

    // Build email
    let from: Mailbox = format!("{} <{}>", config.from_name, config.from_email)
        .parse()
        .map_err(|e| {
            error!("Invalid sender email: {}", e);
            AppError::server_error(ErrorMessage::EmailConfigurationError)
        })?;

    let to: Mailbox = format!("{} <{}>", to_name, to_email).parse().map_err(|e| {
        error!("Invalid recipient email: {}", e);
        AppError::server_error(ErrorMessage::EmailSendError)
    })?;

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html_template),
        )
        .map_err(|e| {
            error!("Failed to build email: {}", e);
            AppError::server_error(ErrorMessage::EmailSendError)
        })?;

    // Configure SMTP
    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let builder = if config.enable_tls {
        SmtpTransport::starttls_relay(&config.smtp_host).map_err(|e| {
            error!("Failed to create SMTP transport: {}", e);
            AppError::server_error(ErrorMessage::EmailConfigurationError)
        })?
    } else {
        SmtpTransport::builder_dangerous(&config.smtp_host)
    };

    let mailer = builder.credentials(creds).port(config.smtp_port).build();

    // Send email
    match mailer.send(&email) {
        Ok(_) => {
            info!("Email sent successfully to {}", to_email);
            Ok(())
        }
        Err(e) => {
            error!("Failed to send email to {}: {:?}", to_email, e);
            Err(AppError::server_error(ErrorMessage::EmailSendError))
        }
    }
}

/// Send email in background (fire and forget)
pub fn send_email_background(
    to_email: String,
    to_name: String,
    subject: String,
    template_path: String,
    placeholders: Vec<(String, String)>,
    config: EmailConfig,
) {
    tokio::spawn(async move {
        if let Err(e) = send_email(
            &to_email,
            &to_name,
            &subject,
            &template_path,
            &placeholders,
            &config,
        )
        .await
        {
            error!("Background email sending failed: {:?}", e);
        }
    });
}
