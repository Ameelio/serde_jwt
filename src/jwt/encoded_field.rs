use base64ct::{Base64UrlUnpadded, Encoding};
use serde::de;
use std::{fmt, marker};

pub struct EncodedField<T>(pub marker::PhantomData<T>);

impl<'de, T> de::DeserializeSeed<'de> for EncodedField<T>
where
    T: de::DeserializeOwned,
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<T>(marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for Visitor<T>
        where
            T: de::DeserializeOwned,
        {
            type Value = T;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a utf8 encoded byte array of json")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let decoded_bytes: Vec<u8> =
                    Base64UrlUnpadded::decode_vec(v).map_err(de::Error::custom)?;

                serde_json::from_slice(&decoded_bytes).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(Visitor(marker::PhantomData))
    }
}
