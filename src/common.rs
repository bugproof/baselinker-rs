use serde::{Deserialize};

pub trait RequestTrait<T> {
    const METHOD: &'static str;
}

#[derive(Deserialize)]
pub struct BaseLinkerError {
    #[serde(rename = "error_code")]
    pub code: String,

    #[serde(rename = "error_message")]
    pub message: String,
}