use crate::common::RequestTrait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub product_id: String,
    pub price: Decimal,
    pub variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variant {
    pub variant_id: String,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetExternalStorageProductsPricesResponse {
    pub storage_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetExternalStorageProductsPrices {
    pub storage_id: String,
    pub page: Option<i64>,
}

impl RequestTrait<GetExternalStorageProductsPricesResponse> for GetExternalStorageProductsPrices {
    const METHOD: &'static str = "getExternalStorageProductsPrices";
}
