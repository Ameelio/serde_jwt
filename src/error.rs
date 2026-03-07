use serde::{de, ser};
use std::{fmt, result, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to convert via base64, {source}")]
    Base64ConversionError {
        #[from]
        source: base64ct::Error,
    },
    #[error("unable to determine order of elements, use sequences or tuples")]
    UnableToDetermineElementOrder,
    #[error("unexpected decoding error, {msg}")]
    UnexpectedDecodingError { msg: Box<str> },
    #[error("unexpected encoding error, {msg}")]
    UnexpectedEncodingError { msg: Box<str> },
    #[error("unexpected type: bool")]
    UnexpectedTypeBool,
    #[error("unexpected type: char")]
    UnexpectedTypeChar,
    #[error("unexpected type: float")]
    UnexpectedTypeFloat,
    #[error("unexpected type: integer")]
    UnexpectedTypeInteger,
    #[error("unexpected type is empty")]
    UnexpectedEmptyType,
    #[error("expected byte")]
    ExpectedByte,
    #[error("{value} is expected to be a utf8 string")]
    InvalidEncoding { value: Box<str> },
    #[error("{value} expected to be a JWS in the format of EncodedHeader.EncodedPayload.Signature")]
    InvalidJwsFormat { value: Box<str> },
    #[error("invalid length")]
    InvalidLength,
}

pub type Result<T> = result::Result<T, Error>;

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        let attempted_bytes: &[u8] = err.as_bytes();
        let value: Box<str> = {
            let value = String::from_utf8_lossy(attempted_bytes);

            value.to_string().into_boxed_str()
        };

        Self::InvalidEncoding { value }
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        let msg: Box<str> = msg.to_string().into_boxed_str();
        Self::UnexpectedDecodingError { msg }
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        let msg: Box<str> = msg.to_string().into_boxed_str();

        Self::UnexpectedEncodingError { msg }
    }
}
