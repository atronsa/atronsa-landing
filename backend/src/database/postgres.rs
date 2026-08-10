use crate::modules::users::model::{User, UserRole, UserStatus};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
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

const C: &str = "id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at";

macro_rules! user_query {
    ($sql:literal) => {
        sqlx::query_as::<_, User>($sql)
    };
}

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        id: Option<Uuid>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;
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
    async fn save_user(
        &self,
        first: &str,
        last: &str,
        email: &str,
        password_hash: &str,
        phone: Option<&str>,
        token: &str,
        expires: DateTime<Utc>,
    ) -> Result<User, sqlx::Error>;
    async fn get_user_count(&self) -> Result<i64, sqlx::Error>;
    async fn update_user_profile(
        &self,
        id: Uuid,
        first: Option<&str>,
        last: Option<&str>,
    ) -> Result<User, sqlx::Error>;
    async fn update_user_email(&self, id: Uuid, email: &str) -> Result<User, sqlx::Error>;
    async fn update_user_phone(&self, id: Uuid, phone: &str) -> Result<User, sqlx::Error>;
    async fn update_user_role(&self, id: Uuid, role: UserRole) -> Result<User, sqlx::Error>;
    async fn update_user_status(&self, id: Uuid, status: UserStatus) -> Result<User, sqlx::Error>;
    async fn update_user_password(
        &self,
        id: Uuid,
        password_hash: String,
    ) -> Result<User, sqlx::Error>;
    async fn verify_email(&self, token: &str) -> Result<(), sqlx::Error>;
    async fn clear_reset_token(&self, token: &str) -> Result<(), sqlx::Error>;
    async fn get_user_by_reset_token(&self, token: &str) -> Result<Option<User>, sqlx::Error>;
    async fn set_reset_token(
        &self,
        id: Uuid,
        token: &str,
        expires: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
    async fn add_verification_token(
        &self,
        id: Uuid,
        token: &str,
        expires: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
    async fn email_exists(&self, email: &str) -> Result<bool, sqlx::Error>;
    async fn phone_exists(&self, phone: &str) -> Result<bool, sqlx::Error>;
    async fn update_verification_token(
        &self,
        email: &str,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient {
    async fn get_user(
        &self,
        id: Option<Uuid>,
        email: Option<&str>,
        token: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        if let Some(v) = id {
            return user_query!("SELECT id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at FROM users WHERE id=$1").bind(v).fetch_optional(&self.pool).await;
        }
        if let Some(v) = email {
            return user_query!("SELECT id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at FROM users WHERE LOWER(email)=LOWER($1)").bind(v).fetch_optional(&self.pool).await;
        }
        if let Some(v) = token {
            return user_query!("SELECT id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at FROM users WHERE verification_token=$1 AND token_expires_at > NOW()").bind(v).fetch_optional(&self.pool).await;
        }
        Ok(None)
    }
    async fn get_users(
        &self,
        page: u32,
        limit: usize,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>, sqlx::Error> {
        user_query!("SELECT id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at FROM users WHERE ($1::user_role IS NULL OR role=$1) AND ($2::user_status IS NULL OR status=$2) ORDER BY created_at DESC LIMIT $3 OFFSET $4").bind(role).bind(status).bind(limit as i64).bind(((page.max(1)-1)*limit as u32) as i64).fetch_all(&self.pool).await
    }
    async fn save_user(
        &self,
        first: &str,
        last: &str,
        email: &str,
        password_hash: &str,
        phone: Option<&str>,
        token: &str,
        expires: DateTime<Utc>,
    ) -> Result<User, sqlx::Error> {
        user_query!("INSERT INTO users(first_name,last_name,email,password_hash,phone_no,verification_token,token_expires_at) VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at").bind(first).bind(last).bind(email).bind(password_hash).bind(phone).bind(token).bind(expires).fetch_one(&self.pool).await
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
    async fn update_user_profile(
        &self,
        id: Uuid,
        first: Option<&str>,
        last: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        user_query!("UPDATE users SET first_name=COALESCE($1,first_name),last_name=COALESCE($2,last_name),updated_at=NOW() WHERE id=$3 RETURNING id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at").bind(first).bind(last).bind(id).fetch_one(&self.pool).await
    }
    async fn update_user_email(&self, id: Uuid, email: &str) -> Result<User, sqlx::Error> {
        user_query!("UPDATE users SET email=$1,is_email_verified=false,updated_at=NOW() WHERE id=$2 RETURNING id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at").bind(email).bind(id).fetch_one(&self.pool).await
    }
    async fn update_user_phone(&self, id: Uuid, phone: &str) -> Result<User, sqlx::Error> {
        user_query!("UPDATE users SET phone_no=$1,is_phone_no_verified=false,updated_at=NOW() WHERE id=$2 RETURNING id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at").bind(phone).bind(id).fetch_one(&self.pool).await
    }
    async fn update_user_role(&self, id: Uuid, role: UserRole) -> Result<User, sqlx::Error> {
        user_query!("UPDATE users SET role=$1,updated_at=NOW() WHERE id=$2 RETURNING id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at").bind(role).bind(id).fetch_one(&self.pool).await
    }
    async fn update_user_status(&self, id: Uuid, status: UserStatus) -> Result<User, sqlx::Error> {
        user_query!("UPDATE users SET status=$1,updated_at=NOW() WHERE id=$2 RETURNING id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at").bind(status).bind(id).fetch_one(&self.pool).await
    }
    async fn update_user_password(
        &self,
        id: Uuid,
        password_hash: String,
    ) -> Result<User, sqlx::Error> {
        user_query!("UPDATE users SET password_hash=$1,updated_at=NOW() WHERE id=$2 RETURNING id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at").bind(password_hash).bind(id).fetch_one(&self.pool).await
    }
    async fn verify_email(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET is_email_verified=true,verification_token=NULL,token_expires_at=NULL,updated_at=NOW() WHERE verification_token=$1 AND token_expires_at>NOW()").bind(token).execute(&self.pool).await?;
        Ok(())
    }
    async fn clear_reset_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password_reset_token=NULL,password_reset_expires_at=NULL,updated_at=NOW() WHERE password_reset_token=$1 AND password_reset_expires_at>NOW()").bind(token).execute(&self.pool).await?;
        Ok(())
    }
    async fn get_user_by_reset_token(&self, token: &str) -> Result<Option<User>, sqlx::Error> {
        user_query!("SELECT id, first_name, last_name, email, password_hash, phone_no, is_email_verified, is_phone_no_verified, verification_token, token_expires_at, role, status, created_at, updated_at FROM users WHERE password_reset_token=$1 AND password_reset_expires_at>NOW()").bind(token).fetch_optional(&self.pool).await
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
    async fn add_verification_token(
        &self,
        id: Uuid,
        token: &str,
        expires: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET verification_token=$1,token_expires_at=$2,updated_at=NOW() WHERE id=$3").bind(token).bind(expires).bind(id).execute(&self.pool).await?;
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
    async fn update_verification_token(
        &self,
        email: &str,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users SET verification_token=$1, token_expires_at=$2, updated_at=NOW() WHERE LOWER(email)=LOWER($3) AND is_email_verified=false"
        )
        .bind(token)
        .bind(expires_at)
        .bind(email)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}