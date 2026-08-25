use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, SinglePart, header},
    transport::smtp::authentication::Credentials,
};
use tracing::{error, info};

use crate::config::EmailConfig;
use crate::error::{AppError, ErrorMessage};

// ============================================
// Email Sender
// ============================================

/// Core email sending function.
///
/// `template` is the raw HTML body, embedded at compile time by the callers in
/// `templates.rs` — never read from disk, so a deployed binary doesn't depend on
/// its working directory to send mail.
pub async fn send_email(
    to_email: &str,
    to_name: &str,
    subject: &str,
    template: &str,
    placeholders: &[(String, String)],
    config: &EmailConfig,
) -> Result<(), AppError> {
    let mut html_template = template.to_string();

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
    template: &'static str,
    placeholders: Vec<(String, String)>,
    config: EmailConfig,
) {
    tokio::spawn(async move {
        if let Err(e) = send_email(
            &to_email,
            &to_name,
            &subject,
            template,
            &placeholders,
            &config,
        )
        .await
        {
            error!("Background email sending failed: {:?}", e);
        }
    });
}
