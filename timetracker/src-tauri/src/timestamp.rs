use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull, error::BoxDynError, sqlite::SqliteTypeInfo, Database, Decode, Encode, Sqlite,
    Type,
};

#[derive(Debug, Clone, Copy, specta::Type, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(DateTime<Utc>);

impl Type<Sqlite> for Timestamp {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for Timestamp {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        self.0.timestamp_millis().encode(buf)
    }
}

impl<'r> Decode<'r, Sqlite> for Timestamp {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(<i64 as Decode<Sqlite>>::decode(value)?.into())
    }
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Self(DateTime::from_timestamp_millis(value).expect("invalid timestamp"))
    }
}

#[derive(Debug, Clone, Copy, specta::Type, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NullableTimestamp(Option<DateTime<Utc>>);

impl Type<Sqlite> for NullableTimestamp {
    fn type_info() -> SqliteTypeInfo {
        <Option<i64> as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for NullableTimestamp {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as Database>::ArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        self.0.map(|t| t.timestamp_millis()).encode(buf)
    }
}

impl<'r> Decode<'r, Sqlite> for NullableTimestamp {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(<Option<i64> as Decode<Sqlite>>::decode(value)?.into())
    }
}

// impl From<i64> for NullableTimestamp {
//     fn from(value: i64) -> Self {
//         Self(Some(
//             DateTime::from_timestamp_millis(value).expect("invalid timestamp"),
//         ))
//     }
// }

impl From<Option<i64>> for NullableTimestamp {
    fn from(value: Option<i64>) -> Self {
        Self(value.map(|v| DateTime::from_timestamp_millis(v).expect("invalid timestamp")))
    }
}
