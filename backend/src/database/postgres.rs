use crate::modules::admin::model::Admin;
use crate::modules::super_admin::model::SuperAdmin;
use crate::modules::user_wallet::model::UserWallet;
use crate::modules::users::model::{
    EmailVerificationState, Gender, PhoneVerificationState, User, UserRole, UserStatus,
};
use crate::shared::wallet;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{Acquire, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}
impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

/// Every column of `users` that maps onto the `User` model, in struct order.
/// Kept in one place so adding a field can't silently miss a query.
macro_rules! user_columns {
    () => {
        "id, first_name, last_name, email, password_hash, phone_no, email_verified, phone_verified, fayda_verified, atronsa_verified, wallet_id, last_login, role, status, date_of_birth, gender, location, work, profile_created_at, profile_updated_at, created_at, updated_at"
    };
}

macro_rules! user_query {
    ($($sql:expr),+ $(,)?) => {
        sqlx::query_as::<_, User>(concat!($($sql),+))
    };
}

/// Same idea as `user_columns!`, for `user_wallets` / `UserWallet`.
macro_rules! wallet_columns {
    () => {
        "wallet_id, user_id, wallet_number, passcode, status, currency, balance, current_card_id, created_at, updated_at"
    };
}

macro_rules! wallet_query {
    ($($sql:expr),+ $(,)?) => {
        sqlx::query_as::<_, UserWallet>(concat!($($sql),+))
    };
}

/// Same idea as `user_columns!`, for `super_admins` / `SuperAdmin`.
macro_rules! super_admin_columns {
    () => {
        "super_admin_id, name, email, password_hash, status, atronsa_verified, role, last_login, created_at"
    };
}

macro_rules! super_admin_query {
    ($($sql:expr),+ $(,)?) => {
        sqlx::query_as::<_, SuperAdmin>(concat!($($sql),+))
    };
}

/// Same idea as `user_columns!`, for `admins` / `Admin`.
macro_rules! admin_columns {
    () => {
        "admin_id, first_name, last_name, email, phone_no, password_hash, status, assigned_by, role, email_verified, phone_verified, fayda_verified, atronsa_verified, last_login, created_at, updated_at"
    };
}

macro_rules! admin_query {
    ($($sql:expr),+ $(,)?) => {
        sqlx::query_as::<_, Admin>(concat!($($sql),+))
    };
}

/// How many times to re-roll a colliding wallet number before giving up.
/// See `shared::wallet` for why collisions are expected at all.
const WALLET_NUMBER_ATTEMPTS: usize = 8;

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        id: Option<Uuid>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;
    async fn get_user_by_phone(&self, phone: &str) -> Result<Option<User>, sqlx::Error>;
    async fn get_users(
        &self,
        page: u32,
        limit: usize,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>, sqlx::Error>;
    async fn get_user_count_filtered(
        &self,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<i64, sqlx::Error>;
    async fn get_user_count(&self) -> Result<i64, sqlx::Error>;
    /// Creates the user *and* their wallet in one transaction, then points
    /// `users.wallet_id` at it. There is deliberately no plain `save_user`:
    /// every user has exactly one wallet, and splitting this into two calls
    /// would let a failure land between them and leave a walletless user.
    async fn save_user_with_wallet(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password_hash: &str,
        phone_no: &str,
    ) -> Result<(User, UserWallet), sqlx::Error>;
    async fn update_user_role(&self, id: Uuid, role: UserRole) -> Result<User, sqlx::Error>;
    async fn update_user_status(&self, id: Uuid, status: UserStatus) -> Result<User, sqlx::Error>;
    async fn update_user_password(
        &self,
        id: Uuid,
        password_hash: String,
    ) -> Result<User, sqlx::Error>;
    async fn update_last_login(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn clear_reset_token(&self, token: &str) -> Result<(), sqlx::Error>;
    async fn get_user_by_reset_token(&self, token: &str) -> Result<Option<User>, sqlx::Error>;
    async fn set_reset_token(
        &self,
        id: Uuid,
        token: &str,
        expires: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
    async fn email_exists(&self, email: &str) -> Result<bool, sqlx::Error>;
    async fn phone_exists(&self, phone: &str) -> Result<bool, sqlx::Error>;

    // Profile — the profile fields live on `users`, so this returns the whole
    // updated user rather than a separate profile row.
    async fn update_profile(
        &self,
        user_id: Uuid,
        date_of_birth: Option<NaiveDate>,
        gender: Option<Gender>,
        location: Option<&str>,
        work: Option<&str>,
    ) -> Result<User, sqlx::Error>;

    // Refresh token revocation
    async fn revoke_refresh_token(
        &self,
        jti: Uuid,
        user_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
    async fn is_refresh_token_revoked(&self, jti: Uuid) -> Result<bool, sqlx::Error>;

    // Email verification (OTP) — see `modules::users::model::EmailVerificationState`
    // and CLAUDE.md "Email verification" for the full flow this backs.
    async fn get_email_verification_state(
        &self,
        id: Uuid,
    ) -> Result<EmailVerificationState, sqlx::Error>;
    /// Issues a fresh OTP: sets the hash/expiry, stamps `last_sent_at` (for
    /// the resend cooldown), and resets attempts/lock. Only ever called once
    /// the handler has confirmed no lock is currently active, so resetting
    /// the lock here is always resetting an already-expired one.
    async fn set_email_verification_otp(
        &self,
        id: Uuid,
        otp_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
    /// Atomically increments the failed-attempt counter and, if it just
    /// crossed `max_attempts`, sets `locked_until` to `lockout_minutes` from
    /// now in the same statement — avoids a check-then-act race between two
    /// concurrent wrong guesses. Returns the new attempt count and lock.
    async fn record_failed_email_verification_attempt(
        &self,
        id: Uuid,
        max_attempts: i16,
        lockout_minutes: i32,
    ) -> Result<(i16, Option<DateTime<Utc>>), sqlx::Error>;
    async fn mark_email_verified(&self, id: Uuid) -> Result<(), sqlx::Error>;

    // Phone verification (OTP) — see `modules::users::model::PhoneVerificationState`
    // and CLAUDE.md "Phone verification" for the full flow this backs.
    // Unlike email verification, the lockout here rate-limits how many OTPs
    // can be *sent* (not wrong verification guesses), so there is no
    // equivalent of `record_failed_email_verification_attempt` — the send
    // count and its lockout are computed by the caller (which already has
    // to read the current state to check the resend cooldown) and passed
    // in explicitly.
    async fn get_phone_verification_state(
        &self,
        id: Uuid,
    ) -> Result<PhoneVerificationState, sqlx::Error>;
    /// Issues a fresh OTP: sets the hash/expiry, stamps `last_sent_at`, and
    /// records the send count / lockout the caller computed.
    async fn set_phone_verification_otp(
        &self,
        id: Uuid,
        otp_hash: &str,
        expires_at: DateTime<Utc>,
        send_count: i16,
        locked_until: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error>;
    async fn mark_phone_verified(&self, id: Uuid) -> Result<(), sqlx::Error>;

    // Atronsa verification — a derived status, never set directly. It's
    // recomputed from `email_verified`/`phone_verified`/`fayda_verified` in
    // one atomic UPDATE (rather than read-then-write) so it can't drift from
    // whatever those three columns currently say. Returns the resulting
    // value.
    async fn sync_atronsa_verified(&self, id: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient {
    async fn get_user(
        &self,
        id: Option<Uuid>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        if let Some(v) = id {
            return user_query!("SELECT ", user_columns!(), " FROM users WHERE id=$1")
                .bind(v)
                .fetch_optional(&self.pool)
                .await;
        }
        if let Some(v) = email {
            return user_query!(
                "SELECT ",
                user_columns!(),
                " FROM users WHERE LOWER(email)=LOWER($1)"
            )
            .bind(v)
            .fetch_optional(&self.pool)
            .await;
        }
        Ok(None)
    }

    async fn get_user_by_phone(&self, phone: &str) -> Result<Option<User>, sqlx::Error> {
        user_query!("SELECT ", user_columns!(), " FROM users WHERE phone_no=$1")
            .bind(phone)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_users(
        &self,
        page: u32,
        limit: usize,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>, sqlx::Error> {
        user_query!("SELECT ", user_columns!(), " FROM users WHERE ($1::user_role IS NULL OR role=$1) AND ($2::user_status IS NULL OR status=$2) ORDER BY created_at DESC LIMIT $3 OFFSET $4").bind(role).bind(status).bind(limit as i64).bind(((page.max(1)-1)*limit as u32) as i64).fetch_all(&self.pool).await
    }

    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await
    }

    async fn get_user_count_filtered(
        &self,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE ($1::user_role IS NULL OR role=$1) AND ($2::user_status IS NULL OR status=$2)")
            .bind(role).bind(status).fetch_one(&self.pool).await
    }

    async fn save_user_with_wallet(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password_hash: &str,
        phone_no: &str,
    ) -> Result<(User, UserWallet), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let user = user_query!("INSERT INTO users(first_name,last_name,email,password_hash,phone_no) VALUES($1,$2,$3,$4,$5) RETURNING ", user_columns!())
            .bind(first_name).bind(last_name).bind(email).bind(password_hash).bind(phone_no)
            .fetch_one(&mut *tx).await?;

        // Wallet numbers are random, so two concurrent registrations can pick
        // the same one and the UNIQUE index is what catches it. Each attempt
        // runs inside a savepoint: a unique violation would otherwise poison
        // the whole transaction and take the user insert down with it.
        let mut wallet = None;
        for attempt in 0..WALLET_NUMBER_ATTEMPTS {
            let number = wallet::generate_number();
            let mut savepoint = (&mut *tx).begin().await?;

            let inserted = wallet_query!(
                "INSERT INTO user_wallets(user_id, wallet_number) VALUES($1,$2) RETURNING ",
                wallet_columns!()
            )
            .bind(user.id)
            .bind(&number)
            .fetch_one(&mut *savepoint)
            .await;

            match inserted {
                Ok(row) => {
                    savepoint.commit().await?;
                    wallet = Some(row);
                    break;
                }
                Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
                    savepoint.rollback().await?;
                    tracing::warn!(
                        attempt = attempt + 1,
                        "wallet number collision, retrying with a new number"
                    );
                }
                Err(e) => return Err(e),
            }
        }

        let Some(wallet) = wallet else {
            // Every attempt collided — only plausible once the number space
            // is genuinely crowded. See `shared::wallet`.
            tracing::error!("exhausted wallet number attempts for a new user");
            return Err(sqlx::Error::Protocol(
                "could not allocate a unique wallet number".into(),
            ));
        };

        let user = user_query!(
            "UPDATE users SET wallet_id=$1, updated_at=NOW() WHERE id=$2 RETURNING ",
            user_columns!()
        )
        .bind(wallet.wallet_id)
        .bind(user.id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok((user, wallet))
    }

    async fn update_user_role(&self, id: Uuid, role: UserRole) -> Result<User, sqlx::Error> {
        user_query!(
            "UPDATE users SET role=$1,updated_at=NOW() WHERE id=$2 RETURNING ",
            user_columns!()
        )
        .bind(role)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user_status(&self, id: Uuid, status: UserStatus) -> Result<User, sqlx::Error> {
        user_query!(
            "UPDATE users SET status=$1,updated_at=NOW() WHERE id=$2 RETURNING ",
            user_columns!()
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user_password(
        &self,
        id: Uuid,
        password_hash: String,
    ) -> Result<User, sqlx::Error> {
        user_query!(
            "UPDATE users SET password_hash=$1,updated_at=NOW() WHERE id=$2 RETURNING ",
            user_columns!()
        )
        .bind(password_hash)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_last_login(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET last_login=NOW(), updated_at=NOW() WHERE id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn clear_reset_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password_reset_token=NULL,password_reset_expires_at=NULL,updated_at=NOW() WHERE password_reset_token=$1 AND password_reset_expires_at>NOW()").bind(token).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_user_by_reset_token(&self, token: &str) -> Result<Option<User>, sqlx::Error> {
        user_query!(
            "SELECT ",
            user_columns!(),
            " FROM users WHERE password_reset_token=$1 AND password_reset_expires_at>NOW()"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
    }

    async fn set_reset_token(
        &self,
        id: Uuid,
        token: &str,
        expires: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password_reset_token=$1,password_reset_expires_at=$2,updated_at=NOW() WHERE id=$3").bind(token).bind(expires).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    async fn email_exists(&self, email: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM users WHERE LOWER(email)=LOWER($1))",
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
    }

    async fn phone_exists(&self, phone: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE phone_no=$1)")
            .bind(phone)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_profile(
        &self,
        user_id: Uuid,
        date_of_birth: Option<NaiveDate>,
        gender: Option<Gender>,
        location: Option<&str>,
        work: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        // COALESCE keeps this a partial update: fields the client omitted
        // arrive as NULL and leave the stored value untouched.
        // `profile_created_at` is stamped once, on the first submission, and
        // is what marks the user as having a profile at all.
        user_query!(
            "UPDATE users SET ",
            "date_of_birth = COALESCE($2, date_of_birth), ",
            "gender = COALESCE($3, gender), ",
            "location = COALESCE($4, location), ",
            "work = COALESCE($5, work), ",
            "profile_created_at = COALESCE(profile_created_at, NOW()), ",
            "profile_updated_at = NOW(), ",
            "updated_at = NOW() ",
            "WHERE id = $1 RETURNING ",
            user_columns!()
        )
        .bind(user_id)
        .bind(date_of_birth)
        .bind(gender)
        .bind(location)
        .bind(work)
        .fetch_one(&self.pool)
        .await
    }

    async fn revoke_refresh_token(
        &self,
        jti: Uuid,
        user_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO revoked_refresh_tokens (jti, user_id, expires_at) VALUES ($1, $2, $3) ON CONFLICT (jti) DO NOTHING",
        )
        .bind(jti)
        .bind(user_id)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn is_refresh_token_revoked(&self, jti: Uuid) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM revoked_refresh_tokens WHERE jti=$1)",
        )
        .bind(jti)
        .fetch_one(&self.pool)
        .await
    }

    async fn get_email_verification_state(
        &self,
        id: Uuid,
    ) -> Result<EmailVerificationState, sqlx::Error> {
        sqlx::query_as::<_, EmailVerificationState>(
            "SELECT email_verified, email_verification_otp_hash, email_verification_otp_expires_at, email_verification_attempts, email_verification_last_sent_at, email_verification_locked_until FROM users WHERE id=$1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn set_email_verification_otp(
        &self,
        id: Uuid,
        otp_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET email_verification_otp_hash=$1, email_verification_otp_expires_at=$2, email_verification_last_sent_at=NOW(), email_verification_attempts=0, email_verification_locked_until=NULL, updated_at=NOW() WHERE id=$3",
        )
        .bind(otp_hash)
        .bind(expires_at)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn record_failed_email_verification_attempt(
        &self,
        id: Uuid,
        max_attempts: i16,
        lockout_minutes: i32,
    ) -> Result<(i16, Option<DateTime<Utc>>), sqlx::Error> {
        sqlx::query_as::<_, (i16, Option<DateTime<Utc>>)>(
            "UPDATE users SET \
                email_verification_attempts = email_verification_attempts + 1, \
                email_verification_locked_until = CASE \
                    WHEN email_verification_attempts + 1 >= $2 THEN NOW() + make_interval(mins => $3) \
                    ELSE email_verification_locked_until \
                END, \
                updated_at = NOW() \
             WHERE id = $1 \
             RETURNING email_verification_attempts, email_verification_locked_until",
        )
        .bind(id)
        .bind(max_attempts)
        .bind(lockout_minutes)
        .fetch_one(&self.pool)
        .await
    }

    async fn mark_email_verified(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET email_verified=true, email_verification_otp_hash=NULL, email_verification_otp_expires_at=NULL, email_verification_attempts=0, email_verification_locked_until=NULL, updated_at=NOW() WHERE id=$1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_phone_verification_state(
        &self,
        id: Uuid,
    ) -> Result<PhoneVerificationState, sqlx::Error> {
        sqlx::query_as::<_, PhoneVerificationState>(
            "SELECT phone_verified, phone_verification_otp_hash, phone_verification_otp_expires_at, phone_verification_send_count, phone_verification_last_sent_at, phone_verification_locked_until FROM users WHERE id=$1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn set_phone_verification_otp(
        &self,
        id: Uuid,
        otp_hash: &str,
        expires_at: DateTime<Utc>,
        send_count: i16,
        locked_until: Option<DateTime<Utc>>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET phone_verification_otp_hash=$1, phone_verification_otp_expires_at=$2, phone_verification_send_count=$3, phone_verification_last_sent_at=NOW(), phone_verification_locked_until=$4, updated_at=NOW() WHERE id=$5",
        )
        .bind(otp_hash)
        .bind(expires_at)
        .bind(send_count)
        .bind(locked_until)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn mark_phone_verified(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET phone_verified=true, phone_verification_otp_hash=NULL, phone_verification_otp_expires_at=NULL, phone_verification_send_count=0, phone_verification_last_sent_at=NULL, phone_verification_locked_until=NULL, updated_at=NOW() WHERE id=$1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn sync_atronsa_verified(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "UPDATE users SET \
                atronsa_verified = (email_verified AND phone_verified AND fayda_verified), \
                updated_at = NOW() \
             WHERE id = $1 \
             RETURNING atronsa_verified",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
}

// ============================================
// Wallets
// ============================================

#[async_trait]
pub trait WalletExt {
    /// The 1:1 lookup every authenticated wallet request uses.
    async fn get_wallet_by_user_id(&self, user_id: Uuid)
    -> Result<Option<UserWallet>, sqlx::Error>;
    async fn get_wallet_by_id(&self, wallet_id: Uuid) -> Result<Option<UserWallet>, sqlx::Error>;
    async fn get_wallet_by_number(
        &self,
        wallet_number: &str,
    ) -> Result<Option<UserWallet>, sqlx::Error>;
    async fn get_wallets(&self, page: u32, limit: usize) -> Result<Vec<UserWallet>, sqlx::Error>;
    async fn get_wallet_count(&self) -> Result<i64, sqlx::Error>;
}

#[async_trait]
impl WalletExt for DBClient {
    async fn get_wallet_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserWallet>, sqlx::Error> {
        wallet_query!(
            "SELECT ",
            wallet_columns!(),
            " FROM user_wallets WHERE user_id=$1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_wallet_by_id(&self, wallet_id: Uuid) -> Result<Option<UserWallet>, sqlx::Error> {
        wallet_query!(
            "SELECT ",
            wallet_columns!(),
            " FROM user_wallets WHERE wallet_id=$1"
        )
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_wallet_by_number(
        &self,
        wallet_number: &str,
    ) -> Result<Option<UserWallet>, sqlx::Error> {
        wallet_query!(
            "SELECT ",
            wallet_columns!(),
            " FROM user_wallets WHERE wallet_number=$1"
        )
        .bind(wallet_number)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_wallets(&self, page: u32, limit: usize) -> Result<Vec<UserWallet>, sqlx::Error> {
        wallet_query!(
            "SELECT ",
            wallet_columns!(),
            " FROM user_wallets ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(((page.max(1) - 1) * limit as u32) as i64)
        .fetch_all(&self.pool)
        .await
    }

    async fn get_wallet_count(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM user_wallets")
            .fetch_one(&self.pool)
            .await
    }
}

// ============================================
// Super admins
// ============================================
//
// Deliberately minimal: the writer of this table is the `create-super-admin`
// bootstrap binary (existence check + a single insert); the login handler
// adds a lookup-by-email and a last-login stamp. Not a general CRUD surface.

#[async_trait]
pub trait SuperAdminExt {
    /// Whether any super admin row exists yet — what makes the bootstrap
    /// binary idempotent.
    async fn super_admin_exists(&self) -> Result<bool, sqlx::Error>;
    async fn create_super_admin(
        &self,
        name: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<SuperAdmin, sqlx::Error>;
    async fn get_super_admin_by_email(
        &self,
        email: &str,
    ) -> Result<Option<SuperAdmin>, sqlx::Error>;
    /// Used by `super_admin_auth_middleware` to re-resolve the JWT's `sub`
    /// against a live row (and its current `status`) on every request.
    async fn get_super_admin_by_id(&self, id: Uuid) -> Result<Option<SuperAdmin>, sqlx::Error>;
    async fn update_super_admin_last_login(&self, id: Uuid) -> Result<(), sqlx::Error>;

    // Super-admin refresh token revocation — same idea as
    // `UserExt::revoke_refresh_token`/`is_refresh_token_revoked`, backed by
    // `revoked_super_admin_refresh_tokens` instead of
    // `revoked_refresh_tokens` (see that migration's comment for why).
    async fn revoke_super_admin_refresh_token(
        &self,
        jti: Uuid,
        super_admin_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
    async fn is_super_admin_refresh_token_revoked(&self, jti: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
impl SuperAdminExt for DBClient {
    async fn super_admin_exists(&self) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM super_admins)")
            .fetch_one(&self.pool)
            .await
    }

    async fn create_super_admin(
        &self,
        name: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<SuperAdmin, sqlx::Error> {
        super_admin_query!(
            "INSERT INTO super_admins(name, email, password_hash) VALUES($1,$2,$3) RETURNING ",
            super_admin_columns!()
        )
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
    }

    async fn get_super_admin_by_email(
        &self,
        email: &str,
    ) -> Result<Option<SuperAdmin>, sqlx::Error> {
        super_admin_query!(
            "SELECT ",
            super_admin_columns!(),
            " FROM super_admins WHERE LOWER(email)=LOWER($1)"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_super_admin_by_id(&self, id: Uuid) -> Result<Option<SuperAdmin>, sqlx::Error> {
        super_admin_query!(
            "SELECT ",
            super_admin_columns!(),
            " FROM super_admins WHERE super_admin_id=$1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn update_super_admin_last_login(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE super_admins SET last_login=NOW() WHERE super_admin_id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn revoke_super_admin_refresh_token(
        &self,
        jti: Uuid,
        super_admin_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO revoked_super_admin_refresh_tokens (jti, super_admin_id, expires_at) VALUES ($1, $2, $3) ON CONFLICT (jti) DO NOTHING",
        )
        .bind(jti)
        .bind(super_admin_id)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn is_super_admin_refresh_token_revoked(&self, jti: Uuid) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM revoked_super_admin_refresh_tokens WHERE jti=$1)",
        )
        .bind(jti)
        .fetch_one(&self.pool)
        .await
    }
}

// ============================================
// Admins
// ============================================
//
// Staff accounts created exclusively via `POST /api/v1/super-admin/admins`.
// Deliberately minimal, same spirit as `SuperAdminExt`: an existence check
// per uniqueness column and a single insert, not a general CRUD surface.

#[async_trait]
pub trait AdminExt {
    async fn admin_email_exists(&self, email: &str) -> Result<bool, sqlx::Error>;
    async fn admin_phone_exists(&self, phone: &str) -> Result<bool, sqlx::Error>;
    async fn create_admin(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_no: &str,
        password_hash: &str,
        assigned_by: Uuid,
    ) -> Result<Admin, sqlx::Error>;
    async fn get_admin_by_email(&self, email: &str) -> Result<Option<Admin>, sqlx::Error>;
    /// Used by `admin_auth_middleware` to re-resolve the JWT's `sub` against
    /// a live row (and its current `status`) on every request.
    async fn get_admin_by_id(&self, id: Uuid) -> Result<Option<Admin>, sqlx::Error>;
    async fn update_admin_last_login(&self, id: Uuid) -> Result<(), sqlx::Error>;

    // Admin refresh token revocation — same idea as
    // `UserExt::revoke_refresh_token`/`is_refresh_token_revoked` and
    // `SuperAdminExt`'s equivalents, backed by its own
    // `revoked_admin_refresh_tokens` table (see that migration's comment
    // for why it can't reuse either existing one).
    async fn revoke_admin_refresh_token(
        &self,
        jti: Uuid,
        admin_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
    async fn is_admin_refresh_token_revoked(&self, jti: Uuid) -> Result<bool, sqlx::Error>;
}

#[async_trait]
impl AdminExt for DBClient {
    async fn admin_email_exists(&self, email: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM admins WHERE LOWER(email)=LOWER($1))",
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
    }

    async fn admin_phone_exists(&self, phone: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM admins WHERE phone_no=$1)")
            .bind(phone)
            .fetch_one(&self.pool)
            .await
    }

    async fn create_admin(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_no: &str,
        password_hash: &str,
        assigned_by: Uuid,
    ) -> Result<Admin, sqlx::Error> {
        admin_query!(
            "INSERT INTO admins(first_name, last_name, email, phone_no, password_hash, assigned_by) VALUES($1,$2,$3,$4,$5,$6) RETURNING ",
            admin_columns!()
        )
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(phone_no)
        .bind(password_hash)
        .bind(assigned_by)
        .fetch_one(&self.pool)
        .await
    }

    async fn get_admin_by_email(&self, email: &str) -> Result<Option<Admin>, sqlx::Error> {
        admin_query!(
            "SELECT ",
            admin_columns!(),
            " FROM admins WHERE LOWER(email)=LOWER($1)"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_admin_by_id(&self, id: Uuid) -> Result<Option<Admin>, sqlx::Error> {
        admin_query!("SELECT ", admin_columns!(), " FROM admins WHERE admin_id=$1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn update_admin_last_login(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE admins SET last_login=NOW() WHERE admin_id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn revoke_admin_refresh_token(
        &self,
        jti: Uuid,
        admin_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO revoked_admin_refresh_tokens (jti, admin_id, expires_at) VALUES ($1, $2, $3) ON CONFLICT (jti) DO NOTHING",
        )
        .bind(jti)
        .bind(admin_id)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn is_admin_refresh_token_revoked(&self, jti: Uuid) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM revoked_admin_refresh_tokens WHERE jti=$1)",
        )
        .bind(jti)
        .fetch_one(&self.pool)
        .await
    }
}
