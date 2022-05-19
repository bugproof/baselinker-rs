use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Courier {
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCouriersListResponse {
    pub couriers: Vec<Courier>,
}

/// The method allows you to retrieve a list of available couriers.
#[derive(Serialize, Deserialize)]
pub struct GetCouriersList {}

impl RequestTrait<GetCouriersListResponse> for GetCouriersList {
    const METHOD: &'static str = "getCouriersList";
}
