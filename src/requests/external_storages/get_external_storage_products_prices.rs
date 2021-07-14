use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub product_id: String,
    pub price: f64,
    pub variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize)]
pub struct Variant {
    pub variant_id: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsPricesResponse {
    pub storage_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsPrices {
    pub storage_id: String,
    pub page: Option<i64>,
}

impl RequestTrait<GetExternalStorageProductsPricesResponse> for GetExternalStorageProductsPrices { const METHOD: &'static str = "getExternalStorageProductsPrices"; }