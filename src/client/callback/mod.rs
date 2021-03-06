use async_trait::async_trait;
use chrono::Utc;
use hmac::{Hmac, Mac, NewMac};
use http::Method;
use nekoton_utils::TrustMe;
use reqwest::Url;

use crate::models::AccountTransactionEvent;
use crate::prelude::ServiceError;

#[async_trait]
pub trait CallbackClient: Send + Sync {
    async fn send(
        &self,
        url: String,
        payload: AccountTransactionEvent,
        secret: String,
    ) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct CallbackClientImpl {
    client: reqwest::Client,
}

impl CallbackClientImpl {
    pub fn new() -> Self {
        Self {
            client: reqwest::ClientBuilder::new().build().unwrap(),
        }
    }
}

impl Default for CallbackClientImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CallbackClient for CallbackClientImpl {
    async fn send(
        &self,
        url: String,
        payload: AccountTransactionEvent,
        secret: String,
    ) -> Result<(), ServiceError> {
        let nonce = Utc::now().naive_utc().timestamp() * 1000;
        let body = serde_json::to_string(&payload).unwrap_or_default();
        let full_url = Url::parse(&url)
            .map_err(|_| ServiceError::Auth("Url can not be parsed".to_string()))?;
        let sign = calc_sign(body, full_url.path().to_string(), nonce, secret);

        let res = self
            .client
            .request(Method::POST, &url)
            .header("SIGN", sign)
            .header("TIMESTAMP", nonce.to_string())
            .json(&payload)
            .send()
            .await
            .map_err(ServiceError::from)?;

        if res.status() != http::StatusCode::OK {
            Err(ServiceError::Other(anyhow::Error::msg(format!(
                "Received status is not 200. Payload: {:#?}. Receive: {:?}.",
                payload, res
            ))))
        } else {
            Ok(())
        }
    }
}

fn calc_sign(body: String, url: String, timestamp_ms: i64, secret: String) -> String {
    let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes()).trust_me();
    let signing_phrase = format!("{}{}{}", timestamp_ms, url, body);
    mac.update(signing_phrase.as_bytes());
    let hash_result = mac.finalize().into_bytes();
    base64::encode(&hash_result)
}
