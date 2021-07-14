use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct PackageHistory {
    pub tracking_status_date: String,
    pub courier_status_code: String,
    pub tracking_status: String,
}

#[derive(Deserialize)]
pub struct GetCourierPackagesStatusHistoryResponse {
    /// Key parcel ID
    pub packages_history: HashMap<String, PackageHistory>,
}

/// The method allows you to retrieve the history of the status list of the given shipments. Maximum 100 shipments at a time
#[derive(Serialize)]
pub struct GetCourierPackagesStatusHistory {
    pub package_ids: Vec<i64>,
}

impl RequestTrait<GetCourierPackagesStatusHistoryResponse> for GetCourierPackagesStatusHistory { const METHOD: &'static str = "getCourierPackagesStatusHistory"; }