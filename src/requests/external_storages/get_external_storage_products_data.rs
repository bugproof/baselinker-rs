use crate::common::RequestTrait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Variant {
    pub variant_id: String,
    pub name: String,
    pub price: Decimal,
    pub quantity: String,
    pub sku: String,
    pub ean: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub product_id: i64,
    pub price_wholesale_netto: Decimal,
    pub price_netto: Decimal,
    pub price_brutto: Decimal,
    pub tax_rate: Decimal,
    pub name: String,
    pub quantity: i64,
    pub category_id: String,
    pub weight: String,
    pub ean: String,
    pub sku: Option<String>,
    pub description: String,
    pub description_extra1: String,
    pub description_extra2: String,
    pub man_name: String,
    pub images: Vec<String>,
    pub features: Option<HashMap<String, String>>,
    pub variants: Option<Vec<Variant>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub enum PriceNetto {
//     Double(f64),
//     String(String),
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub enum PriceWholesaleNetto {
//     Integer(i64),
//     String(String),
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct GetExternalStorageProductsDataResponse {
    pub storage_id: String,
    pub products: HashMap<String, Product>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetExternalStorageProductsData {
    pub storage_id: String,
    pub products: Vec<i64>,
}

impl RequestTrait<GetExternalStorageProductsDataResponse> for GetExternalStorageProductsData {
    const METHOD: &'static str = "getExternalStorageProductsData";
}
