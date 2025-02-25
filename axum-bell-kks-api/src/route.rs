use std::sync::Arc;

use axum::{
    routing::{get},
    Router,
};

use crate::{
    AppState,
};

use crate::handler::health::healthhandler::{
    health_checker_handler,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(app_state)
}
