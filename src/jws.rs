use base64ct::{Base64UrlUnpadded, Encoding};
use serde::{
    Deserialize, Serialize,
    de::{self, IntoDeserializer, value::SeqDeserializer},
};
use std::{borrow::Cow, fmt};

mod encoded_token;
mod signature;

use encoded_token::EncodedToken;
use signature::Signature;

/// JSON Web Signature.
pub struct Jws {
    encoded_token: EncodedToken,
    signature: Signature,
}

impl Jws {
    pub fn new(encoded_token: &str, signature: &[u8]) -> Self {
        let encoded_token = EncodedToken::from(encoded_token);
        let signature = Signature::from(signature);

        Self {
            encoded_token,
            signature,
        }
    }

    /// Returns the base64 encoded header and claims.
    /// This can be deserialized futher into a jwt or
    /// used to verify the signature.
    pub fn encoded_token(&self) -> Cow<'_, str> {
        let ptr: &str = &self.encoded_token;

        Cow::from(ptr)
    }

    /// The decoded signature.
    pub fn signature(&self) -> Cow<'_, [u8]> {
        let ptr: &[u8] = self.signature.as_ref();

        Cow::from(ptr)
    }
}

impl<'de> Deserialize<'de> for Jws {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Jws;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "<header (json via base64)>.<claims (json via base64)>.<signature (base64)>"
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let signature: Signature = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let encoded_token: EncodedToken = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Self::Value {
                    encoded_token,
                    signature,
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts = v.rsplitn(2, '.');

                let de = SeqDeserializer::new(parts);

                self.visit_seq(de)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl Serialize for Jws {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enc_signature: String = Base64UrlUnpadded::encode_string(self.signature.as_ref());

        let enc_token: Cow<str> = self.encoded_token();

        serializer.collect_str(&format_args!("{}.{}", enc_token, enc_signature))
    }
}

impl TryFrom<&str> for Jws {
    type Error = de::value::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let de = value.into_deserializer();

        Self::deserialize(de)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use hmac::Mac;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use sha2;

    type Hmac256 = hmac::Hmac<sha2::Sha256>;

    #[derive(Deserialize, Serialize)]
    struct TestData {
        pub token: Jws,
    }

    #[test]
    fn test_from_str() {
        let valid_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.KMUFsIDTnFmyG3nMiGM6H9FNFUROf3wh7SmqJp-QV30";

        let input = json!({
            "token": valid_token
        })
        .to_string();

        let data: TestData = serde_json::from_str(&input).unwrap();
        let jws: &Jws = &data.token;

        assert_eq!(input, serde_json::to_string(&data).unwrap());

        assert_eq!(
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0".as_bytes(),
            jws.encoded_token().as_bytes()
        );

        assert_eq!(
            [
                40, 197, 5, 176, 128, 211, 156, 89, 178, 27, 121, 204, 136, 99, 58, 31, 209, 77,
                21, 68, 78, 127, 124, 33, 237, 41, 170, 38, 159, 144, 87, 125
            ],
            jws.signature().as_ref()
        );
    }

    #[test]
    fn test_signature_validation() {
        let signed_message = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.KMUFsIDTnFmyG3nMiGM6H9FNFUROf3wh7SmqJp-QV30";

        let secret = "a-string-secret-at-least-256-bits-long";

        let input = json!({
            "token": signed_message
        })
        .to_string();

        let data: TestData = serde_json::from_str(&input).unwrap();
        let jws: Jws = data.token;

        let mut mac = Hmac256::new_from_slice(secret.as_bytes()).unwrap();

        mac.update(jws.encoded_token().as_bytes());

        assert!(mac.verify_slice(&jws.signature()).is_ok())
    }
}
