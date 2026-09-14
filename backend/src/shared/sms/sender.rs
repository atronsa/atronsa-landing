use tracing::info;

use crate::config::SmsConfig;
use crate::error::AppError;

// ============================================
// SMS Sender
// ============================================

/// Core SMS sending function. No SMS gateway is wired in yet, so this logs
/// the message (and the sender identity it would go out from) via
/// `tracing` instead of making a provider API call — this is the one
/// function to swap for a real gateway (e.g. Africa's Talking, Twilio)
/// later; every caller above it (phone verification, and any future SMS
/// flow) stays unchanged.
pub async fn send_sms(to: &str, body: &str, config: &SmsConfig) -> Result<(), AppError> {
    info!(to, from = %config.from_number, body, "SMS (stub, not actually sent)");
    Ok(())
}

/// Sends a phone-verification OTP SMS. `otp` is the plaintext 6-digit code
/// — only its Argon2 hash is ever persisted (see `shared::otp`).
pub async fn send_verification_otp_sms(
    to_phone: &str,
    otp: &str,
    config: &SmsConfig,
) -> Result<(), AppError> {
    let body = format!(
        "Your Atronsa verfication code is {}. This token is valid for 3 minutes old.",
        otp
    );

    send_sms(to_phone, &body, config).await
}
