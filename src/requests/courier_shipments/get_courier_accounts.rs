use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCourierAccountsResponse {
    pub accounts: Vec<Account>,
}

/// The method allows you to retrieve the list of accounts connected to a given courier.
#[derive(Serialize, Deserialize)]
pub struct GetCourierAccounts {
    pub courier_code: String,
}

impl RequestTrait<GetCourierAccountsResponse> for GetCourierAccounts { const METHOD: &'static str = "getCourierAccounts"; }