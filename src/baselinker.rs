use crate::common::{BaseLinkerError, Error, RequestTrait};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::result::Result::Err;

#[derive(Deserialize)]
struct BaseLinkerResponse {
    pub status: String,

    #[serde(flatten)]
    pub error: Option<BaseLinkerError>,
}

pub struct BaseLinkerClient {
    http_client: reqwest::Client,
    token: String,
}

impl BaseLinkerClient {
    pub fn new(token: String, http_client: reqwest::Client) -> Self {
        Self { token, http_client }
    }

    pub async fn send<Request, Response>(&self, request: &Request) -> Result<Response, Error>
    where
        Request: RequestTrait<Response> + Serialize,
        Response: DeserializeOwned,
    {
        let parameters = serde_json::to_string(request).unwrap();

        let mut params = HashMap::new();
        params.insert("method", Request::METHOD);
        params.insert("parameters", parameters.as_str());

        let response = self
            .http_client
            .post("https://api.baselinker.com/connector.php")
            .header("X-BLToken", self.token.as_str())
            .form(&params)
            .send()
            .await?;

        let text = response.text().await.unwrap();

        let api_response = serde_json::from_str::<BaseLinkerResponse>(text.as_str()).unwrap();
        if api_response.status != "SUCCESS" {
            return Err(Error::BaseLinkerError(api_response.error.unwrap()));
        }

        return Ok(serde_json::from_str(text.as_str()).unwrap());
    }
}
