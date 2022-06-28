use crate::common::RequestTrait;
use crate::serialization::inconsistent_bool;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrderResponse {
    pub order_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub storage: String,
    pub storage_id: i64,
    pub product_id: String,
    pub variant_id: i64,
    pub name: String,
    pub sku: String,
    pub ean: String,
    pub location: String,
    pub warehouse_id: i64,
    pub price_brutto: Decimal,
    pub tax_rate: Decimal,
    pub quantity: i64,
    pub weight: f64,
}

/// The method allows adding a new order to the BaseLinker order manager.
#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrder {
    pub order_status_id: String,
    pub source_id: Option<i64>,
    #[serde(with = "ts_seconds")]
    pub date_add: DateTime<Utc>,
    pub user_comments: String,
    pub admin_comments: String,
    pub phone: String,
    pub email: String,
    pub user_login: String,
    pub currency: String,
    pub payment_method: String,
    pub payment_method_cod: String,
    pub paid: String,
    pub delivery_method: String,
    pub delivery_price: Decimal,
    pub delivery_fullname: String,
    pub delivery_company: String,
    pub delivery_address: String,
    pub delivery_city: String,
    pub delivery_postcode: String,
    pub delivery_country_code: String,
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
    pub invoice_country_code: String,
    #[serde(deserialize_with = "inconsistent_bool")]
    pub want_invoice: bool,
    pub custom_extra_fields: HashMap<String, serde_json::Value>,
    pub products: Vec<Product>,
}

impl RequestTrait<AddOrderResponse> for AddOrder {
    const METHOD: &'static str = "addOrder";
}
