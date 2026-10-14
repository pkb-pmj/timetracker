use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull, error::BoxDynError, sqlite::SqliteTypeInfo, Database, Decode, Encode, Sqlite,
    Type,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(DateTime<Utc>);

impl specta::Type for Timestamp {
    fn definition(types: &mut specta::Types) -> specta::datatype::DataType {
        use specta::datatype;
        use std::borrow::Cow;
        static SENTINEL: &str = concat!(module_path!(), "::", stringify!(Timestamp));
        fn build(types: &mut specta::Types) -> datatype::DataType {
            <DateTime<Utc> as specta::Type>::definition(types)
        }
        datatype::DataType::Reference(datatype::NamedDataType::init_with_sentinel(
            SENTINEL,
            &[],
            false,
            false,
            types,
            |types, ndt| {
                ndt.name = Cow::Borrowed("Timestamp");
                ndt.module_path = Cow::Borrowed(module_path!());
                ndt.ty = Some(build(types));
            },
            build,
        ))
    }
}

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NullableTimestamp(Option<DateTime<Utc>>);

impl specta::Type for NullableTimestamp {
    fn definition(types: &mut specta::Types) -> specta::datatype::DataType {
        use specta::datatype;
        use std::borrow::Cow;
        static SENTINEL: &str = concat!(module_path!(), "::", stringify!(NullableTimestamp));
        fn build(types: &mut specta::Types) -> datatype::DataType {
            <Option<DateTime<Utc>> as specta::Type>::definition(types)
        }
        datatype::DataType::Reference(datatype::NamedDataType::init_with_sentinel(
            SENTINEL,
            &[],
            false,
            false,
            types,
            |types, ndt| {
                ndt.name = Cow::Borrowed("NullableTimestamp");
                ndt.module_path = Cow::Borrowed(module_path!());
                ndt.ty = Some(build(types));
            },
            build,
        ))
    }
}

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
