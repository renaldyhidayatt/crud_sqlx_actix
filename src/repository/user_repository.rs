use crate::models::UserModel;
use sqlx::{Error, PgPool, Row};
use uuid::Uuid;

pub struct UserRepository {
    pub db_pool: PgPool,
}

impl UserRepository {
    pub async fn find_by_email_exists(&self, email: &str) -> Result<bool, Error> {
        let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await?
            .get(0);
        Ok(exists)
    }

    pub async fn create_user(
        &self,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> Result<UserModel, Error> {
        let query_result = sqlx::query_as!(
            UserModel,
            "INSERT INTO users (firstname, lastname, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
            firstname,
            lastname,
            email,
            password
        )
        .fetch_one(&self.db_pool)
        .await?;
        Ok(query_result)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, Error> {
        let query_result =
            sqlx::query_as!(UserModel, "SELECT * FROM users WHERE email = $1", email)
                .fetch_optional(&self.db_pool)
                .await?;
        Ok(query_result)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Error> {
        let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(query_result)
    }

    pub async fn update_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: &str,
    ) -> Result<Option<UserModel>, Error> {
        let query_result = sqlx::query_as!(
            UserModel,
            "UPDATE users SET firstname = $1, lastname = $2, password = $3 WHERE email = $4 RETURNING *",
            firstname,
            lastname,
            password,
            email
        )
        .fetch_optional(&self.db_pool)
        .await?;
        Ok(query_result)
    }

    pub async fn delete_user(&self, email: &str) -> Result<bool, Error> {
        let result = sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&self.db_pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
