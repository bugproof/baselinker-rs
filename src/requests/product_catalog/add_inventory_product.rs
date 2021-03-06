use crate::common::RequestTrait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddInventoryProductResponse {
    pub product_id: i64,
    pub warnings: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductLink {
    pub product_id: String,
    pub variant_id: Option<String>,
}

/// The method allows you to add a new product to BaseLinker catalog.
///
/// Entering the product with the ID updates previously saved product.
#[derive(Serialize, Deserialize, Debug)]
pub struct AddInventoryProduct {
    pub inventory_id: i64,
    pub product_id: Option<String>,
    pub parent_id: Option<String>,
    pub is_bundle: bool,
    pub bundle_products: Option<HashMap<String, i64>>,
    pub ean: String,
    pub sku: String,
    pub tax_rate: Decimal,
    pub weight: f64,
    pub height: f64,
    pub width: f64,
    pub length: f64,
    pub star: i64,
    pub manufacturer_id: i64,
    pub category_id: i64,
    pub prices: HashMap<i64, Decimal>,
    pub stock: HashMap<String, i64>,
    pub locations: HashMap<String, String>,
    pub text_fields: HashMap<String, String>,
    pub images: Vec<String>,
    pub links: HashMap<String, ProductLink>,
}

impl RequestTrait<AddInventoryProductResponse> for AddInventoryProduct {
    const METHOD: &'static str = "addInventoryProduct";
}
