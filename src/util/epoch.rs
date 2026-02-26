use std::ops::{Deref, Sub};
use std::{fmt, ops::Add};

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use serde::de::Unexpected;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UtcDateTime(pub DateTime<Utc>);

impl Add<TimeDelta> for UtcDateTime {
    type Output = UtcDateTime;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        let inner: DateTime<Utc> = self.0 + rhs;

        UtcDateTime(inner)
    }
}

impl AsRef<DateTime<Utc>> for UtcDateTime {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Deref for UtcDateTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for UtcDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor)
    }
}

impl PartialEq<DateTime<Utc>> for UtcDateTime {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<DateTime<Utc>> for UtcDateTime {
    fn partial_cmp(&self, other: &DateTime<Utc>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Serialize for UtcDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = self.timestamp();

        serializer.serialize_i64(value)
    }
}

impl Sub for UtcDateTime {
    type Output = TimeDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<DateTime<Utc>> for UtcDateTime {
    type Output = TimeDelta;

    fn sub(self, rhs: DateTime<Utc>) -> Self::Output {
        self.0 - rhs
    }
}

impl Sub<TimeDelta> for UtcDateTime {
    type Output = UtcDateTime;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        let value: DateTime<Utc> = self.0 - rhs;

        UtcDateTime(value)
    }
}

impl PartialEq<UtcDateTime> for DateTime<Utc> {
    fn eq(&self, other: &UtcDateTime) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<UtcDateTime> for DateTime<Utc> {
    fn partial_cmp(&self, other: &UtcDateTime) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl Sub<UtcDateTime> for DateTime<Utc> {
    type Output = TimeDelta;

    fn sub(self, rhs: UtcDateTime) -> Self::Output {
        self - rhs.0
    }
}

struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = UtcDateTime;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an integer between -2^63 and 2^63 that represents epoch seconds")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let Some(v): Option<DateTime<Utc>> = Utc.timestamp_opt(v, 0).single() else {
            return Err(E::invalid_value(Unexpected::Signed(v), &self));
        };

        let v = UtcDateTime(v);

        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v = i64::try_from(v).map_err(|_| E::invalid_value(Unexpected::Unsigned(v), &self))?;

        self.visit_i64(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v: i64 = v
            .parse()
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))?;

        self.visit_i64(v)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_serializes_and_deserializes() {
        #[derive(Deserialize, Serialize)]
        struct Bar {
            foo: UtcDateTime,
        }

        let epoch: i64 = 1772144586;
        let expected: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 2, 26, 22, 23, 6).unwrap();
        let future = expected.clone() + TimeDelta::days(1);

        let input = json!({ "foo": "1772144586" });

        let actual: UtcDateTime = {
            let bar: Bar = serde_json::from_value(input).unwrap();

            bar.foo
        };

        assert_eq!(expected, actual);
        assert!(actual < future);

        let input = json!({ "foo": epoch });

        let actual: UtcDateTime = {
            let bar: Bar = serde_json::from_value(input).unwrap();

            bar.foo
        };

        assert_eq!(expected, actual);
    }
}
