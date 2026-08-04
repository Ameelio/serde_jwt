mod encoded_field;

use std::fmt;
use std::marker::PhantomData;

use base64ct::{Base64UrlUnpadded, Encoding};
use serde::de::IntoDeserializer;
use serde::de::value::SeqDeserializer;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize, de, ser};

use crate::error::Error;
use crate::ser::Serializer;

use encoded_field::EncodedField;

/// The JSON Web Token
/// This uses generics, for the claims and header, all that is required is that they are structs which implement [Deserialize](https://serde.rs/).
/// See: [What is a JSON Web Token](https://www.jwt.io/introduction#what-is-json-web-token)
#[derive(PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Jwt<C, H> {
    claims: C,
    header: H,
}

impl<C, H> Jwt<C, H>
where
    C: de::DeserializeOwned + Sized,
    H: de::DeserializeOwned + Sized,
{
    pub fn new(claims: C, header: H) -> Self {
        Self { claims, header }
    }

    pub fn claims(&self) -> &C {
        let claims = &self.claims;
        claims
    }

    pub fn header(&self) -> &H {
        let header = &self.header;
        header
    }
}

impl<Claims, Header> Jwt<Claims, Header>
where
    Claims: Serialize,
    Header: Serialize,
{
    pub fn to_string(&self) -> Result<String, Error> {
        let mut serializer = Serializer {
            output: String::new(),
        };

        self.serialize(&mut serializer)?;

        Ok(serializer.output)
    }
}

impl<'de, C, H> Deserialize<'de> for Jwt<C, H>
where
    C: de::DeserializeOwned,
    H: de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<C, H>(PhantomData<(C, H)>);

        impl<'de, C, H> de::Visitor<'de> for Visitor<C, H>
        where
            C: de::DeserializeOwned,
            H: de::DeserializeOwned,
        {
            type Value = Jwt<C, H>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expecting base64 encoded strings joined with a '.'")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let header: H = seq
                    .next_element_seed(EncodedField(PhantomData))?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let claims: C = seq
                    .next_element_seed(EncodedField(PhantomData))?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Jwt { header, claims })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts = v.split('.');

                let de = SeqDeserializer::new(parts);

                self.visit_seq(de)
            }
        }

        let visitor = Visitor(PhantomData);

        deserializer.deserialize_str(visitor)
    }
}

impl<C, H> Serialize for Jwt<C, H>
where
    C: Serialize,
    H: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // NOTE: Its possible there may be some optimization writing a base64 serializer
        // so I can then have serde transcode from json to base64 without allocating as
        // much.

        let ser_claims: String = {
            let ser_claims: Vec<u8> =
                serde_json::to_vec(&self.claims).map_err(ser::Error::custom)?;

            Base64UrlUnpadded::encode_string(&ser_claims)
        };

        let ser_header: String = {
            let ser_header: Vec<u8> =
                serde_json::to_vec(&self.header).map_err(ser::Error::custom)?;

            Base64UrlUnpadded::encode_string(&ser_header)
        };

        serializer.collect_str(&format_args!("{}.{}", ser_header, ser_claims))
    }
}

impl<C, H> TryFrom<&str> for Jwt<C, H>
where
    C: de::DeserializeOwned,
    H: de::DeserializeOwned,
{
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let de = value.into_deserializer();

        Self::deserialize(de)
    }
}

#[cfg(test)]
mod test {
    use serde_test::{Token, assert_tokens};
    use std::time;

    use super::*;
    use serde::{Deserialize, Serialize};

    use crate::numeric_date::NumericDate;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct TestHeader {
        alg: Box<str>,
        typ: Box<str>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct TestClaims {
        #[serde(alias = "sub")]
        user_id: Box<str>,
        name: Box<str>,
        admin: bool,
        #[serde(alias = "iat", with = "NumericDate")]
        issued_at: time::Duration,
    }

    #[cfg(feature = "debug")]
    #[test]
    fn test_ser_de() {
        let header = TestHeader {
            alg: Box::from("HS256"),
            typ: Box::from("JWT"),
        };
        let claims = TestClaims {
            admin: false,
            issued_at: time::Duration::new(1516239022, 0),
            name: Box::from("John Doe"),
            user_id: Box::from("1234567890"),
        };

        let jwt = Jwt { header, claims };

        assert_tokens(
            &jwt,
            &[Token::Str(
                "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoiMTIzNDU2Nzg5MCIsIm5hbWUiOiJKb2huIERvZSIsImFkbWluIjpmYWxzZSwiaXNzdWVkX2F0IjoxNTE2MjM5MDIyfQ",
            )],
        );
    }
}
