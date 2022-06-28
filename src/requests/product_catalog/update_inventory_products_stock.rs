use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInventoryProductsStockResponse {
    pub counter: i64,
    pub warnings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInventoryProductsStock {
    pub inventory_id: i64,
    pub products: HashMap<String, i64>,
}

impl RequestTrait<UpdateInventoryProductsStockResponse> for UpdateInventoryProductsStock {
    const METHOD: &'static str = "updateInventoryProductsStock";
}
