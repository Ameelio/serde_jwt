use serde::{de, ser};
use std::{error, fmt, result, string::FromUtf8Error};

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    value: Option<String>,
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
enum ErrorKind {
    DecodeError(String),
    EncodeError(String),
    ExpectedByte,
    InvalidEncoding,
    InvalidJwsFormat,
    InvalidLength,
}

impl Error {
    pub fn decoding_error<D, T>(err: T, value: &D) -> Self
    where
        D: fmt::Debug,
        T: fmt::Display,
    {
        let kind = ErrorKind::DecodeError(err.to_string());
        let value = Some(format!("{:?}", value));

        Self { kind, value }
    }

    pub fn expected_byte() -> Self {
        let kind = ErrorKind::ExpectedByte;

        Self { kind, value: None }
    }

    pub fn invalid_encoding<D: fmt::Debug>(value: D) -> Self {
        let kind = ErrorKind::InvalidEncoding;
        let value = Some(format!("{:?}", value));

        Self { kind, value }
    }

    pub fn invalid_jws_format(value: &str) -> Self {
        let kind = ErrorKind::InvalidJwsFormat;
        let value = Some(String::from(value));

        Self { kind, value }
    }
    pub fn invalid_length() -> Self {
        let kind = ErrorKind::InvalidLength;

        Self { kind, value: None }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::EncodeError(msg) | ErrorKind::DecodeError(msg) => write!(f, "{}", msg),
            ErrorKind::ExpectedByte => write!(f, "expected byte"),
            ErrorKind::InvalidJwsFormat => write!(
                f,
                "{} expected to be a JWS in the format of EncodedHeader.EncodedPayload.Signature",
                self.value
                    .as_ref()
                    .expect("An Invalid Jws format will always have a value.")
            ),
            ErrorKind::InvalidEncoding => write!(f, "invalid encoding"),
            ErrorKind::InvalidLength => write!(f, "invalid length"),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        let attempted_bytes: &[u8] = err.as_bytes();
        let value = String::from_utf8_lossy(attempted_bytes);
        Self::invalid_encoding(&value)
    }
}

impl error::Error for Error {}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        let kind = ErrorKind::DecodeError(msg.to_string());

        Self { kind, value: None }
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        let kind = ErrorKind::EncodeError(msg.to_string());

        Self { kind, value: None }
    }
}

impl From<base64ct::Error> for Error {
    fn from(value: base64ct::Error) -> Self {
        match value {
            base64ct::Error::InvalidLength => Self::invalid_length(),
            base64ct::Error::InvalidEncoding => Self::invalid_encoding(""),
        }
    }
}
