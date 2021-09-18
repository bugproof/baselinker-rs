use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct AddInvoiceResponse {
    pub invoice_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AddInvoice {
    pub order_id: i64,
    pub series_id: i64,
}

impl RequestTrait<AddInvoiceResponse> for AddInvoice { const METHOD: &'static str = "getOrdersByPhone"; }