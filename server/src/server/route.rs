use std::sync::Arc;

use axum::{
    middleware,
    routing::get,
    Router,
};

use crate::{
    auth::route::authenticate,
    server::handler::server_info,
    AppState,
};

pub fn server_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/server_info", get(server_info)) 
    .with_state(app_state)
    .layer(middleware::from_fn(authenticate))
}