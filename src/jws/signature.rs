use base64ct::{Base64UrlUnpadded, Encoding};
use serde::{Deserialize, de};
use std::fmt;

pub struct Signature(pub Box<[u8]>);

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Signature;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a base64 encoded string")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let signature = Signature(Box::from(v));

                Ok(signature)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let decoded_bytes: Vec<u8> =
                    Base64UrlUnpadded::decode_vec(v).map_err(de::Error::custom)?;

                self.visit_bytes(&decoded_bytes)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl From<&[u8]> for Signature {
    fn from(value: &[u8]) -> Self {
        Self(Box::from(value))
    }
}
