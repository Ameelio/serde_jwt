use serde::{Deserialize, Serialize};

/// JWA supported algorithms.
/// See [IETF RFC 7518](https://datatracker.ietf.org/doc/html/rfc7518#section-3.1)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Algorithm {
    /// HMAC using SHA-256
    HS256,
    /// HMAC using SHA-384
    HS384,
    /// HMAC using SHA-512
    HS512,
    /// RSASSA-PKCS1-v1_5 using SHA-256
    RS256,
    /// RSASSA-PKCS1-v1_5 using SHA-384
    RS384,
    /// RSASSA-PKCS1-v1_5 using SHA-512
    RS512,
    /// ECDSA using P-256 and SHA-256
    ES256,
    /// ECDSA using P-384 and SHA-384
    ES384,
    /// ECDSA using P-512 and SHA-512
    ES512,
    /// RSASSA-PSS and MFG1 using SHA-256
    PS256,
    /// RSASSA-PSS and MFG1 using SHA-384
    PS384,
    /// RSASSA-PSS and MFG1 using SHA-512
    PS512,
    /// No digital signature or MAC
    None,
}

impl Algorithm {
    pub fn len(&self) -> usize {
        match self {
            Self::HS256 | Self::RS256 | Self::ES256 | Self::PS256 => 32,
            Self::HS384 | Self::RS384 | Self::ES384 | Self::PS384 => 48,
            Self::HS512 | Self::RS512 | Self::ES512 | Self::PS512 => 64,
            _ => 0,
        }
    }
}
