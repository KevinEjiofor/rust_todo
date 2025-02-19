use crate::domain::services::auth_service::AuthService;
use crate::domain::models::user::User;
use crate::data::user_repo::UserRepository;
use async_trait::async_trait;
use argon2::{self, Config};
use chrono::Utc;

pub struct AuthServiceImpl<R: UserRepository> {
    pub user_repo: R,
}

impl<R: UserRepository> AuthServiceImpl<R> {
    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl<R: UserRepository + 'static> AuthService for AuthServiceImpl<R> {
    async fn register(&mut self, username: &str, password: &str) -> Result<User, anyhow::Error> {
        let salt = "randomsalt"; // In production, generate a unique salt per user
        let password_hash = argon2::hash_encoded(password.as_bytes(), salt, &Config::default())?;
        let user = User {
            id: 0, // DB will assign an ID
            username: username.to_owned(),
            password_hash,
        };
        let created = self.user_repo.create(user).await?;
        Ok(created)
    }

    async fn login(&self, username: &str, password: &str) -> Result<String, anyhow::Error> {
        let user = self.user_repo.find_by_username(username).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Verify password using argon2
        if argon2::verify_encoded(&user.password_hash, password.as_bytes())? {
            // Set expiration time (e.g., 24 hours from now)
            let exp = (Utc::now().timestamp() + 24 * 3600) as usize;
            let token = crate::utils::jwt::create_jwt(username, exp)?;
            Ok(token)
        } else {
            Err(anyhow::anyhow!("Invalid credentials"))
        }
    }
}
