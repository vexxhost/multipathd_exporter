use serde::Deserialize;
use strum_macros::{Display, EnumIter};

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L623-L636
#[derive(Clone, Debug, Default, Display, PartialEq, Deserialize, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum PathGroupDeviceMapperState {
    Active,
    Enabled,
    Disabled,

    #[default]
    #[serde(rename = "undef")]
    Undefined,
}

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L549-L563
#[derive(Clone, Debug, Default, Display, PartialEq, Deserialize, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum PathDeviceMapperState {
    Active,
    Failed,

    #[default]
    #[serde(rename = "undef")]
    Undefined,
}

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L512-L521
#[derive(Clone, Debug, Default, Display, PartialEq, Deserialize, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum DeviceState {
    Running,
    Offline,

    #[default]
    Unknown,
}

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//               https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L523-L547
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginalState {
    Marginal,

    #[default]
    Normal,
}
