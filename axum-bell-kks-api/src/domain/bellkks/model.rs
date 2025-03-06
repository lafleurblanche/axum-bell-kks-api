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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks02 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks03 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks04 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks05 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks06 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks07 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks08 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks09 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks10 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks11 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks12 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks13 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks14 {
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BellKks15 {
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
