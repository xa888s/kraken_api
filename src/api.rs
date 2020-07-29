use crate::crypto;
use crate::error::KrakenError;
use crate::log::info;
use crate::GenError;
use async_trait::async_trait;
use boringauth::oath::TOTPBuilder;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[async_trait]
trait SecureExchange {
    const BASE_URL: &'static str;
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

#[derive(Deserialize, Serialize)]
struct FormData {
    nonce: u64,
    otp: String,
}

pub struct Kraken {
    token: Option<String>,
    sign: Option<String>,
    nonce: u64,
    secret: String,
    key: String,
    totp: String,
}

impl Kraken {
    const TOKEN_PATH: &'static str = "/private/GetWebSocketsToken";
    const API_VERSION: &'static str = "/0";

    pub fn new(key: String, secret: String, totp: String) -> Self {
        info!("Set key to: {}", &key);
        info!("Set secret to: {}", &secret);
        info!("Set totp to: {}", &totp);
        Kraken {
            key,
            secret,
            totp,
            sign: None,
            nonce: Kraken::get_nonce(),
            token: None,
        }
    }

    pub async fn start(mut self) -> Result<Self, GenError> {
        let inner_sign = crypto::get_inner_sign(
            Self::TOKEN_PATH,
            serde_urlencoded::to_string(self.get_formdata())?,
            self.nonce,
        )?;
        self.sign = Some(crypto::get_sign(&self.key, inner_sign)?);

        self.get_token().await?;
        info!("Set token to: {}", &self.token.as_ref().unwrap());
        Ok(self)
    }

    // TODO: fix invalid signature response
    async fn get_token(&mut self) -> Result<(), GenError> {
        let post_data = self.get_formdata();

        let res = self.get_res(post_data).await?.to_result()?;

        self.token = Some(res);
        Ok(())
    }

    async fn get_res(&self, data: FormData) -> Result<KrakenResponse, GenError> {
        let res = surf::post([Self::BASE_URL, Self::API_VERSION, Self::TOKEN_PATH].concat())
            .set_header("API-Key", &self.secret)
            .set_header("API-Sign", &self.sign.as_ref().unwrap())
            .body_form(&data)?
            .await?
            .body_json()
            .await?;
        Ok(res)
    }

    fn get_formdata(&self) -> FormData {
        info!("Creating output formdata");
        FormData {
            nonce: self.nonce,
            otp: self.get_auth(),
        }
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
    const BASE_URL: &'static str = "https://api.kraken.com";
}
