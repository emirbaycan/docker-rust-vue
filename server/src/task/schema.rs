use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSchema {
    pub title: String,
    pub description: String,
    pub imgs: Vec<String>,
    pub demo: String,
    pub git: String,
    pub stacks: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub imgs: Option<Vec<String>>,
    pub demo: Option<String>,
    pub git: Option<String>,
    pub stacks: Option<Vec<String>>,
}
