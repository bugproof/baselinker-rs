use crate::common::RequestTrait;
use chrono::serde::ts_seconds;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceItem {
    pub name: String,
    pub sku: String,
    pub ean: String,
    pub price_brutto: Decimal,
    pub price_netto: Decimal,
    pub tax_rate: Decimal,
    pub quantity: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub invoice_id: i64,
    pub order_id: i64,
    pub series_id: i64,
    pub r#type: String,
    pub number: String,
    pub sub_id: i64,
    pub month: i64,
    pub year: i64,
    pub postfix: char,
    #[serde(with = "ts_seconds")]
    pub date_add: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub date_sell: DateTime<Utc>,
    /// Due date (unix time format). Not completed by default (value 0)
    #[serde(with = "ts_seconds")]
    pub date_pay_to: DateTime<Utc>,
    pub currency: String,
    pub total_price_brutto: Decimal,
    pub payment: String,
    pub additional_info: String,
    pub invoice_fullname: String,
    pub invoice_company: String,
    pub invoice_nip: String,
    pub invoice_address: String,
    pub invoice_postcode: String,
    pub invoice_city: String,
    pub invoice_country: String,
    pub invoice_country_code: String,
    pub seller: String,
    // TODO: serde flatten instead of all Option?...
    pub correcting_to_invoice_id: Option<i64>,
    pub correcting_reason: Option<String>,
    pub correcting_items: Option<bool>,
    pub correcting_data: Option<bool>,
    pub external_invoice_number: String,
    pub exchange_currency: Option<String>,
    pub exchange_rate: Option<Decimal>,
    pub exchange_date: String, // unknown date format
    pub exchange_info: String,
    pub external_id: i64,
    pub items: Vec<InvoiceItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInvoicesResponse {
    pub invoices: Vec<Invoice>,
}

/// The method allows you to download invoices issued from the BaseLinker order manager.
///
/// The list of invoices can be limited using filters described in the method parameters.
///
/// Maximum 100 invoices are returned at a time.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetInvoices {
    /// Invoice identifier. Completing this field will result in downloading information about only one specific invoice.
    pub invoice_id: Option<i64>,
    /// Order identifier. Completing this field will result in downloading information only about the invoice associated with this order (if the order has an invoice created).
    pub order_id: Option<i64>,
    /// Date from which invoices are to be collected. Unix time stamp format.
    #[serde(with = "ts_seconds_option")]
    pub date_from: Option<DateTime<Utc>>,
    /// The invoice ID number from which subsequent invoices are to be retrieved.
    pub id_from: Option<i64>,
    /// Numbering series ID that allows filtering after the invoice numbering series.
    pub series_id: Option<i64>,
    /// (true by default) Download external invoices as well.
    pub get_external_invoices: Option<bool>,
}

impl RequestTrait<GetInvoicesResponse> for GetInvoices {
    const METHOD: &'static str = "getInvoices";
}
