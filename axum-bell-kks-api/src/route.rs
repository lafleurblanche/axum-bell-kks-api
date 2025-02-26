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

use crate::handler::bellkks::bellkks03handler::{
    bellkks03_list_handler,
    get_bellkks03_handler,
    get_stacode_bellkks03_handler,
};

use crate::handler::bellkks::bellkks04handler::{
    bellkks04_list_handler,
    get_bellkks04_handler,
    get_stacode_bellkks04_handler,
};

use crate::handler::bellkks::bellkks05handler::{
    bellkks05_list_handler,
    get_bellkks05_handler,
    get_stacode_bellkks05_handler,
};

use crate::handler::bellkks::bellkks06handler::{
    bellkks06_list_handler,
    get_bellkks06_handler,
    get_stacode_bellkks06_handler,
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
        .route("/api/bellkks/bellkks03", get(bellkks03_list_handler))
        .route("/api/bellkks/bellkks03/:id", get(get_bellkks03_handler))
        .route("/api/bellkks/bellkks03/stacode/:sta_code", get(get_stacode_bellkks03_handler))
        .route("/api/bellkks/bellkks04", get(bellkks04_list_handler))
        .route("/api/bellkks/bellkks04/:id", get(get_bellkks04_handler))
        .route("/api/bellkks/bellkks04/stacode/:sta_code", get(get_stacode_bellkks04_handler))
        .route("/api/bellkks/bellkks05", get(bellkks05_list_handler))
        .route("/api/bellkks/bellkks05/:id", get(get_bellkks05_handler))
        .route("/api/bellkks/bellkks05/stacode/:sta_code", get(get_stacode_bellkks05_handler))
        .route("/api/bellkks/bellkks06", get(bellkks06_list_handler))
        .route("/api/bellkks/bellkks06/:id", get(get_bellkks06_handler))
        .route("/api/bellkks/bellkks06/stacode/:sta_code", get(get_stacode_bellkks06_handler))
        .with_state(app_state)
}
