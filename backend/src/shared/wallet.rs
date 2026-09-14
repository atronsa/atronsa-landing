// Wallet number generation.
//
// A wallet number is the user-facing handle for a wallet: `ATR` followed by
// six digits (e.g. `ATR418902`). It is generated at registration and must be
// unique, which the `user_wallets.wallet_number` UNIQUE index enforces — this
// module only proposes candidates, the database is the authority. Callers
// retry on a unique violation (see `DBClient::save_user_with_wallet`).
//
// Six digits is 900,000 possible numbers, so collisions become common well
// before that many wallets exist (a ~50% chance of at least one collision
// somewhere around 1,100 wallets). The retry loop absorbs that; if the wallet
// count ever approaches six figures, widen the numeric part rather than
// raising the retry count.

use uuid::Uuid;

pub const WALLET_NUMBER_PREFIX: &str = "ATR";

/// A candidate wallet number: `ATR` + six digits, no leading zero.
///
/// Randomness comes from a v4 UUID (i.e. the OS RNG) rather than a counter or
/// the clock, so two registrations in the same instant don't line up — the
/// same reason the test helpers moved off `SystemTime`.
pub fn generate_number() -> String {
    let n = (Uuid::new_v4().as_u128() % 900_000) as u32 + 100_000;
    format!("{WALLET_NUMBER_PREFIX}{n}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn generated_number_has_the_atr_prefix_and_six_digits() {
        for _ in 0..1000 {
            let number = generate_number();
            assert_eq!(number.len(), 9, "expected ATR + 6 digits, got {number}");
            let digits = number
                .strip_prefix(WALLET_NUMBER_PREFIX)
                .unwrap_or_else(|| panic!("{number} should start with ATR"));
            assert_eq!(digits.len(), 6);
            assert!(
                digits.chars().all(|c| c.is_ascii_digit()),
                "{number} should be all digits after the prefix"
            );
            assert!(
                !digits.starts_with('0'),
                "{number} should not have a leading zero"
            );
        }
    }

    #[test]
    fn generated_numbers_are_not_all_the_same() {
        // Not a uniqueness guarantee — that's the DB's job — just a check
        // that the generator is actually random and not, say, seeded once.
        let unique: HashSet<String> = (0..100).map(|_| generate_number()).collect();
        assert!(unique.len() > 90, "generator produced too many duplicates");
    }
}
