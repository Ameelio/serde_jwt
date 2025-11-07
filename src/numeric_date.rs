use serde::{Deserialize, de, ser};
use std::time::Duration;

pub struct NumericDate {}

impl NumericDate {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        let value = Duration::new(secs, 0);

        Ok(value)
    }

    pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u64(value.as_secs())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Serialize;

    #[derive(Deserialize, Serialize)]
    struct Header {
        #[serde(with = "NumericDate")]
        issued_at: Duration,
    }

    #[test]
    fn it_serailizes_and_deserializes_json() {
        let input = r#"{"issued_at":1516239022}"#;

        let header: Header = serde_json::from_str(input).unwrap();

        assert_eq!(Duration::new(1516239022, 0), header.issued_at);

        assert_eq!(input, serde_json::to_string(&header).unwrap())
    }
}
