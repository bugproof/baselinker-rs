use crate::common::RequestTrait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInventoryProductsPricesResponse {
    pub counter: i64,
    pub warnings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInventoryProductsPrices {
    pub inventory_id: i64,
    pub products: HashMap<String, Decimal>,
}

impl RequestTrait<UpdateInventoryProductsPricesResponse> for UpdateInventoryProductsPrices {
    const METHOD: &'static str = "updateInventoryProductsPrices";
}
