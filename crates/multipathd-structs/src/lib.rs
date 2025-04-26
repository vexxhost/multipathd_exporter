pub mod common;
mod deserializers;
pub mod map;
pub mod path;
pub mod path_group;

use map::Map;
use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Multipathd {
    pub major_version: u64,
    pub minor_version: u64,
    pub maps: Vec<Map>,
}
