use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskModel {
    pub task_id: i32,
    pub group_id: i32,
    pub name: String,
    pub date: String,
    pub expiration_date: String,
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
pub struct TaskAgendaModel {
    pub agenda_id: i32,
    pub title: String, 
    pub user_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskUpdateModel {
    pub update_id: i32,
    pub task_id: i32,
    pub user_id: i32,
    pub text: String, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DisplayTaskUpdateModel {
    pub update_id: i32,
    pub task_id: i32,
    pub user_id: i32,
    pub text: String, 
    pub email: String, 
    pub fullname: String, 
    pub avatar: String, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskVisorModel {
    pub visor_id: i32,
    pub task_id: i32,
    pub user_id: i32, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DisplayTaskVisorModel {
    pub visor_id: i32,
    pub task_id: i32,
    pub user_id: i32, 
    pub email: String, 
    pub fullname: String, 
    pub avatar: String, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskSupervisorModel {
    pub supervisor_id: i32,
    pub task_id: i32,
    pub user_id: i32, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DisplayTaskSupervisorModel {
    pub supervisor_id: i32,
    pub task_id: i32,
    pub user_id: i32, 
    pub email: String, 
    pub fullname: String, 
    pub avatar: String, 
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}