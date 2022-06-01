use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ProductStockData {
    pub stock: HashMap<String, i64>,
    pub reservations: HashMap<String, i64>,
    pub variants: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductsStockResponse {
    pub products: HashMap<String, ProductStockData>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductsStock {
    pub inventory_id: i64,
    pub page: Option<i64>,
}

impl RequestTrait<GetInventoryProductsStockResponse> for GetInventoryProductsStock {
    const METHOD: &'static str = "getInventoryProductsStock";
}
