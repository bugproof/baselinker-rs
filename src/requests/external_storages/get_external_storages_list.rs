use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Deserialize)]
pub struct Storage {
    pub storage_id: String,
    pub name: String,
    pub methods: Vec<String>,
}

#[derive(Deserialize)]
pub struct GetExternalStoragesListResponse {
    pub storages: Vec<Storage>,
}

#[derive(Serialize)]
pub struct GetExternalStoragesList {}

impl RequestTrait<GetExternalStoragesListResponse> for GetExternalStoragesList { const METHOD: &'static str = "getExternalStoragesList"; }