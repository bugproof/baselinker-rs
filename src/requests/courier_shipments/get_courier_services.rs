use crate::common::RequestTrait;
use crate::requests::courier_shipments::create_package::{Field, Package};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCourierServicesResponse {
    /// List of available services.
    ///
    /// The key to each element is the service id (varchar)
    ///
    /// Value is the name of the service (varchar)
    pub services: HashMap<String, String>,
}

// A copy of createPackage...

/// The method allows you to retrieve additional courier services, which depend on other shipment settings.
///
/// Used only for X-press and BrokerSystem couriers.
///
/// Not applicable to other couriers whose forms have fixed options.
///
/// The details of the package should be sent with the method (the format as in createPackage) in order to receive a list of additional services
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCourierServices {
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

impl RequestTrait<GetCourierServicesResponse> for GetCourierServices {
    const METHOD: &'static str = "getCourierServices";
}
