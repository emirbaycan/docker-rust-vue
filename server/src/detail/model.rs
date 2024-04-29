use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DetailModel {
    pub id: Uuid,
    pub title: String,
    pub logo: String,
    pub keywords: String,
    pub site_description: String,
    pub description: String,
    pub about: String,
    pub position: String,
    pub company: String,
    pub img: String,
    pub github: String,
    pub linkedin: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
