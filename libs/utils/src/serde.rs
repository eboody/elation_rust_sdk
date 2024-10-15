use std::fmt::{self, Display};
use time::{format_description, Date, Time};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub fn serialize_option_as_string<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_vec_to_strings<T, S>(value: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    let strings: Vec<String> = value.iter().map(|v| v.to_string()).collect();
    strings.serialize(serializer)
}

pub fn serialize_time<S>(time: &Time, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description::parse("[hour]:[minute]").unwrap();
    let time_str = time.format(&format).unwrap();
    serializer.serialize_str(&time_str)
}

struct TimeVisitor;

impl<'de> Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing time in HH:MM format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let format = format_description::parse("[hour]:[minute]").unwrap();
        Time::parse(v, &format).map_err(de::Error::custom)
    }
}

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(TimeVisitor)
}

pub fn deserialize_date_option<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Date>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_s = Option::<String>::deserialize(deserializer)?;

    // println!("{opt_s:#?}");

    match opt_s {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => {
            let format = format_description::parse("[year]-[month]-[day]").unwrap();
            Date::parse(&s, &format)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}

pub fn deserialize_date<'de, D>(deserializer: D) -> std::result::Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    Date::parse(&s, &format).map_err(serde::de::Error::custom)
}

pub fn serialize_date<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Define the date format
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    // Format the date object into a string
    let date_str = date.format(&format).unwrap();
    // Use the serializer to serialize the string
    serializer.serialize_str(&date_str)
}

pub fn serialize_date_option<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(date) => serialize_date(date, serializer),
        None => serializer.serialize_none(),
    }
}
