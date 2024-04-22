use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    auth::route::authenticate,
    user::handler::{
        create_user_handler, delete_user_handler, edit_user_handler, get_user_handler,
        user_list_handler, new_user_list_handler, update_user_handler
    },
    AppState,
};

pub fn user_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/new_users", get(new_user_list_handler))
    .route("/users", get(user_list_handler))
    .route("/users", post(create_user_handler))
    .route(
        "/users/:id",
        get(get_user_handler)
        .patch(edit_user_handler)
        .delete(delete_user_handler),
    )
    .with_state(app_state)
    .layer(middleware::from_fn(authenticate))
}

pub fn all_user_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/account", post(update_user_handler))
    .with_state(app_state)
    .layer(middleware::from_fn(authenticate))
}
