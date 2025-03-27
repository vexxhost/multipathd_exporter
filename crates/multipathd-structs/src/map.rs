use crate::{
    common::PathDeviceMapperState,
    deserializers::{deferred_failback, optional_string, queueing_value},
    path_group::PathGroup,
};
use serde::Deserialize;

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L214-L231
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MapFailback {
    Immediate,
    FollowOver,
    Manual,

    #[serde(deserialize_with = "deferred_failback")]
    Deferred(i32),

    #[default]
    #[serde(rename = "undef")]
    Undefined,
}

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L181-L190
#[derive(Debug, Default, PartialEq, Deserialize)]
pub enum MapWriteProtection {
    #[serde(rename = "ro")]
    ReadOnly,

    #[serde(rename = "rw")]
    ReadWrite,

    #[default]
    #[serde(rename = "undef")]
    Undefined,
}

#[derive(Debug, PartialEq, Clone)]
pub enum QueueingValue {
    Seconds(u32), // For "X sec" format
    Checks(u32),  // For "X chk" format
}

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L233-L253
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MapQueueing {
    Off,
    On,

    #[serde(deserialize_with = "queueing_value")]
    Timed(QueueingValue),

    #[default]
    #[serde(rename = "-")]
    Undefined,
}

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L181-L190
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MapAction {
    Reject,
    Rename,
    Reload,
    Create,

    #[serde(rename = "switchpg")]
    SwitchPathGroup,

    #[default]
    #[serde(rename = "")]
    None,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Map {
    pub name: String,
    pub uuid: String,

    #[serde(deserialize_with = "optional_string")]
    pub sysfs: Option<String>,

    pub failback: MapFailback,
    pub queueing: MapQueueing,
    pub paths: u64,

    #[serde(rename = "write_prot")]
    pub write_protection: MapWriteProtection,

    #[serde(rename = "dm_st")]
    pub device_mapper_state: PathDeviceMapperState,

    // "features" : "0",
    pub features: String,

    // "hwhandler" : "1 alua",
    pub hwhandler: String,

    pub action: MapAction,
    pub path_faults: u64,

    #[serde(rename = "vend")]
    pub vendor: String,

    #[serde(rename = "prod")]
    pub product: String,

    // "rev" : "8888",
    pub rev: String,

    #[serde(rename = "switch_grp")]
    pub switch_group: u64,

    #[serde(rename = "map_loads")]
    pub loads: u64,

    #[serde(rename = "total_q_time")]
    pub total_queue_time: u64,

    #[serde(rename = "q_timeouts")]
    pub queue_timeouts: u64,

    pub path_groups: Vec<PathGroup>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::PathDeviceMapperState;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    #[case(json!({
        "name" : "3624a9370e92ace0b12e941bf000cb4d5",
        "uuid" : "3624a9370e92ace0b12e941bf000cb4d5",
        "sysfs" : "dm-31",
        "failback" : "immediate",
        "queueing" : "off",
        "paths" : 8,
        "write_prot" : "rw",
        "dm_st" : "active",
        "features" : "0",
        "hwhandler" : "1 alua",
        "action" : "",
        "path_faults" : 23,
        "vend" : "PURE",
        "prod" : "FlashArray",
        "rev" : "8888",
        "switch_grp" : 0,
        "map_loads" : 16,
        "total_q_time" : 0,
        "q_timeouts" : 0,
        "path_groups" : [],
    }), Map {
        name: "3624a9370e92ace0b12e941bf000cb4d5".into(),
        uuid: "3624a9370e92ace0b12e941bf000cb4d5".into(),
        sysfs: Some("dm-31".into()),
        failback: MapFailback::Immediate,
        queueing: MapQueueing::Off,
        paths: 8,
        write_protection: MapWriteProtection::ReadWrite,
        device_mapper_state: PathDeviceMapperState::Active,
        features: "0".into(),
        hwhandler: "1 alua".into(),
        action: MapAction::None,
        path_faults: 23,
        vendor: "PURE".into(),
        product: "FlashArray".into(),
        rev: "8888".into(),
        switch_group: 0,
        loads: 16,
        total_queue_time: 0,
        queue_timeouts: 0,
        path_groups: vec![],
    })]
    #[case(json!({
      "name" : "3624a9370ca38a5c63c724ca8000a5c6d",
      "uuid" : "3624a9370ca38a5c63c724ca8000a5c6d",
      "sysfs" : "dm-0",
      "failback" : "immediate",
      "queueing" : "off",
      "paths" : 8,
      "write_prot" : "rw",
      "dm_st" : "active",
      "features" : "0",
      "hwhandler" : "1 alua",
      "action" : "",
      "path_faults" : 0,
      "vend" : "PURE",
      "prod" : "FlashArray",
      "rev" : "8888",
      "switch_grp" : 0,
      "map_loads" : 0,
      "total_q_time" : 0,
      "q_timeouts" : 0,
      "path_groups": []
    }), Map {
        name: "3624a9370ca38a5c63c724ca8000a5c6d".into(),
        uuid: "3624a9370ca38a5c63c724ca8000a5c6d".into(),
        sysfs: Some("dm-0".into()),
        failback: MapFailback::Immediate,
        queueing: MapQueueing::Off,
        paths: 8,
        write_protection: MapWriteProtection::ReadWrite,
        device_mapper_state: PathDeviceMapperState::Active,
        features: "0".into(),
        hwhandler: "1 alua".into(),
        action: MapAction::None,
        path_faults: 0,
        vendor: "PURE".into(),
        product: "FlashArray".into(),
        rev: "8888".into(),
        switch_group: 0,
        loads: 0,
        total_queue_time: 0,
        queue_timeouts: 0,
        path_groups: vec![],
    })]
    fn test_parsing_map(#[case] input: serde_json::Value, #[case] expected: Map) {
        let result: Map = serde_json::from_value(input).unwrap();
        assert_eq!(result, expected);
    }
}
