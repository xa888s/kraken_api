use crate::crypto;
use crate::error::KrakenError;
use crate::json;
use crate::json::Value;
use crate::log::info;
use crate::GenError;
use async_trait::async_trait;
use boringauth::oath::TOTPBuilder;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[async_trait]
trait SecureExchange {
    const TOKEN_URL: &'static str;
}

#[derive(Deserialize, Serialize)]
struct KrakenResponse {
    pub result: Option<String>,
    pub error: Option<Vec<String>>,
}

impl KrakenResponse {
    pub fn to_result(self) -> Result<String, KrakenError> {
        // match on errors (ideally None ;))
        match (self.result, &self.error) {
            (Some(res), None) => Ok(res),
            _ => Err(self.error.unwrap().into()),
        }
    }
}

pub struct Kraken {
    token: String,
    sign: String,
    nonce: u64,
    secret: String,
    key: String,
    totp: String,
}

impl Kraken {
    pub fn new(key: String, secret: String, totp: String) -> Self {
        Kraken {
            key,
            secret,
            totp,
            sign: String::new(),
            nonce: Kraken::get_nonce(),
            token: String::new(),
        }
    }

    pub async fn start(mut self) -> Result<Self, GenError> {
        let inner_sign = crypto::get_inner_sign(Self::TOKEN_URL, self.get_json()?, self.nonce)?;
        self.sign = crypto::get_sign(&self.key, inner_sign)?;

        self.get_token().await?;
        info!("Set token to: {}", &self.token);
        Ok(self)
    }

    // TODO: fix invalid signature response
    async fn get_token(&mut self) -> Result<(), GenError> {
        let post_json = self.get_json()?;

        let res = self.get_res(post_json).await?.to_result()?;

        self.token = res;
        Ok(())
    }

    async fn get_res(&self, json: Value) -> Result<KrakenResponse, GenError> {
        let res = surf::post(Self::TOKEN_URL)
            .set_header("API-Key", &self.secret)
            .set_header("API-Sign", &self.sign)
            .body_json(&json)?
            .await?
            .body_json()
            .await?;
        Ok(res)
    }

    fn get_json(&self) -> Result<Value, GenError> {
        info!("Creating output json");
        Ok(json!({
            "nonce": self.nonce,
            "otp": self.get_auth(),
        }))
    }

    fn get_auth(&self) -> String {
        TOTPBuilder::new()
            .base32_key(&self.totp)
            .finalize()
            .unwrap()
            .generate()
    }

    fn get_nonce() -> u64 {
        let start = SystemTime::now();
        start
            .duration_since(UNIX_EPOCH)
            .expect("Your time is screwed up!")
            .as_millis() as u64
    }
}

impl SecureExchange for Kraken {
    const TOKEN_URL: &'static str = "https://api.kraken.com/0/private/GetWebSocketsToken";
}
