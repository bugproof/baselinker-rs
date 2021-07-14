use serde::{Serialize, Deserialize};
use crate::common::RequestTrait;

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub log_id: i64,
    pub log_type: i64,
    pub order_id: i64,
    pub object_id: i64,
    pub date: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetJournalListResponse {
    pub logs: Vec<Log>,
}

/// The method allows you to download a list of order events.
#[derive(Serialize, Deserialize)]
pub struct GetJournalList {
    pub last_log_id: i64,
    pub logs_types: Vec<i64>,
    pub order_id: Option<i64>
}

impl RequestTrait<GetJournalListResponse> for GetJournalList { const METHOD: &'static str = "getJournalList"; }