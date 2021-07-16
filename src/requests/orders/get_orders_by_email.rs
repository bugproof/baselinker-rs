use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub order_status_id: String,
    #[serde(with = "ts_seconds")]
    pub date_in_status: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub date_add: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrdersByEmailResponse {
    pub orders: Vec<Order>,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrdersByEmail {
    pub email: String,
}

impl RequestTrait<GetOrdersByEmailResponse> for GetOrdersByEmail { const METHOD: &'static str = "getOrdersByEmail"; }