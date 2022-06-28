use crate::common::RequestTrait;
use crate::serialization::inconsistent_bool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub inventory_id: i64,
    pub name: String,
    pub description: String,
    pub languages: Vec<String>,
    pub default_language: String,
    pub price_groups: Vec<i64>,
    pub default_price_group: i64,
    pub warehouses: Vec<String>,
    pub default_warehouse: String,
    #[serde(deserialize_with = "inconsistent_bool")]
    pub reservations: bool,
    #[serde(deserialize_with = "inconsistent_bool")]
    pub is_default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventoriesResponse {
    pub inventories: Vec<Inventory>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInventories {}

impl RequestTrait<GetInventoriesResponse> for GetInventories {
    const METHOD: &'static str = "getInventories";
}
