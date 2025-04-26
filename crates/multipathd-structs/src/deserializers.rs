use serde::Deserialize;

use crate::map::QueueingValue;

pub fn optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    if s == "[undef]" {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

pub fn hex_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let hex_str: String = String::deserialize(deserializer)?;

    // Remove "0x" prefix if present
    let cleaned_hex = hex_str.trim_start_matches("0x");

    // Parse as hexadecimal
    u64::from_str_radix(cleaned_hex, 16)
        .map_err(serde::de::Error::custom)
        .map(Some)
}

pub fn deferred_failback<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    if let Some(num_str) = s.strip_prefix("deferred:") {
        num_str.parse::<i32>().map_err(serde::de::Error::custom)
    } else {
        // For the progress representation format
        Err(serde::de::Error::custom(format!(
            "unexpected format: {}",
            s
        )))
    }
}

pub fn queueing_value<'de, D>(deserializer: D) -> Result<QueueingValue, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    // Handle "X sec" format
    if let Some(num_str) = s.strip_suffix(" sec") {
        return num_str
            .parse::<u32>()
            .map(QueueingValue::Seconds)
            .map_err(|_| serde::de::Error::custom(format!("invalid number in: {}", s)));
    }

    // Handle "X chk" format
    if let Some(num_str) = s.strip_suffix(" chk") {
        return num_str
            .parse::<u32>()
            .map(QueueingValue::Checks)
            .map_err(|_| serde::de::Error::custom(format!("invalid number in: {}", s)));
    }

    Err(serde::de::Error::custom(format!(
        "unexpected format: {}",
        s
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn test_deserialize_optional_string_with_undefined() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "optional_string")]
            field: Option<String>,
        }

        let result: TestStruct = serde_json::from_value(json!({
            "field": "[undef]"
        }))
        .unwrap();

        assert_eq!(result.field, None);
    }

    #[test]
    fn test_deserialize_optional_string_with_value() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "optional_string")]
            field: Option<String>,
        }

        let result: TestStruct = serde_json::from_value(json!({
            "field": "actual value"
        }))
        .unwrap();

        assert_eq!(result.field, Some("actual value".into()));
    }

    #[test]
    fn test_deserialize_optional_string_with_empty_string() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "optional_string")]
            field: Option<String>,
        }

        let result: TestStruct = serde_json::from_value(json!({
            "field": ""
        }))
        .unwrap();

        assert_eq!(result.field, Some("".into()));
    }

    #[test]
    fn test_deserialize_deferred_failback() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "deferred_failback")]
            field: i32,
        }

        let result: TestStruct = serde_json::from_value(json!({
            "field": "deferred:42"
        }))
        .unwrap();

        assert_eq!(result.field, 42);
    }

    #[test]
    fn test_deserialize_deferred_failback_invalid_format() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "deferred_failback")]
            field: i32,
        }

        let result = serde_json::from_value::<TestStruct>(json!({
            "field": "not_deferred"
        }));

        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_deferred_failback_invalid_number() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "deferred_failback")]
            field: i32,
        }

        let result = serde_json::from_value::<TestStruct>(json!({
            "field": "deferred:not_a_number"
        }));

        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_queueing_seconds() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "queueing_value")]
            field: QueueingValue,
        }

        let result: TestStruct = serde_json::from_value(json!({
            "field": "42 sec"
        }))
        .unwrap();

        assert_eq!(result.field, QueueingValue::Seconds(42));
    }

    #[test]
    fn test_deserialize_queueing_checks() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "queueing_value")]
            field: QueueingValue,
        }

        let result: TestStruct = serde_json::from_value(json!({
            "field": "5 chk"
        }))
        .unwrap();

        assert_eq!(result.field, QueueingValue::Checks(5));
    }

    #[test]
    fn test_deserialize_queueing_invalid_format() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "queueing_value")]
            field: QueueingValue,
        }

        let result = serde_json::from_value::<TestStruct>(json!({
            "field": "invalid"
        }));

        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_queueing_invalid_number() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestStruct {
            #[serde(deserialize_with = "queueing_value")]
            field: QueueingValue,
        }

        let result = serde_json::from_value::<TestStruct>(json!({
            "field": "abc sec"
        }));

        assert!(result.is_err());
    }
}
