use crate::{
    common::{DeviceState, MarginalState, PathDeviceMapperState},
    deserializers::{hex_string, optional_string},
};
use serde::Deserialize;
use strum_macros::{Display, EnumIter};

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/checkers.h#L84-L91
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PathChecker {
    #[serde(rename = "directio")]
    DirectIO,

    #[serde(rename = "tur")]
    TestUnitReady,

    #[serde(rename = "hp_sw")]
    HpServiceGuard,

    #[serde(rename = "rdac")]
    RDAC,

    #[serde(rename = "emc_clariion")]
    EmcClariion,

    #[serde(rename = "readsector0")]
    ReadSector0,

    #[serde(rename = "cciss_tur")]
    HpSmartArrayTestUnitReady,

    #[serde(rename = "none")]
    None,

    #[default]
    #[serde(other)]
    Invalid,
}

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L523-L547
#[derive(Clone, Debug, Default, Display, PartialEq, Deserialize, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum PathCheckerState {
    Ready,
    Faulty,
    Shaky,
    Ghost,
    Delayed,

    #[serde(rename = "i/o pending")]
    IoPending,

    #[serde(rename = "i/o timeout")]
    IoTimeout,

    #[default]
    #[serde(rename = "undef")]
    Undefined,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Path {
    #[serde(rename = "dev")]
    pub device_name: String,

    #[serde(rename = "dev_t")]
    pub device_major_minor: String,

    #[serde(rename = "dm_st")]
    pub device_mapper_state: PathDeviceMapperState,

    #[serde(rename = "dev_st")]
    pub device_state: DeviceState,

    #[serde(rename = "checker")]
    pub checker: PathChecker,

    #[serde(rename = "chk_st")]
    pub checker_state: PathCheckerState,

    #[serde(rename = "pri")]
    pub priority: u64,

    #[serde(rename = "host_wwnn")]
    #[serde(deserialize_with = "optional_string")]
    pub host_world_wide_node_name: Option<String>,

    #[serde(rename = "target_wwnn")]
    pub target_world_wide_node_name: String,

    #[serde(rename = "host_wwpn")]
    #[serde(deserialize_with = "optional_string")]
    pub host_world_wide_port_name: Option<String>,

    #[serde(rename = "target_wwpn")]
    #[serde(deserialize_with = "optional_string")]
    pub target_world_wide_port_name: Option<String>,

    #[serde(deserialize_with = "optional_string")]
    pub host_adapter: Option<String>,

    #[serde(rename = "lun_hex")]
    #[serde(deserialize_with = "hex_string")]
    pub logical_unit_number: Option<u64>,

    #[serde(rename = "marginal_st")]
    pub marginal_path_state: MarginalState,
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
        "dev" : "sdag",
        "dev_t" : "66:0",
        "dm_st" : "active",
        "dev_st" : "running",
        "chk_st" : "ready",
        "checker" : "tur",
        "pri" : 50,
        "host_wwnn" : "[undef]",
        "target_wwnn" : "iqn.2010-06.com.purestorage:flasharray.1007a309fb52d942",
        "host_wwpn" : "[undef]",
        "target_wwpn" : "[undef]",
        "host_adapter" : "192.168.131.31",
        "lun_hex" : "0x000e000000000000",
        "marginal_st" : "normal"
     }), Path {
        device_name: "sdag".into(),
        device_major_minor: "66:0".into(),
        device_mapper_state: PathDeviceMapperState::Active,
        device_state: DeviceState::Running,
        checker_state: PathCheckerState::Ready,
        checker: PathChecker::TestUnitReady,
        priority: 50,
        host_world_wide_node_name: None,
        target_world_wide_node_name: "iqn.2010-06.com.purestorage:flasharray.1007a309fb52d942"
            .into(),
        host_world_wide_port_name: None,
        target_world_wide_port_name: None,
        host_adapter: Some("192.168.131.31".into()),
        logical_unit_number: Some(0x000e000000000000),
        marginal_path_state: MarginalState::Normal,
    })]
    #[case(json!({
        "dev" : "sdcx",
        "dev_t" : "70:80",
        "dm_st" : "active",
        "dev_st" : "running",
        "chk_st" : "ready",
        "checker" : "tur",
        "pri" : 50,
        "host_wwnn" : "0x20000090fa678144",
        "target_wwnn" : "0x524a937247e74300",
        "host_wwpn" : "0x10000090fa678144",
        "target_wwpn" : "0x524a937247e74300",
        "host_adapter" : "0000:00:03.0",
        "marginal_st" : "normal"
     }), Path {
        device_name: "sdcx".into(),
        device_major_minor: "70:80".into(),
        device_mapper_state: PathDeviceMapperState::Active,
        device_state: DeviceState::Running,
        checker_state: PathCheckerState::Ready,
        checker: PathChecker::TestUnitReady,
        priority: 50,
        host_world_wide_node_name: Some("0x20000090fa678144".into()),
        target_world_wide_node_name: "0x524a937247e74300".into(),
        host_world_wide_port_name: Some("0x10000090fa678144".into()),
        target_world_wide_port_name: Some("0x524a937247e74300".into()),
        host_adapter: Some("0000:00:03.0".into()),
        marginal_path_state: MarginalState::Normal,
        ..Default::default()
    })]
    fn test_parsing_path(#[case] input: serde_json::Value, #[case] expected: Path) {
        let result: Path = serde_json::from_value(input).unwrap();
        assert_eq!(result, expected);
    }
}
