use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize)]
pub struct ReceiptOrderProduct {
    pub name: String,
    pub price_brutto: f64,
    pub tax_rate: f64,
    pub quantity: i64,
    pub sku: String,
    pub ean: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReceiptOrder {
    pub receipt_id: i64,
    pub series_id: i64,
    pub receipt_full_nr: String,
    pub order_id: i64,
    #[serde(with = "ts_seconds")]
    pub date_add: DateTime<Utc>,
    pub payment_method: String,
    pub nip: String,
    pub products: Vec<ReceiptOrderProduct>,
}

#[derive(Serialize, Deserialize)]
pub struct GetNewReceiptsResponse {
    pub orders: Vec<ReceiptOrder>,
}

/// The method allows you to retrieve receipts waiting to be issued.
///
/// This method should be used in creating integration with a fiscal printer.
///
/// The method can be requested for new receipts every e.g. 10 seconds. If any receipts appear in response, they should be confirmed by the setOrderReceipt method after printing to disappear from the waiting list.
#[derive(Serialize, Deserialize)]
pub struct GetNewReceipts {
    /// The numbering series ID allows filtering by the receipt numbering series.
    ///
    /// Using multiple series numbering for receipts is recommended when the user has multiple fiscal printers.
    ///
    /// Each fiscal printer should have a separate series.
    pub series_id: Option<i64>,
}

impl RequestTrait<GetNewReceiptsResponse> for GetNewReceipts { const METHOD: &'static str = "getNewReceipts"; }