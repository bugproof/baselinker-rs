use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to edit the data of selected items (e.g. prices, quantities etc.) of a specific order.
///
/// Only the fields that you want to edit should be given, the remaining fields can be omitted in the request.
#[derive(Serialize, Deserialize)]
pub struct SetOrderProductFields {
    pub order_id: i64,
    pub order_product_id: i64,
    pub storage: String,
    pub storage_id: String,
    pub product_id: String,
    pub variant_id: String,
    pub auction_id: String,
    pub name: String,
    pub sku: String,
    pub ean: String,
    pub location: String,
    pub warehouse_id: i64,
    pub attributes: String,
    pub price_brutto: f64,
    pub tax_rate: f64,
    pub quantity: i64,
    pub weight: f64,
}

impl RequestTrait<IgnoredAny> for SetOrderProductFields {
    const METHOD: &'static str = "setOrderProductFields";
}
