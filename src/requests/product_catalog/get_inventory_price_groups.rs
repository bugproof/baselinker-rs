use crate::common::RequestTrait;
use crate::serialization::inconsistent_bool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InventoryPriceGroup {
    pub price_group_id: i64,
    pub name: String,
    pub description: String,
    pub currency: String,
    #[serde(deserialize_with = "inconsistent_bool")]
    pub is_default: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryPriceGroupsResponse {
    pub price_groups: Vec<InventoryPriceGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryPriceGroups {}

impl RequestTrait<GetInventoryPriceGroupsResponse> for GetInventoryPriceGroups {
    const METHOD: &'static str = "getInventoryPriceGroups";
}
