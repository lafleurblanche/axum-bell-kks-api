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

use crate::handler::bellkks::bellkks01handler::{
    bellkks01_list_handler,
    get_bellkks01_handler,
    get_stacode_bellkks01_handler,
};

use crate::handler::bellkks::bellkks02handler::{
    bellkks02_list_handler,
    get_bellkks02_handler,
    get_stacode_bellkks02_handler,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/bellkks/bellkks01", get(bellkks01_list_handler))
        .route("/api/bellkks/bellkks01/:id", get(get_bellkks01_handler))
        .route("/api/bellkks/bellkks01/stacode/:sta_code", get(get_stacode_bellkks01_handler))
        .route("/api/bellkks/bellkks02", get(bellkks02_list_handler))
        .route("/api/bellkks/bellkks02/:id", get(get_bellkks02_handler))
        .route("/api/bellkks/bellkks02/stacode/:sta_code", get(get_stacode_bellkks02_handler))
        .with_state(app_state)
}
