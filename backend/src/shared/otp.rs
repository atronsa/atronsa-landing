// One-time-passcode generation/hashing for email verification.
//
// OTPs are short-lived, numeric, and rate-limited, so they don't go through
// `shared::password` (which enforces password complexity rules a 6-digit
// code could never satisfy) — this module hashes them the same way
// (Argon2id via the same `argon2` crate) but without that gate.

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use uuid::Uuid;

use crate::error::{AppError, ErrorMessage};

/// A 6-digit numeric code, zero-padded (e.g. `"042918"`). Randomness comes
/// from a v4 UUID (the OS RNG), the same source `shared::wallet::generate_number`
/// uses, rather than pulling in a dedicated `rand` dependency for one call site.
pub fn generate_otp() -> String {
    let n = (Uuid::new_v4().as_u128() % 1_000_000) as u32;
    format!("{n:06}")
}

/// Hash an OTP for storage (Argon2id).
pub fn hash_otp(otp: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let hashed = Argon2::default()
        .hash_password(otp.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("OTP hashing failed: {:?}", e);
            AppError::server_error(ErrorMessage::HashingError)
        })?
        .to_string();

    Ok(hashed)
}

/// Verify a candidate OTP against its stored hash.
pub fn verify_otp(otp: &str, hashed_otp: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hashed_otp).map_err(|e| {
        tracing::error!("Invalid OTP hash format: {:?}", e);
        AppError::server_error(ErrorMessage::InvalidHashFormat)
    })?;

    Ok(Argon2::default()
        .verify_password(otp.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn generated_otp_is_six_digits() {
        for _ in 0..1000 {
            let otp = generate_otp();
            assert_eq!(otp.len(), 6, "expected 6 digits, got {otp}");
            assert!(
                otp.chars().all(|c| c.is_ascii_digit()),
                "{otp} should be all digits"
            );
        }
    }

    #[test]
    fn generated_otps_are_not_all_the_same() {
        let unique: HashSet<String> = (0..100).map(|_| generate_otp()).collect();
        assert!(unique.len() > 90, "generator produced too many duplicates");
    }

    #[test]
    fn hash_and_verify_round_trip() {
        let otp = "123456";
        let hash = hash_otp(otp).unwrap();
        assert!(hash.starts_with("$argon2id$"));
        assert!(verify_otp(otp, &hash).unwrap());
        assert!(!verify_otp("654321", &hash).unwrap());
    }
}
