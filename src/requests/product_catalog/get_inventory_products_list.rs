use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub ean: String,
    pub sku: String,
    pub name: String,
    pub prices: HashMap<i64, serde_json::Number>,
    pub stock: HashMap<String, i64>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductsListResponse {
    pub products: HashMap<String, Product>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductsList {
    pub inventory_id: i64,
    pub filter_id: Option<i64>,
    pub filter_category_id: Option<i64>,
    pub filter_ean: Option<String>,
    pub filter_sku: Option<String>,
    pub filter_name: Option<String>,
    pub filter_price_from: Option<f64>,
    pub filter_price_to: Option<f64>,
    pub filter_stock_from: Option<i64>,
    pub filter_stock_to: Option<i64>,
    pub page: Option<i64>,
    pub filter_sort: Option<String>,
}

impl RequestTrait<GetInventoryProductsListResponse> for GetInventoryProductsList {
    const METHOD: &'static str = "getInventoryProductsList";
}
