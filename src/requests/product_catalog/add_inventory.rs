use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddInventoryResponse {
    /// Catalog ID. The list of identifiers can be retrieved using the method getInventories.
    pub inventory_id: i64,
}

/// The method allows you to add the BaseLinker catalogs.
///
/// Adding a catalog with the same identifier again will cause updates of the previously saved catalog.
#[derive(Serialize, Deserialize)]
pub struct AddInventory {
    /// Catalog ID. The list of identifiers can be retrieved using the method getInventories.
    pub inventory_id: Option<i64>,
    /// Catalog name
    pub name: String,
    /// Catalog description
    pub description: String,
    /// An array of languages available in the catalogue.
    pub languages: Vec<String>,
    /// Default catalogue language. Must be included in the "languages" parameter.
    pub default_language: String,
    /// An array of price group identifiers available in the catalogue. The list of price group identifiers can be downloaded using the getInventoryPriceGroups method
    pub price_groups: Vec<i64>,
    /// ID of the price group default for the catalogue. The identifier must be included in the "price_groups" parameter.
    pub default_price_group: i64,
    /// An array of warehouse identifiers available in the catalogue.
    ///
    /// The list of warehouse identifiers can be retrieved using the getInventoryWarehouses API method.
    ///
    /// The format of the identifier should be as follows: "[type:bl|shop|warehouse]_[id:int]". (e.g. "shop_2445")
    pub warehouses: Vec<String>,
    /// Identifier of the warehouse default for the catalogue. The identifier must be included in the "warehouses" parameter.
    pub default_warehouse: String,
    /// Does this catalogue support reservations
    pub reservations: bool,
}

impl RequestTrait<AddInventoryResponse> for AddInventory {
    const METHOD: &'static str = "addInventory";
}
