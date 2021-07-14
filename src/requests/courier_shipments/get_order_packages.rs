use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub package_id: String,
    pub courier_package_nr: String,
    pub courier_inner_number: String,
    pub courier_code: String,
    pub courier_other_name: String,
    pub tracking_status_date: String,
    pub tracking_delivery_days: String,
    pub tracking_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrderPackagesResponse {
    pub packages: Vec<Package>,
}

/// The method allows you to download shipments previously created for the selected order.
#[derive(Serialize, Deserialize)]
pub struct GetOrderPackages {
    pub order_id: String,
}

impl RequestTrait<GetOrderPackagesResponse> for GetOrderPackages { const METHOD: &'static str = "getOrderPackages"; }