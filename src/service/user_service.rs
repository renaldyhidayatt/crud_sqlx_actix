use async_trait::async_trait;
use std::sync::Arc;

use crate::abstract_trait::{DynUserRepository, UserServiceTrait};
use crate::models::UserModel;
use crate::repository::UserRepository;
use sqlx::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    pub repository: DynUserRepository,
}

impl UserService {
    pub fn new(repository: DynUserRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> Result<UserModel, Error> {
        self.repository
            .create_user(firstname, lastname, email, password)
            .await
    }

    async fn find_by_email_exists(&self, email: &str) -> Result<bool, Error> {
        self.repository.find_by_email_exists(email).await
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserModel>, Error> {
        self.repository.find_by_email(email).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        self.repository.find_by_id(id).await
    }

    async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> Result<Option<UserModel>, Error> {
        self.repository
            .update_user(email, firstname, lastname, password)
            .await
    }

    async fn delete_user(&self, email: &str) -> Result<bool, Error> {
        self.repository.delete_user(email).await
    }
}
