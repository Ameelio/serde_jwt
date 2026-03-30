use std::{fmt, ops::Deref};

use serde::{Deserialize, Serialize, de};

#[derive(Clone, Debug, Serialize)]
pub struct EncodedToken(pub Box<str>);

impl Deref for EncodedToken {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<'de> Deserialize<'de> for EncodedToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = EncodedToken;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a base64 encoded string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let token = EncodedToken(Box::from(v));

                Ok(token)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl From<&str> for EncodedToken {
    fn from(value: &str) -> Self {
        Self(Box::from(value))
    }
}
