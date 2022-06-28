use crate::common::RequestTrait;
use crate::requests::product_catalog::add_inventory_product::ProductLink;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedProductData {
    pub ean: String,
    pub sku: String,
    pub tax_rate: Decimal,
    pub weight: f64,
    pub height: f64,
    pub width: f64,
    pub length: f64,
    pub star: i64,
    pub category_id: i64,
    pub manufacturer_id: i64,
    pub prices: HashMap<i64, Decimal>,
    pub stock: HashMap<String, i64>,
    pub locations: HashMap<String, String>,
    pub text_fields: HashMap<String, String>,
    pub average_cost: Decimal,
    pub average_landed_cost: Decimal,
    pub images: Vec<String>,
    pub links: HashMap<String, ProductLink>,
    pub variants: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryProductsDataResponse {
    pub products: HashMap<String, DetailedProductData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryProductsData {
    pub inventory_id: i64,
    pub products: Vec<String>,
}

impl RequestTrait<GetInventoryProductsDataResponse> for GetInventoryProductsData {
    const METHOD: &'static str = "getInventoryProductsData";
}
