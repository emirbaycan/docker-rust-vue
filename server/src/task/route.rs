use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    task::handler::{
        create_task_handler, delete_task_handler,
    },
    AppState,
};

use super::handler::{all_tasks_list_handler, create_task_agenda_handler, create_task_group_handler, create_task_supervisor_handler, create_task_update_handler, create_task_visor_handler, delete_task_agenda_handler, delete_task_group_handler, delete_task_supervisor_handler, delete_task_update_handler, delete_task_visor_handler, edit_task_agenda_handler, edit_task_group_handler, edit_task_handler, edit_task_supervisor_handler, edit_task_update_handler, edit_task_visor_handler, task_agenda_list_handler, task_agendas_list_handler, task_groups_list_handler, task_supervisors_list_handler, task_updates_list_handler, task_visors_list_handler, tasks_list_handler};

pub fn task_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/all_tasks", get(all_tasks_list_handler))
        .route("/tasks", post(create_task_handler).get(tasks_list_handler))
        .route("/tasks/:id", delete(delete_task_handler).patch(edit_task_handler))
        .route("/task_agendas", post(create_task_agenda_handler).get(task_agendas_list_handler))
        .route("/task_agendas/:id", get(task_agenda_list_handler).delete(delete_task_agenda_handler).patch(edit_task_agenda_handler))
        .route("/task_groups", post(create_task_group_handler).get(task_groups_list_handler))
        .route("/task_groups/:id", delete(delete_task_group_handler).patch(edit_task_group_handler))
        .route("/task_updates", post(create_task_update_handler).get(task_updates_list_handler))
        .route("/task_updates/:id", delete(delete_task_update_handler).patch(edit_task_update_handler))
        .route("/task_visors", post(create_task_visor_handler).get(task_visors_list_handler))
        .route("/task_visors/:id", delete(delete_task_visor_handler).patch(edit_task_visor_handler))
        .route("/task_supervisors", post(create_task_supervisor_handler).get(task_supervisors_list_handler))
        .route("/task_supervisors/:id", delete(delete_task_supervisor_handler).patch(edit_task_supervisor_handler))
        .with_state(app_state)
}
