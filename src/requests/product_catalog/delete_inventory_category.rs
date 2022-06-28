use crate::common::RequestTrait;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The method allows you to remove categories from BaseLinker warehouse.
///
/// Along with the category, the products contained therein are removed (however, this does not apply to products in subcategories).
///
/// The subcategories will be changed to the highest level categories.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteInventoryCategory {
    /// The number of the category to be removed in the BaseLinker storage.
    pub category_id: i64,
}

impl RequestTrait<IgnoredAny> for DeleteInventoryCategory {
    const METHOD: &'static str = "deleteInventoryCategory";
}
