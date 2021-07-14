use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use crate::requests::courier_shipments::create_package::{Field, Package};
use std::collections::HashMap;

/// The method allows you to delete a previously created shipment. The method removes the shipment from the BaseLinker system and from the courier system if the courier API allows it
#[derive(Serialize)]
pub struct DeleteCourierPackage {
    /// Courier code
    pub courier_code: String,
    /// Shipment ID, optional if package_number is provided
    pub package_id: Option<i64>,
    /// Shipping number (consignment number), optional if package_id was provided
    pub package_number: Option<String>,
    /// (optional, false by default) Forcing a shipment to be removed from BaseLinker database in the case of an error with the removal of the shipment in the courier API.
    pub force_delete: Option<bool>,
}

impl RequestTrait<serde_json::Value> for DeleteCourierPackage { const METHOD: &'static str = "deleteCourierPackage"; }