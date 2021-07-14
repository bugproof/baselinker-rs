use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Deserialize)]
pub struct Courier {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct GetCouriersListResponse {
    pub couriers: Vec<Courier>
}

#[derive(Serialize)]
pub struct GetCouriersList {}

impl RequestTrait<GetCouriersListResponse> for GetCouriersList { const METHOD: &'static str = "getCouriersList"; }