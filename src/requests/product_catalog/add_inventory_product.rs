use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct AddInventoryProductResponse {
    pub product_id: String,
    pub warnings: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductLink {
    pub product_id: String,
    pub variant_id: Option<String>,
}

/// The method allows you to add a new product to BaseLinker catalog.
///
/// Entering the product with the ID updates previously saved product.
#[derive(Serialize, Deserialize)]
pub struct AddInventoryProduct {
    pub inventory_id: String,
    pub product_id: Option<String>,
    pub parent_id: Option<String>,
    pub is_bundle: bool,
    pub ean: String,
    pub sku: String,
    pub tax_rate: f64,
    pub weight: f64,
    pub height: f64,
    pub width: f64,
    pub length: f64,
    pub star: i64,
    pub manufacturer_id: i64,
    pub category_id: i64,
    pub prices: HashMap<i64, f64>,
    pub stock: HashMap<String, i64>,
    pub locations: HashMap<String, String>,
    pub text_fields: HashMap<String, String>,
    pub images: Vec<String>,
    pub links: HashMap<String, ProductLink>,
    pub bundle_products: HashMap<String, i64>,
}

impl RequestTrait<AddInventoryProductResponse> for AddInventoryProduct { const METHOD: &'static str = "addInventoryProduct"; }