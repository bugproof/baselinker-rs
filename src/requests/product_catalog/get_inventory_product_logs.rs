use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: verify this

#[derive(Serialize, Deserialize)]
pub enum StockOrPrice {
    Stock(i64),
    Price(serde_json::Number),
}

#[derive(Serialize, Deserialize)]
pub enum StringOrInt {
    String(String),
    Int(i64),
}

#[derive(Serialize, Deserialize)]
pub struct ProductLogEvent {
    pub r#type: i64,
    pub from: StockOrPrice,
    pub to: StockOrPrice,
    pub info: StringOrInt,
}

#[derive(Serialize, Deserialize)]
pub struct ProductLog {
    pub profile: String,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub entries: Vec<ProductLogEvent>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductLogsResponse {
    pub logs: Vec<ProductLog>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductLogs {
    pub product_id: i64,
    #[serde(with = "ts_seconds_option")]
    pub date_from: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub date_to: Option<DateTime<Utc>>,
    pub log_type: Option<i64>,
    pub sort: Option<String>,
    pub page: Option<i64>,
}

impl RequestTrait<GetInventoryProductLogsResponse> for GetInventoryProductLogs {
    const METHOD: &'static str = "getInventoryProductLogs";
}
