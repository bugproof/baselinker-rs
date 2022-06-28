use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub id: String,
    pub value: String,
}

// TODO: use uom crate?

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    /// centimeters
    pub length: f64,
    /// centimeters
    pub height: f64,
    /// centimeters
    pub width: f64,
    /// kilograms
    pub weight: f64,
    pub size_custom: f64,
}

/// The method allows you to create a shipment in the system of the selected courier.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePackage {
    /// Order identifier
    pub order_id: i64,
    /// Courier code
    pub courier_code: String,
    /// Courier API account id for the courier accounts retrieved from the request getCourierAccounts
    ///
    /// If blank, the first account will be used.
    pub account_id: Option<i64>,
    /// List of form fields retrieved from the request getCourierFields
    pub fields: Vec<Field>,
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePackageResponse {
    /// Shipment ID
    pub package_id: i64,
    /// Shipping number (consignment number)
    pub package_number: String,
    /// Courier internal number
    pub courier_inner_number: String,
}

impl RequestTrait<CreatePackageResponse> for CreatePackage {
    const METHOD: &'static str = "createPackage";
}
