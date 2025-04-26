use crate::{
    common::{MarginalState, PathGroupDeviceMapperState},
    path::Path,
};
use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct PathGroup {
    pub selector: String,

    #[serde(rename = "pri")]
    pub priority: u64,

    #[serde(rename = "dm_st")]
    pub device_mapper_state: PathGroupDeviceMapperState,

    #[serde(rename = "marginal_st")]
    pub marginal_path_state: MarginalState,

    pub group: u64,
    pub paths: Vec<Path>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::MarginalState;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    #[case(json!({
        "selector" : "service-time 0",
        "pri" : 50,
        "dm_st" : "active",
        "marginal_st" : "normal",
        "group" : 1
     }), PathGroup {
        selector: "service-time 0".into(),
        priority: 50,
        device_mapper_state: PathGroupDeviceMapperState::Active,
        marginal_path_state: MarginalState::Normal,
        group: 1,
        paths: vec![],
    })]
    fn test_parsing_path_group(#[case] input: serde_json::Value, #[case] expected: PathGroup) {
        let result: PathGroup = serde_json::from_value(input).unwrap();
        assert_eq!(result, expected);
    }
}
