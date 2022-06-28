use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryWarehouse {
    pub warehouse_type: String,
    pub warehouse_id: i64,
    pub name: String,
    pub description: String,
    pub stock_edition: bool,
    pub is_default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryWarehousesResponse {
    pub warehouses: HashMap<String, InventoryWarehouse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoryWarehouses {}

impl RequestTrait<GetInventoryWarehousesResponse> for GetInventoryWarehouses {
    const METHOD: &'static str = "getInventoryWarehouses";
}
