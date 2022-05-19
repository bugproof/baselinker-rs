use serde::{Deserialize, Serialize};

pub trait RequestTrait<T> {
    const METHOD: &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseLinkerError {
    #[serde(rename = "error_code")]
    pub code: String,

    #[serde(rename = "error_message")]
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("BaseLinker error")]
    BaseLinkerError(BaseLinkerError),

    #[error("Network error `{0}`")]
    NetworkError(#[from] reqwest::Error),
}
