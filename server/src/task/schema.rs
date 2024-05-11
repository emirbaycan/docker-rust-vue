use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, Default)]
pub struct TaskFilters {
    pub agenda_id: Option<i32>,
}

#[derive(Deserialize, Debug, Default)]
pub struct TaskUpdateFilters {
    pub task_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSchema {
    pub group_id: i32,
    pub name: String,
    pub date: String,
    pub expiration_date: String,
    pub status: i16,
    pub priority: i16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSchema {
    pub group_id: Option<i32>,
    pub name: Option<String>,
    pub date: Option<String>,
    pub expiration_date: Option<String>,
    pub status: Option<i16>,
    pub priority: Option<i16>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskGroupSchema {
    pub agenda_id: i32,
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskGroupSchema {
    pub title: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskAgendaSchema {
    pub title: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskAgendaSchema {
    pub title: Option<String>,
    pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskUpdateSchema {
    pub task_id: i32,
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskUpdateSchema {
    pub text: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskAgendaVisorSchema {
    pub task_id: i32,
    pub email: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskAgendaVisorSchema {
    pub visor_id: Option<i32>,
    pub email: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSupervisorSchema {
    pub task_id: i32,
    pub email: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSupervisorSchema {
    pub supervisor_id: Option<i32>,
    pub email: Option<String>,
}
