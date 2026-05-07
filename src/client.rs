use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use hmac::{Hmac, Mac};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest::Client;
use serde::de::DeserializeOwned;
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;

pub const BASE_URL: &str = "https://openapi.liblibai.cloud";

pub struct LiblibClient {
    client: Client,
    access_key: String,
    secret_key: String,
}

impl LiblibClient {
    pub fn new(access_key: impl Into<String>, secret_key: impl Into<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .context("build HTTP client")?;
        Ok(Self {
            client,
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        })
    }

    // ── 签名生成 ──────────────────────────────
    fn sign(&self, endpoint: &str) -> (String, String, String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_millis()
            .to_string();

        let nonce: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()
            .to_lowercase();

        let string_to_sign = format!("{}&{}&{}", endpoint, timestamp, nonce);

        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC key creation");
        mac.update(string_to_sign.as_bytes());
        let sig_bytes = mac.finalize().into_bytes();
        let signature = URL_SAFE_NO_PAD.encode(&sig_bytes);

        (signature, timestamp, nonce)
    }

    fn build_url(&self, endpoint: &str) -> String {
        let (sig, ts, nonce) = self.sign(endpoint);
        format!(
            "{}{}?AccessKey={}&Signature={}&Timestamp={}&SignatureNonce={}",
            BASE_URL,
            endpoint,
            urlencoding::encode(&self.access_key),
            urlencoding::encode(&sig),
            ts,
            nonce
        )
    }

    // ── POST 请求 ─────────────────────────────
    pub async fn post_json<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(endpoint);
        let resp = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .context("HTTP POST")?;

        let status = resp.status();
        let text = resp.text().await.context("read response body")?;

        if !status.is_success() {
            return Err(anyhow!("HTTP {}: {}", status, &text[..text.len().min(500)]));
        }

        let api_resp: ApiResponse<T> =
            serde_json::from_str(&text).context(format!("parse JSON: {}", &text[..text.len().min(300)]))?;

        if !api_resp.is_success() {
            return Err(anyhow!(
                "API error code={}, msg={}",
                api_resp.code,
                api_resp.msg
            ));
        }

        api_resp
            .data
            .ok_or_else(|| anyhow!("API returned empty data"))
    }

    // ── GET 请求 ──────────────────────────────
    pub async fn get_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = self.build_url(endpoint);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("HTTP GET")?;

        let status = resp.status();
        let text = resp.text().await.context("read response body")?;

        if !status.is_success() {
            return Err(anyhow!("HTTP {}: {}", status, &text[..text.len().min(500)]));
        }

        let api_resp: ApiResponse<T> =
            serde_json::from_str(&text).context(format!("parse JSON: {}", &text[..text.len().min(300)]))?;

        if !api_resp.is_success() {
            return Err(anyhow!(
                "API error code={}, msg={}",
                api_resp.code,
                api_resp.msg
            ));
        }

        api_resp
            .data
            .ok_or_else(|| anyhow!("API returned empty data"))
    }
}
