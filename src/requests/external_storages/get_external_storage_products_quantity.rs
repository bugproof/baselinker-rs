use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Deserialize)]
pub struct Product {
    pub product_id: String,
    pub quantity: i64,
    pub variants: Vec<Variant>,
}

#[derive(Deserialize)]
pub struct Variant {
    pub variant_id: String,
    pub quantity: i64,
}

#[derive(Deserialize)]
pub struct GetExternalStorageProductsQuantityResponse {
    pub storage_id: String,
    pub products: Vec<Product>,
}

/// The method allows to retrieve stock from an external storage (shop/wholesaler) connected to BaseLinker.
#[derive(Serialize)]
pub struct GetExternalStorageProductsQuantity {
    pub storage_id: String,
    pub page: Option<i64>,
}

impl RequestTrait<GetExternalStorageProductsQuantityResponse> for GetExternalStorageProductsQuantity { const METHOD: &'static str = "getExternalStorageProductsQuantity"; }