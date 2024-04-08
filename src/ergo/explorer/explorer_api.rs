use ergo_lib::chain::transaction::Transaction;
use ergo_lib::chain::transaction::TxId;
use reqwest::blocking::RequestBuilder;
use reqwest::blocking::Response;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;

use super::error::ExplorerApiError;

pub struct ExplorerApi {
    pub url: url::Url,
}

impl ExplorerApi {
    pub fn new(url: Url) -> Self {
        Self { url }
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
}