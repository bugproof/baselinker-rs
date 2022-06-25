use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize)]
pub struct ProductPriceData {
    pub prices: HashMap<i64, Decimal>,
    pub variants: HashMap<String, Decimal>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductsPricesResponse {
    pub products: HashMap<String, ProductPriceData>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryProductsPrices {
    pub inventory_id: i64,
    pub page: Option<i64>,
}

impl RequestTrait<GetInventoryProductsPricesResponse> for GetInventoryProductsPrices {
    const METHOD: &'static str = "getInventoryProductsPrices";
}
