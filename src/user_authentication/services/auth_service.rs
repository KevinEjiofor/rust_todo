use crate::domain::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&mut self, username: &str, password: &str) -> Result<User, anyhow::Error>;
    async fn login(&self, username: &str, password: &str) -> Result<String, anyhow::Error>;
}
