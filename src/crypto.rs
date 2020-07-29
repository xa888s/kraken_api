use crate::GenError;
use base64;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};

type HmacSha512 = Hmac<Sha512>;

pub fn get_inner_sign(path: &str, data: String, nonce: u64) -> Result<Vec<u8>, GenError> {
    let nonce = nonce.to_string();
    let input = [nonce, data].concat();
    let bytes = input.as_bytes();

    let hashed: [u8; 32] = Sha256::digest(bytes).into();

    let res = [path.as_bytes(), &hashed].concat();
    Ok(res)
}

pub fn get_sign(key: &str, input: Vec<u8>) -> Result<String, GenError> {
    let key = base64::decode(key)?;
    let mut mac = HmacSha512::new_varkey(&key).unwrap();

    mac.input(&input);
    let result = mac.result();
    let code = result.code().to_vec();
    let res = base64::encode(&code);
    Ok(res)
}
