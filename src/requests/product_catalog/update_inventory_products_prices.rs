use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UpdateInventoryProductsPricesResponse {
    pub counter: i64,
    pub warnings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateInventoryProductsPrices {
    pub inventory_id: i64,
    pub products: HashMap<String, serde_json::Number>
}

impl RequestTrait<UpdateInventoryProductsPricesResponse> for UpdateInventoryProductsPrices {
    const METHOD: &'static str = "updateInventoryProductsPrices";
}
