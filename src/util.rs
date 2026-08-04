//! Utilities for serialization and deserialization of claims.
//!
//! # Claims that can be a string or an array of strings.
//! `OneOrMore` represents a value that is either:
//!     - `One`: a single string represented as `Box<str>`
//!     - `More`: a list of strings represented as `Box<[Box<str>]>`
//!
//! This is a common claim representation, `aud` for example.
//!
//! ## Example
//!     use json_web_token::util::OneOrMore;
//!     use serde::{Deserialize, Serialize};
//!     use serde_json::json;
//!
//!     #[derive(Deserialize, Serialize)]
//!     struct Claims {
//!         aud: OneOrMore
//!     }
//!
//!     let input = json!({
//!         "aud": "one",
//!     });
//!
//!     let claims : Claims = serde_json::from_value(input).unwrap();
//!
//!     if claims.aud.iter().any(|x| x == "one") {
//!         println!("claim matches");
//!     }
//!
//!     let input = json!({
//!         "aud": ["one", "two"]
//!     });
//!
//!     let claims : Claims = serde_json::from_value(input).unwrap();
//!
//!     if claims.aud.iter().any(|x| x == "one") {
//!         println!("claim matches");
//!     }
//!
//! # Epoch Time
//! `UtcDateTime` is a newtype that wraps around `chrono::DateTime<chrono::Utc>`, it allows for
//! converting from and to the common epoch seconds format used for `iat` and `exp` claims.
//!
//! ## Example
//!     use chrono::{DateTime, TimeDelta, Utc};
//!     use json_web_token::util::epoch::UtcDateTime;
//!     use serde::{Deserialize, Serialize};
//!
//!     #[derive(Deserialize, Serialize)]
//!     struct Claims {
//!         pub exp: UtcDateTime,
//!         pub iat: UtcDateTime,
//!     }
//!
//!     let iat: UtcDateTime = {
//!         let now : DateTime<Utc> = Utc::now();
//!
//!         UtcDateTime(now)
//!     };
//!
//!     let exp: UtcDateTime = iat.clone() + TimeDelta::days(60);
//!
//!     let claims: String  = {
//!
//!         let claims = Claims { exp, iat };
//!
//!         serde_json::to_string(&claims).unwrap()
//!     };
//!
//!     println!("Claims: {}", claims);
//!
//!     let claims : Claims = serde_json::from_str(&claims).unwrap();
//!
//!     let now: DateTime<Utc> = Utc::now();
//!
//!     if claims.iat < claims.exp && claims.exp < now {
//!         println!("claims are valid");
//!     }

pub mod epoch;

pub use one_or_more::OneOrMore;

mod one_or_more;
