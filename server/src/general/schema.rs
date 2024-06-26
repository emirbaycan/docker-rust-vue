use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub search: Option<String>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug, Default, sqlx::FromRow)]
pub struct Table {
    pub count: Option<i64>,
}