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

use crate::handler::bellkks::bellkks11handler::{
    bellkks11_list_handler,
    get_bellkks11_handler,
    get_stacode_bellkks11_handler,
};

use crate::handler::bellkks::bellkks12handler::{
    bellkks12_list_handler,
    get_bellkks12_handler,
    get_stacode_bellkks12_handler,
};

use crate::handler::bellkks::bellkks13handler::{
    bellkks13_list_handler,
    get_bellkks13_handler,
    get_stacode_bellkks13_handler,
};

use crate::handler::bellkks::bellkks14handler::{
    bellkks14_list_handler,
    get_bellkks14_handler,
    get_stacode_bellkks14_handler,
};

use crate::handler::bellkks::bellkks15handler::{
    bellkks15_list_handler,
    get_bellkks15_handler,
    get_stacode_bellkks15_handler,
};

use crate::handler::bellkks::bellkks16handler::{
    bellkks16_list_handler,
    get_bellkks16_handler,
    get_stacode_bellkks16_handler,
};

use crate::handler::bellkks::bellkks17handler::{
    bellkks17_list_handler,
    get_bellkks17_handler,
    get_stacode_bellkks17_handler,
};

use crate::handler::bellkks::bellkks18handler::{
    bellkks18_list_handler,
    get_bellkks18_handler,
    get_stacode_bellkks18_handler,
};

use crate::handler::bellkks::bellkks19handler::{
    bellkks19_list_handler,
    get_bellkks19_handler,
    get_stacode_bellkks19_handler,
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
        .route("/api/bellkks/bellkks11", get(bellkks11_list_handler))
        .route("/api/bellkks/bellkks11/:id", get(get_bellkks11_handler))
        .route("/api/bellkks/bellkks11/stacode/:sta_code", get(get_stacode_bellkks11_handler))
        .route("/api/bellkks/bellkks12", get(bellkks12_list_handler))
        .route("/api/bellkks/bellkks12/:id", get(get_bellkks12_handler))
        .route("/api/bellkks/bellkks12/stacode/:sta_code", get(get_stacode_bellkks12_handler))
        .route("/api/bellkks/bellkks13", get(bellkks13_list_handler))
        .route("/api/bellkks/bellkks13/:id", get(get_bellkks13_handler))
        .route("/api/bellkks/bellkks13/stacode/:sta_code", get(get_stacode_bellkks13_handler))
        .route("/api/bellkks/bellkks14", get(bellkks14_list_handler))
        .route("/api/bellkks/bellkks14/:id", get(get_bellkks14_handler))
        .route("/api/bellkks/bellkks14/stacode/:sta_code", get(get_stacode_bellkks14_handler))
        .route("/api/bellkks/bellkks15", get(bellkks15_list_handler))
        .route("/api/bellkks/bellkks15/:id", get(get_bellkks15_handler))
        .route("/api/bellkks/bellkks15/stacode/:sta_code", get(get_stacode_bellkks15_handler))
        .route("/api/bellkks/bellkks16", get(bellkks16_list_handler))
        .route("/api/bellkks/bellkks16/:id", get(get_bellkks16_handler))
        .route("/api/bellkks/bellkks16/stacode/:sta_code", get(get_stacode_bellkks16_handler))
        .route("/api/bellkks/bellkks17", get(bellkks17_list_handler))
        .route("/api/bellkks/bellkks17/:id", get(get_bellkks17_handler))
        .route("/api/bellkks/bellkks17/stacode/:sta_code", get(get_stacode_bellkks17_handler))
        .route("/api/bellkks/bellkks18", get(bellkks18_list_handler))
        .route("/api/bellkks/bellkks18/:id", get(get_bellkks18_handler))
        .route("/api/bellkks/bellkks18/stacode/:sta_code", get(get_stacode_bellkks18_handler))
        .route("/api/bellkks/bellkks19", get(bellkks19_list_handler))
        .route("/api/bellkks/bellkks19/:id", get(get_bellkks19_handler))
        .route("/api/bellkks/bellkks19/stacode/:sta_code", get(get_stacode_bellkks19_handler))
        .with_state(app_state)
}
