use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AmazonFulfillmentShipment {
    pub product_name: String,
    pub product_sku: String,
    pub quantity: i64,
    pub fba: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderTransactionDetailsResponse {
    pub amazon_fulfillment_shipments: Vec<AmazonFulfillmentShipment>,
    #[serde(with = "ts_seconds")]
    pub amazon_ship_date_from: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub amazon_ship_date_to: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub amazon_delivery_date_from: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub amazon_delivery_date_to: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrderTransactionDetails {
    pub order_id: i64,
}

impl RequestTrait<GetOrderTransactionDetailsResponse> for GetOrderTransactionDetails {
    const METHOD: &'static str = "getOrderTransactionDetails";
}
