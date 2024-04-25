use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ServerModel {
    pub total_memory: String,
    pub used_memory: String,
    pub total_swap: String,
    pub used_swap: String,
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
    pub cpu_number: String,
    pub cpu_usages: Vec<String>,
    pub processes: Vec<String>,
    pub networks : Vec<String>,
    pub components : Vec<String>,
}