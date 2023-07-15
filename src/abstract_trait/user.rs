use std::sync::Arc;

use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::{models::UserModel, response::UserSchema};

pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn find_by_email_exists(&self, email: &str) -> Result<bool, Error>;
    async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> Result<UserModel, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Error>;
    async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> Result<Option<UserModel>, Error>;
    async fn delete_user(&self, email: &str) -> Result<bool, Error>;
}

#[async_trait]
pub trait UserServiceTrait {
    async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<UserSchema>;
    async fn find_by_email_exists(&self, email: &str) -> anyhow::Result<bool>;
    async fn find_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserModel>>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<UserSchema>>;
    async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> anyhow::Result<Option<UserSchema>>;
    async fn delete_user(&self, email: &str) -> anyhow::Result<bool>;
}
