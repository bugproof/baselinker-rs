use rust_decimal::Decimal;
use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddOrderProductResponse {
    /// Identifier of the item added to the order.
    pub order_product_id: i64,
}

/// The method allows you to add a new product to your order.
#[derive(Serialize, Deserialize)]
pub struct AddOrderProduct {
    /// Order Identifier from BaseLinker order manager
    pub order_id: i64,
    /// Type of product source storage (available values: "db" - BaseLinker internal catalog, "shop" - online shop storage, "warehouse" - the connected wholesaler)
    pub storage: String,
    /// ID of the product source storage (one from BaseLinker catalogs or one of the stores connected to the account). Value "0" when the product comes from BaseLinker internal catalog.
    pub storage_id: String,
    /// Product identifier in BaseLinker or shop storage. Blank if the product number is not known
    pub product_id: String,
    /// Product variant ID. Blank if the variant number is unknown
    pub variant_id: String,
    /// Listing ID number (if the order comes from ebay/allegro)
    pub auction_id: String,
    /// Product name
    pub name: String,
    /// Product SKU number
    pub sku: String,
    /// Product EAN number
    pub ean: String,
    /// Product location
    pub location: String,
    /// Product source warehouse identifier. Only applies to products from BaseLinker inventory.
    ///
    /// By default warehouse_id is determined based on the warehouse identifiers in the existing products of the order.
    ///
    /// If no such product exist, it will be determined based on the source of the order
    pub warehouse_id: Option<i64>,
    /// The detailed product attributes, e.g. "Colour: blue" (Variant name)
    pub attributes: String,
    /// Single item gross price
    pub price_brutto: Decimal,
    /// VAT rate
    pub tax_rate: Decimal,
    /// Number of pieces
    pub quantity: i64,
    /// Single piece weight
    pub weight: f64,
}

impl RequestTrait<AddOrderProductResponse> for AddOrderProduct {
    const METHOD: &'static str = "addOrderProduct";
}
