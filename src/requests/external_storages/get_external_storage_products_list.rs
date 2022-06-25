use rust_decimal::Decimal;
use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub product_id: String,
    pub ean: String,
    pub sku: String,
    pub name: String,
    pub quantity: i64,
    pub price_brutto: Decimal,
}

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsListResponse {
    pub storage_id: String,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize)]
pub struct GetExternalStorageProductsList {
    pub storage_id: String,
    pub filter_category_id: Option<String>,
    pub filter_sort: Option<String>,
    pub filter_id: Option<String>,
    pub filter_ean: Option<String>,
    pub filter_sku: Option<String>,
    pub filter_name: Option<String>,
    pub filter_price_from: Option<Decimal>,
    pub filter_price_to: Option<Decimal>,
    pub filter_quantity_from: Option<i64>,
    pub filter_quantity_to: Option<i64>,
    pub filter_available: Option<i64>,
    pub page: Option<i64>,
}

impl RequestTrait<GetExternalStorageProductsListResponse> for GetExternalStorageProductsList {
    const METHOD: &'static str = "getExternalStorageProductsList";
}
