use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Field {
    pub id: String,
    pub value: String,
}

// TODO: use uom crate?

#[derive(Serialize)]
pub struct Package {
    /// centimeters
    pub length: f32,
    /// centimeters
    pub height: f32,
    /// centimeters
    pub width: f32,
    /// kilograms
    pub weight: f32,
    pub size_custom: f32,
}

#[derive(Serialize)]
pub struct CreatePackage {
    pub order_id: i64,
    pub courier_code: String,
    /// Courier API account id for the courier accounts retrieved from the request getCourierAccounts
    ///
    /// If blank, the first account will be used.
    pub account_id: Option<i64>,
    pub fields: Vec<Field>,
    pub packages: Vec<Package>,
}

#[derive(Deserialize)]
pub struct CreatePackageResponse {
    pub package_id: i64,
    pub package_number: String,
    pub courier_inner_number: String,
}

impl RequestTrait<CreatePackageResponse> for CreatePackage { const METHOD: &'static str = "createPackage"; }