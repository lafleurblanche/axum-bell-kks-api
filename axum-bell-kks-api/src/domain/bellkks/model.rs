use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks01 {
    pub id: i32,
    #[serde(rename = "routeID")]
    pub route_id: String,
    #[serde(rename = "staCode")]
    pub sta_code: String,
    #[serde(rename = "fromStaCode")]
    pub from_sta_code: String,
    #[serde(rename = "toStaCode")]
    pub to_sta_code: String,
    #[serde(rename = "staName")]
    pub sta_name: String,
}
