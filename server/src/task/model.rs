use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskModel {
    pub task_id: usize,
    pub group_id: usize,
    pub name: String,
    pub date: String,
    pub expiration_date: String,
    pub status: i16,
    pub priority: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskGroupModel {
    pub group_id: usize,
    pub agenda_id: usize,
    pub title: String,    
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskAgenda {
    pub agenda_id: usize,
    pub title: String, 
    pub user_id: usize,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskUpdate {
    pub update_id: usize,
    pub user_id: usize,
    pub text: String, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskVisor {
    pub visor_id: usize,
    pub task_id: usize,
    pub user_id: usize, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskSupervisor {
    pub supervisor_id: usize,
    pub task_id: usize,
    pub user_id: usize, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}