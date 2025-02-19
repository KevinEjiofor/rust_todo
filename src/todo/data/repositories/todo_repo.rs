use crate::domain::models::todo::Todo;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&mut self, todo: Todo) -> Result<Todo, anyhow::Error>;
    async fn get_by_user(&self, user_id: i32) -> Result<Vec<Todo>, anyhow::Error>;
}
