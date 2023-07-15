use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::UserModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSchema {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<chrono::Utc>>,
}

impl From<UserModel> for UserSchema {
    fn from(user: UserModel) -> Self {
        UserSchema {
            id: user.id,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: UserSchema,
}
