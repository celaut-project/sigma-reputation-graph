use ergo_lib::chain::transaction::Transaction;
use ergo_lib::chain::transaction::TxId;
use reqwest::blocking::RequestBuilder;
use reqwest::blocking::Response;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::io::{self, ErrorKind};

use crate::ergo::endpoints::Endpoints;

use super::error::ExplorerApiError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExplorerApi {
    pub url: url::Url,
}

impl ExplorerApi {
    pub fn new() -> Self {
        let endpoint = Endpoints::default();
        Self {
            url: endpoint.explorer_url
        }
    }

    /// Sets required headers for a request
    fn set_req_headers(&self, rb: RequestBuilder) -> RequestBuilder {
        rb.header("accept", "application/json")
            .header(CONTENT_TYPE, "application/json")
    }

    /// Sends a GET request to the Ergo node
    fn send_get_req(&self, endpoint: &str) -> Result<Response, ExplorerApiError> {
        let url = self.url.join(endpoint)?;
        let client = reqwest::blocking::Client::new().get(url);
        let response = self.set_req_headers(client).send()?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(ExplorerApiError::RequestError(
                response.error_for_status()?.error_for_status().unwrap_err(),
            ))
        }
    }

    /// GET /api/v1/transactions/{id}
    pub fn get_transaction_v1(&self, tx_id: TxId) -> Result<Transaction, ExplorerApiError> {
        let endpoint = "/api/v1/transactions/".to_owned() + &tx_id.to_string();
        let response = self.send_get_req(&endpoint)?;
        let text = response.text()?;
        log::debug!("get_transaction_v1 response: {}", text);
        Ok(serde_json::from_str(&text)?)
    }

    // POST /api/v1/boxes/unspend/search/{...}
    pub fn get_unspend_boxes_search(&self, json: Value) -> Result<String, ExplorerApiError> {
        let runtime = tokio::runtime::Runtime::new()?;
        let response = runtime.block_on(async {
            let client = reqwest::Client::new();
            let resp = client
                .post(format!("{}/api/v1/boxes/unspent/search", self.url))
                .json(&json)
                .send()
                .await?;
    
            if resp.status().is_success() {
                resp.text().await.map_err(ExplorerApiError::from)
            } else {
                let error_message = format!("Error: {}", resp.status());
                Err(io::Error::new(io::ErrorKind::Other, error_message).into())
            }
        })?;
        Ok(response)
    }
}