use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use validator::ValidationError;

use crate::error::{AppError, ErrorMessage};

// ============================================
// Constants
// ============================================

const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 8;
/// Stricter policy for staff (admin) accounts, matching the super-admin
/// bootstrap binary's requirement.
const ADMIN_MIN_PASSWORD_LENGTH: usize = 10;

// ============================================
// Password Hashing
// ============================================

/// Hash a password using Argon2id (default)
pub fn hash(password: impl Into<String>) -> Result<String, AppError> {
    let password = password.into();

    validate_password(&password)?;

    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Password hashing failed: {:?}", e);
            AppError::server_error(ErrorMessage::HashingError)
        })?
        .to_string();

    Ok(hashed_password)
}

/// Verify a password against its hash
pub fn compare(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|e| {
        tracing::error!("Invalid password hash format: {:?}", e);
        AppError::server_error(ErrorMessage::InvalidHashFormat)
    })?;

    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

// ============================================
// Password Validation
// ============================================

/// Core password complexity rule, shared by the `hash()` gate and DTO-level
/// validation so both paths enforce exactly the same policy. `min_length` is
/// a parameter (rather than always `MIN_PASSWORD_LENGTH`) so callers with a
/// stricter policy — e.g. the super-admin bootstrap binary, which requires
/// 10+ chars — can reuse the same character-class checks instead of
/// duplicating them.
///
/// Requires: `min_length`-64 chars, at least one uppercase, one lowercase,
/// one digit, and one special (non-alphanumeric) character.
pub fn check_complexity(password: &str, min_length: usize) -> Result<(), &'static str> {
    if password.is_empty() {
        return Err("empty");
    }
    if password.len() < min_length {
        return Err("too_short");
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err("too_long");
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("missing_uppercase");
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err("missing_lowercase");
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("missing_digit");
    }
    if !password
        .chars()
        .any(|c| !c.is_alphanumeric() && !c.is_whitespace())
    {
        return Err("missing_special");
    }
    Ok(())
}

/// Validate password meets security requirements (used internally by `hash()`).
pub fn validate_password(password: &str) -> Result<(), AppError> {
    check_complexity(password, MIN_PASSWORD_LENGTH).map_err(|reason| match reason {
        "empty" => AppError::bad_request(ErrorMessage::EmptyPassword),
        "too_long" => {
            AppError::bad_request(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH))
        }
        _ => AppError::bad_request(ErrorMessage::WeakPassword),
    })
}

/// Validator-crate adapter for DTO-level validation (e.g. `#[validate(custom(...))]`),
/// so registration requests fail fast with a field-level message instead of
/// deep inside `hash()`.
pub fn validate_password_complexity(password: &str) -> Result<(), ValidationError> {
    check_complexity(password, MIN_PASSWORD_LENGTH).map_err(|reason| {
        ValidationError::new("weak_password").with_message(std::borrow::Cow::Borrowed(
            match reason {
                "empty" => "Password is required",
                "too_short" => "Password must be at least 8 characters",
                "too_long" => "Password must not exceed 64 characters",
                "missing_uppercase" => "Password must contain at least one uppercase letter",
                "missing_lowercase" => "Password must contain at least one lowercase letter",
                "missing_digit" => "Password must contain at least one number",
                _ => "Password must contain at least one special character",
            },
        ))
    })
}

/// Validator-crate adapter for admin-creation DTOs, enforcing the stricter
/// 10+ character policy (same requirement as the super-admin bootstrap
/// binary) instead of the regular 8+ policy.
pub fn validate_admin_password_complexity(password: &str) -> Result<(), ValidationError> {
    check_complexity(password, ADMIN_MIN_PASSWORD_LENGTH).map_err(|reason| {
        ValidationError::new("weak_password").with_message(std::borrow::Cow::Borrowed(
            match reason {
                "empty" => "Password is required",
                "too_short" => "Password must be at least 10 characters",
                "too_long" => "Password must not exceed 64 characters",
                "missing_uppercase" => "Password must contain at least one uppercase letter",
                "missing_lowercase" => "Password must contain at least one lowercase letter",
                "missing_digit" => "Password must contain at least one number",
                _ => "Password must contain at least one special character",
            },
        ))
    })
}

/// Check password strength (optional utility)
pub fn check_password_strength(password: &str) -> PasswordStrength {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }
    if password.len() >= 12 {
        score += 1;
    }

    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }
    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    }
    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        score += 1;
    }

    match score {
        0..=2 => PasswordStrength::Weak,
        3..=4 => PasswordStrength::Medium,
        5 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl PasswordStrength {
    pub fn as_str(&self) -> &str {
        match self {
            PasswordStrength::Weak => "weak",
            PasswordStrength::Medium => "medium",
            PasswordStrength::Strong => "strong",
            PasswordStrength::VeryStrong => "very_strong",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_compare() {
        let password = "Atronsa@2026";

        let hash = hash(password).unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2id$"));

        let is_valid = compare(password, &hash).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_wrong_password() {
        let hash = hash("Correct@Pass1").unwrap();
        let is_valid = compare("Wrong@Pass1", &hash).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_empty_password() {
        assert!(hash("").is_err());
    }

    #[test]
    fn test_rejects_weak_passwords() {
        for weak in ["password", "password123", "Password", "Password123"] {
            assert!(hash(weak).is_err(), "expected {weak:?} to be rejected");
        }
    }

    #[test]
    fn test_accepts_valid_password() {
        assert!(hash("Atronsa@2026").is_ok());
    }

    #[test]
    fn test_password_strength() {
        assert_eq!(check_password_strength("weak"), PasswordStrength::Weak);
        assert_eq!(
            check_password_strength("Medium12"),
            PasswordStrength::Medium
        );
        assert_eq!(
            check_password_strength("Str0ng!Pass"),
            PasswordStrength::Strong
        );
        assert_eq!(
            check_password_strength("V3ry$tr0ng!P@ss"),
            PasswordStrength::VeryStrong
        );
    }
}
