use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// `OffsetDateTime` provides RFC-3339 (ISO-8601 subset) serialization, but the default
/// `serde::Serialize` implementation produces array of integers, whiich is great for binary
/// serialization, but infeasible to consume when returned from an API, and certainly
/// not human-readbale.
///
/// With this wrapper type, we override this to provide the serialization format we want.
#[derive(sqlx::Type)]
pub struct Timestamptz(pub OffsetDateTime);

impl Serialize for Timestamptz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0.format(&Rfc3339).map_err(serde::ser::Error::custom)?)
    }
}

impl<'de> Deserialize<'de> for Timestamptz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;

        impl Visitor<'_> for StrVisitor {
            type Value = Timestamptz;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.pad("expected string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                OffsetDateTime::parse(v, &Rfc3339)
                    .map(Timestamptz)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}
