use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddInventoryCategoryResponse {
    /// Number of a category added or updated in BaseLinker storage.
    ///
    /// In an external application you should create a link between the internal number and the number received here.
    ///
    /// It will later be used to update the added category. This number is also used in addProducts and deleteCategory methods.
    pub category_id: i64,
}

/// The method allows you to add a category to the BaseLinker catalog.
///
/// Adding a category with the same identifier again, updates the previously saved category
#[derive(Serialize, Deserialize)]
pub struct AddInventoryCategory {
    /// Catalog ID. The list of identifiers can be retrieved by the getInventories method (inventory_id field).
    ///
    /// To add a category available for all catalogs created in BaseLinker, this field should be omitted.
    pub inventory_id: Option<i64>,
    /// The category identifier to be provided for updates. Should be left blank when creating a new category.
    pub category_id: Option<i64>,
    /// Category name
    pub name: String,
    /// The parent category identifier obtained previously at the output of the addCategory method.
    ///
    /// Categories should be added starting from the hierarchy root so that the child is always added after the parent (you need to know the parent ID to add the child).
    ///
    /// For the top level category, 0 should be given as parent_id.
    pub parent_id: i64,
}

impl RequestTrait<AddInventoryCategoryResponse> for AddInventoryCategory {
    const METHOD: &'static str = "addInventoryCategory";
}
