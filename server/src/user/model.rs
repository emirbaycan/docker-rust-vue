use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub fullname: String,
    pub role: i16,
    pub avatar: String,
    pub notes: String,
    pub active: i16,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}