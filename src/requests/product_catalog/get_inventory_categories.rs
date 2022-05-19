use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InventoryCategory {
    pub category_id: i64,
    pub name: String,
    pub parent_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryCategoriesResponse {
    pub categories: Vec<InventoryCategory>,
}

#[derive(Serialize, Deserialize)]
pub struct GetInventoryCategories {
    pub inventory_id: i64,
}

impl RequestTrait<GetInventoryCategoriesResponse> for GetInventoryCategories {
    const METHOD: &'static str = "getInventoryCategories";
}
