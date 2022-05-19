use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub category_id: i64,
    pub name: String,
    pub parent_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageCategoriesResponse {
    pub storage_id: String,
    pub categories: Vec<Category>,
}

/// The method allows you to retrieve a category list from an external storage (shop/wholesale) connected to BaseLinker.
#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageCategories {
    /// Storage ID in format "[type:shop|warehouse]_[id:int]" (e.g. "shop_2445").
    pub storage_id: String,
}

impl RequestTrait<GetExternalStorageCategoriesResponse> for GetExternalStorageCategories {
    const METHOD: &'static str = "getExternalStorageCategories";
}
