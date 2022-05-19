use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub order_status_id: String,
    #[serde(with = "ts_seconds")]
    pub date_in_status: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub date_add: DateTime<Utc>,
    pub delivery_fullname: String,
    pub delivery_company: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrdersByPhoneResponse {
    pub orders: Vec<Order>,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrdersByPhone {
    pub phone: String,
}

impl RequestTrait<GetOrdersByPhoneResponse> for GetOrdersByPhone {
    const METHOD: &'static str = "getOrdersByPhone";
}
