use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ProjectModel {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub imgs: Vec<String>,
    pub demo: String,
    pub git: String,
    pub stacks: Vec<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}