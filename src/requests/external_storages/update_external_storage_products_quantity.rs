use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateExternalStorageProductsQuantityResponse {
    pub counter: i64,
    pub warnings: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateExternalStorageProductsQuantity {
    pub storage_id: String,
    pub products: Vec<Vec<i64>>,
}

impl RequestTrait<UpdateExternalStorageProductsQuantityResponse>
    for UpdateExternalStorageProductsQuantity
{
    const METHOD: &'static str = "updateExternalStorageProductsQuantity";
}
