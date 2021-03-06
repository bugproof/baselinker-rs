use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    pub storage_id: String,
    pub name: String,
    pub methods: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetExternalStoragesListResponse {
    pub storages: Vec<Storage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetExternalStoragesList {}

impl RequestTrait<GetExternalStoragesListResponse> for GetExternalStoragesList {
    const METHOD: &'static str = "getExternalStoragesList";
}
