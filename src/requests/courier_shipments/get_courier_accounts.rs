use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use crate::requests::courier_shipments::create_package::{Field, Package};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct GetCourierAccountsResponse {
    pub accounts: Vec<Account>,
}

/// The method allows you to retrieve the list of accounts connected to a given courier.
#[derive(Serialize)]
pub struct GetCourierAccounts {
    pub courier_code: String,
}

impl RequestTrait<GetCourierAccountsResponse> for GetCourierAccounts { const METHOD: &'static str = "getCourierAccounts"; }