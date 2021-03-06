use crate::common::RequestTrait;
use crate::serialization::inconsistent_bool;
use chrono::serde::ts_seconds;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_id: String,
    pub shop_order_id: String,
    pub external_order_id: String,
    pub order_source: String,
    pub order_source_id: String,
    pub order_source_info: String,
    pub order_status_id: String,
    #[serde(with = "ts_seconds")]
    pub date_add: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub date_confirmed: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub date_in_status: DateTime<Utc>,
    pub user_login: String,
    pub phone: String,
    pub email: String,
    pub user_comments: String,
    pub admin_comments: String,
    pub currency: String,
    pub payment_method: String,
    pub payment_method_cod: String,
    pub payment_done: String,
    pub delivery_method: String,
    pub delivery_price: Decimal,
    pub delivery_package_module: String,
    pub delivery_package_nr: String,
    pub delivery_fullname: String,
    pub delivery_company: String,
    pub delivery_address: String,
    pub delivery_city: String,
    pub delivery_postcode: String,
    pub delivery_country: String,
    pub delivery_point_id: String,
    pub delivery_point_name: String,
    pub delivery_point_address: String,
    pub delivery_point_postcode: String,
    pub delivery_point_city: String,
    pub invoice_fullname: String,
    pub invoice_company: String,
    pub invoice_nip: String,
    pub invoice_address: String,
    pub invoice_city: String,
    pub invoice_postcode: String,
    pub invoice_country: String,
    #[serde(deserialize_with = "inconsistent_bool")]
    pub want_invoice: bool,
    pub custom_extra_fields: HashMap<String, serde_json::Value>,
    pub order_page: String,
    pub pick_status: String,
    pub pack_status: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub storage: String,
    pub storage_id: i64,
    pub order_product_id: String,
    pub product_id: String,
    pub variant_id: i64,
    pub name: String,
    pub attributes: String,
    pub sku: String,
    pub ean: String,
    pub location: String,
    pub warehouse_id: i64,
    pub auction_id: String,
    pub price_brutto: Decimal,
    pub tax_rate: Decimal,
    pub quantity: i64,
    pub weight: f64,
    pub bundle_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrdersResponse {
    pub orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOrders {
    pub order_id: Option<i64>,
    #[serde(with = "ts_seconds_option")]
    pub date_confirmed_from: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub date_from: Option<DateTime<Utc>>,
    pub include_custom_extra_fields: Option<bool>,
    pub get_unconfirmed_orders: Option<bool>,
    pub status_id: Option<i64>,
    pub filter_email: Option<String>,
    pub filter_order_source: Option<String>,
    pub filter_order_source_id: Option<String>,
}

impl RequestTrait<GetOrdersResponse> for GetOrders {
    const METHOD: &'static str = "getOrders";
}
