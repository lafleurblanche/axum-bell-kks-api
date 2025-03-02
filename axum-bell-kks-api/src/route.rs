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

use crate::handler::bellkks::bellkks07handler::{
    bellkks07_list_handler,
    get_bellkks07_handler,
    get_stacode_bellkks07_handler,
};

use crate::handler::bellkks::bellkks08handler::{
    bellkks08_list_handler,
    get_bellkks08_handler,
    get_stacode_bellkks08_handler,
};

use crate::handler::bellkks::bellkks09handler::{
    bellkks09_list_handler,
    get_bellkks09_handler,
    get_stacode_bellkks09_handler,
};

use crate::handler::bellkks::bellkks10handler::{
    bellkks10_list_handler,
    get_bellkks10_handler,
    get_stacode_bellkks10_handler,
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
        .route("/api/bellkks/bellkks07", get(bellkks07_list_handler))
        .route("/api/bellkks/bellkks07/:id", get(get_bellkks07_handler))
        .route("/api/bellkks/bellkks07/stacode/:sta_code", get(get_stacode_bellkks07_handler))
        .route("/api/bellkks/bellkks08", get(bellkks08_list_handler))
        .route("/api/bellkks/bellkks08/:id", get(get_bellkks08_handler))
        .route("/api/bellkks/bellkks08/stacode/:sta_code", get(get_stacode_bellkks08_handler))
        .route("/api/bellkks/bellkks09", get(bellkks09_list_handler))
        .route("/api/bellkks/bellkks09/:id", get(get_bellkks09_handler))
        .route("/api/bellkks/bellkks09/stacode/:sta_code", get(get_stacode_bellkks09_handler))
        .route("/api/bellkks/bellkks10", get(bellkks10_list_handler))
        .route("/api/bellkks/bellkks10/:id", get(get_bellkks10_handler))
        .route("/api/bellkks/bellkks10/stacode/:sta_code", get(get_stacode_bellkks10_handler))
        .with_state(app_state)
}
