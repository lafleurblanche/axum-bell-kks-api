use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::domain::bellkks::model::BellKks06;

use crate::{
    schema::{FilterOptions},
    AppState,
};

pub async fn bellkks06_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(30);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        BellKks06,
        r#"SELECT * FROM bell_kks06 ORDER by id LIMIT $1 OFFSET $2"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all bellkks06 items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let bellkks06list = query_result.unwrap();

    let json_response = serde_json::json!({
        "bellkks06list": bellkks06list
    });
    Ok(Json(json_response))
}

pub async fn get_bellkks06_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(BellKks06, r#"SELECT * FROM bell_kks06 WHERE id = $1"#, id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(bellkks06) => {
            let bellkks06_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "bellkks06": bellkks06
            })});

            return Ok(Json(bellkks06_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("bellkks06 with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn get_stacode_bellkks06_handler(
    Path(sta_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(BellKks06, r#"SELECT * FROM bell_kks06 WHERE sta_code = $1"#, sta_code)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(bellkks06_stacode) => {
            return Ok(Json(bellkks06_stacode));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("bellkks06 with ID: {} not found", sta_code)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}
