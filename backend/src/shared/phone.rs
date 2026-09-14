use validator::ValidationError;

use crate::error::{AppError, ErrorMessage};

/// Ethiopian mobile numbers: 9-prefixed (Ethio Telecom, Safaricom) and
/// 7-prefixed (Safaricom Ethiopia) lines, 9 digits after the leading digit,
/// optionally written with a `0` trunk prefix or a `+251`/`251` country code.
fn digits_after_country_code(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();

    if let Some(rest) = trimmed.strip_prefix("+251") {
        return Some(rest);
    }
    if let Some(rest) = trimmed.strip_prefix("251") {
        return Some(rest);
    }
    if let Some(rest) = trimmed.strip_prefix('0') {
        return Some(rest);
    }
    Some(trimmed)
}

fn is_valid_local_number(local: &str) -> bool {
    local.len() == 9
        && local.as_bytes()[0].is_ascii_digit()
        && matches!(local.as_bytes()[0], b'7' | b'9')
        && local.bytes().all(|b| b.is_ascii_digit())
}

/// Normalize an Ethiopian mobile number to canonical `+2519XXXXXXXX` /
/// `+2517XXXXXXXX` form, so the same subscriber can't register twice under
/// `0912345678`, `912345678`, and `+251912345678`.
pub fn normalize(raw: &str) -> Result<String, AppError> {
    let local = digits_after_country_code(raw).unwrap_or(raw);

    if !is_valid_local_number(local) {
        return Err(AppError::bad_request(ErrorMessage::InvalidPhoneFormat));
    }

    Ok(format!("+251{}", local))
}

/// Validator-crate adapter for DTO-level validation.
pub fn validate_ethiopian_phone(raw: &str) -> Result<(), ValidationError> {
    let local = digits_after_country_code(raw).unwrap_or(raw);

    if is_valid_local_number(local) {
        Ok(())
    } else {
        Err(
            ValidationError::new("invalid_phone").with_message(std::borrow::Cow::Borrowed(
                "Enter a valid Ethiopian mobile number, e.g. 0912345678 or +251912345678",
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_common_formats() {
        for input in ["0912345678", "912345678", "+251912345678", "251912345678"] {
            assert_eq!(normalize(input).unwrap(), "+251912345678", "input: {input}");
        }
    }

    #[test]
    fn accepts_seven_prefixed_lines() {
        assert_eq!(normalize("0712345678").unwrap(), "+251712345678");
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(normalize("09123456").is_err());
        assert!(normalize("091234567890").is_err());
    }

    #[test]
    fn rejects_non_mobile_leading_digit() {
        assert!(normalize("0212345678").is_err());
    }

    #[test]
    fn rejects_non_numeric() {
        assert!(normalize("09abcd5678").is_err());
    }
}
