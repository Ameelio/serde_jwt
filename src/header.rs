use base64ct::{Base64UrlUnpadded, Encoding};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Index;

use super::{Algorithm, Result};

#[derive(Deserialize, Serialize)]
pub struct Header {
    #[serde(rename = "alg")]
    algorithm: Algorithm,
    #[serde(flatten)]
    data: BTreeMap<Box<str>, Box<str>>,
}

impl Header {
    pub fn from_base64(raw: &str) -> Result<Self> {
        let dec_bytes: Vec<u8> = Base64UrlUnpadded::decode_vec(raw)?;
        let hdr: Header = serde_json::from_slice(&dec_bytes)?;
        Ok(hdr)
    }

    pub fn algorithm(&self) -> Algorithm {
        self.algorithm.clone()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let boxed_value = self.data.get(key)?;

        Some(boxed_value.as_ref())
    }
}
