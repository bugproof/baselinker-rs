use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Variant {
    pub variant_id: String,
    pub name: String,
    pub price: String,
    pub quantity: String,
    pub sku: String,
    pub ean: String,
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub product_id: i64,
    pub price_wholesale_netto: f64,
    pub price_netto: f64,
    pub price_brutto: Option<String>,
    pub tax_rate: f64,
    pub name: String,
    pub quantity: i64,
    pub category_id: String,
    pub weight: String,
    pub ean: String,
    pub sku: Option<serde_json::Value>,
    pub description: String,
    pub description_extra1: String,
    pub description_extra2: String,
    pub man_name: String,
    pub images: Vec<String>,
    pub features: Option<HashMap<String, String>>,
    pub variants: Option<Vec<Variant>>,
}

// #[derive(Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum PriceNetto {
//     Double(f64),
//     String(String),
// }
//
// #[derive(Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum PriceWholesaleNetto {
//     Integer(i64),
//     String(String),
// }

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsDataResponse {
    pub storage_id: String,
    pub products: HashMap<String, Product>,
}

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsData {
    pub storage_id: String,
    pub products: Vec<i64>,
}

impl RequestTrait<GetExternalStorageProductsDataResponse> for GetExternalStorageProductsData { const METHOD: &'static str = "getExternalStorageProductsData"; }