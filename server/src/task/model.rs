use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskModel {
    pub task_id: i32,
    pub group_id: i32,
    pub name: String,
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    pub status: i16,
    pub priority: i16,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskGroupModel {
    pub group_id: i32,
    pub agenda_id: i32,
    pub title: String,    
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskAgenda {
    pub agenda_id: i32,
    pub title: String, 
    pub user_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskUpdate {
    pub update_id: i32,
    pub user_id: i32,
    pub text: String, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskVisor {
    pub visor_id: i32,
    pub task_id: i32,
    pub user_id: i32, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskSupervisor {
    pub supervisor_id: i32,
    pub task_id: i32,
    pub user_id: i32, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}