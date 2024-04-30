use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSchema {
    pub group_id: usize,
    pub name: String,
    pub date: String,
    pub expiration_date: String,
    pub status: i16,
    pub priority: i16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSchema {
    pub group_id: Option<usize>,
    pub name: Option<String>,
    pub date: Option<String>,
    pub expiration_date: Option<String>,
    pub status: Option<i16>,
    pub priority: Option<i16>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskGroupSchema {
    pub agenda_id: usize,
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskGroupSchema {
    pub agenda_id: Option<usize>,
    pub title: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskAgendaSchema {
    pub title: String,
    pub user_id: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskAgendaSchema {
    pub title: Option<String>,
    pub user_id: Option<usize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskUpdateSchema {
    pub update_id: usize,
    pub user_id: usize,
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskUpdateSchema {
    pub update_id: Option<usize>,
    pub user_id: Option<usize>,
    pub text: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskVisorSchema {
    pub task_id: usize,
    pub user_id: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskVisorSchema {
    pub task_id: Option<usize>,
    pub user_id: Option<usize>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSupervisorSchema {
    pub task_id: usize,
    pub user_id: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSupervisorSchema {
    pub task_id: Option<usize>,
    pub user_id: Option<usize>,
}
