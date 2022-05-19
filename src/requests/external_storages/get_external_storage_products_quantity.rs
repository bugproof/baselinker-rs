use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub product_id: String,
    pub quantity: i64,
    pub variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize)]
pub struct Variant {
    pub variant_id: String,
    pub quantity: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsQuantityResponse {
    pub storage_id: String,
    pub products: Vec<Product>,
}

/// The method allows to retrieve stock from an external storage (shop/wholesaler) connected to BaseLinker.
#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsQuantity {
    pub storage_id: String,
    pub page: Option<i64>,
}

impl RequestTrait<GetExternalStorageProductsQuantityResponse>
    for GetExternalStorageProductsQuantity
{
    const METHOD: &'static str = "getExternalStorageProductsQuantity";
}
