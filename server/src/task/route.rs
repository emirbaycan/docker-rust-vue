use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    task::handler::{
        create_task_handler, delete_task_handler,
        task_list_handler,
    },
    AppState,
};

pub fn task_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/tasks", get(task_list_handler))
        .route("/tasks", post(create_task_handler))
        .route("/tasks/:id", delete(delete_task_handler))
        .with_state(app_state)
}
