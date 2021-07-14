use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct UpdateExternalStorageProductsQuantityResponse {
    pub counter: i64,
    pub warnings: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExternalStorageProductsQuantity {
    pub storage_id: String,
    pub products: Vec<Vec<i64>>,
}

impl RequestTrait<UpdateExternalStorageProductsQuantityResponse> for UpdateExternalStorageProductsQuantity { const METHOD: &'static str = "updateExternalStorageProductsQuantity"; }