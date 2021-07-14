use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Deserialize)]
pub struct Product {
    pub product_id: String,
    pub price: f64,
    pub variants: Vec<Variant>,
}

#[derive(Deserialize)]
pub struct Variant {
    pub variant_id: String,
    pub price: f64,
}

#[derive(Deserialize)]
pub struct GetExternalStorageProductsPricesResponse {
    pub storage_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize)]
pub struct GetExternalStorageProductsPrices {
    pub storage_id: String,
    pub page: Option<i64>,
}

impl RequestTrait<GetExternalStorageProductsPricesResponse> for GetExternalStorageProductsPrices { const METHOD: &'static str = "getExternalStorageProductsPrices"; }