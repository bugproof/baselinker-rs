use crate::common::RequestTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCourierAccountsResponse {
    pub accounts: Vec<Account>,
}

/// The method allows you to retrieve the list of accounts connected to a given courier.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCourierAccounts {
    pub courier_code: String,
}

impl RequestTrait<GetCourierAccountsResponse> for GetCourierAccounts {
    const METHOD: &'static str = "getCourierAccounts";
}
