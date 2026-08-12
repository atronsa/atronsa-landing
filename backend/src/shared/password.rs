use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::error::{AppError, ErrorMessage};

// ============================================
// Constants
// ============================================

const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 6;

// ============================================
// Password Hashing
// ============================================

/// Hash a password using Argon2id (default)
///
/// # Arguments
/// * `password` - Plain text password
///
/// # Returns
/// * `Ok(String)` - Hashed password string
/// * `Err(AppError)` - If password is invalid or hashing fails
///
/// # Example
/// ```rust
/// let hash = hash("my_secure_password")?;
/// ```
pub fn hash(password: impl Into<String>) -> Result<String, AppError> {
    let password = password.into();

    // Validate password
    validate_password(&password)?;

    // Generate random salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash password with Argon2id
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
///
/// # Arguments
/// * `password` - Plain text password to verify
/// * `hashed_password` - Previously hashed password
///
/// # Returns
/// * `Ok(bool)` - True if password matches
/// * `Err(AppError)` - If password is invalid or hash format is wrong
///
/// # Example
/// ```rust
/// let is_valid = compare("my_password", &stored_hash)?;
/// ```
pub fn compare(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    // Parse the hash string
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|e| {
        tracing::error!("Invalid password hash format: {:?}", e);
        AppError::server_error(ErrorMessage::InvalidHashFormat)
    })?;

    // Verify password against hash
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

/// Hash a password with custom Argon2 parameters (for higher security)
///
/// # Arguments
/// * `password` - Plain text password
///
/// # Returns
/// * `Ok(String)` - Hashed password string with custom params
pub fn hash_with_custom_params(password: impl Into<String>) -> Result<String, AppError> {
    let password = password.into();
    validate_password(&password)?;

    // Custom Argon2 configuration for higher security
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id, // Algorithm
        argon2::Version::V0x13,      // Version
        argon2::Params::new(
            65536,    // Memory cost (64 MB)
            3,        // Time cost (iterations)
            4,        // Parallelism
            Some(32), // Output length
        )
        .map_err(|e| {
            tracing::error!("Failed to create Argon2 params: {:?}", e);
            AppError::server_error(ErrorMessage::HashingError)
        })?,
    );

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::server_error(ErrorMessage::HashingError))?
        .to_string();

    Ok(hashed_password)
}

/// Validate password meets security requirements
pub fn validate_password(password: &str) -> Result<(), AppError> {
    // Check if empty
    if password.is_empty() {
        return Err(AppError::bad_request(ErrorMessage::EmptyPassword));
    }

    // Check minimum length
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(AppError::bad_request(ErrorMessage::InvalidPasswordFormat));
    }

    // Check maximum length
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(AppError::bad_request(
            ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH),
        ));
    }

    Ok(())
}
/// Check password strength (optional utility)
pub fn check_password_strength(password: &str) -> PasswordStrength {
    let mut score = 0;

    // Length check
    if password.len() >= 8 {
        score += 1;
    }
    if password.len() >= 12 {
        score += 1;
    }

    // Character variety
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
        let password = "SecurePass123!";

        let hash = hash(password).unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2id$"));

        let is_valid = compare(password, &hash).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_wrong_password() {
        let hash = hash("correct_password").unwrap();
        let is_valid = compare("wrong_password", &hash).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_empty_password() {
        let result = hash("");
        assert!(result.is_err());
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
