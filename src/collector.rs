use multipathd_structs::{
    Multipathd,
    common::{DeviceState, PathDeviceMapperState, PathGroupDeviceMapperState},
    path::PathCheckerState,
};
use prometheus::{
    IntGaugeVec, Opts,
    core::{Collector, Desc},
    proto::MetricFamily,
};
use strum::IntoEnumIterator;

pub struct MultipathdCollector {
    path_group_device_mapper_state: IntGaugeVec,
    path_device_mapper_state: IntGaugeVec,
    path_device_state: IntGaugeVec,
    path_checker_state: IntGaugeVec,
}

fn generate_enum_help<E: IntoEnumIterator + ToString>(enum_name: &str) -> String {
    let variants = E::iter()
        .enumerate()
        .map(|(i, variant)| format!("{} = {}", variant.to_string().to_lowercase(), i))
        .collect::<Vec<_>>()
        .join(", ");

    format!("{} ({})", enum_name, variants)
}

impl MultipathdCollector {
    const METRICS_NUMBER: usize = 4;

    const NAMESPACE: &'static str = "multipathd";

    pub fn new() -> Self {
        let path_group_labels = vec!["map", "group"];
        let path_labels = [path_group_labels.clone(), vec!["target_wwnn", "dev"]].concat();

        Self {
            path_group_device_mapper_state: IntGaugeVec::new(
                Opts::new(
                    "device_mapper_state".to_owned(),
                    generate_enum_help::<PathGroupDeviceMapperState>("dm_st"),
                )
                .namespace(Self::NAMESPACE)
                .subsystem("path_group"),
                &path_group_labels,
            )
            .unwrap(),
            path_device_mapper_state: IntGaugeVec::new(
                Opts::new(
                    "device_mapper_state".to_owned(),
                    generate_enum_help::<PathDeviceMapperState>("dm_st"),
                )
                .namespace(Self::NAMESPACE)
                .subsystem("path"),
                &path_labels,
            )
            .unwrap(),
            path_device_state: IntGaugeVec::new(
                Opts::new(
                    "device_state".to_owned(),
                    generate_enum_help::<DeviceState>("dev_st"),
                )
                .namespace(Self::NAMESPACE)
                .subsystem("path"),
                &path_labels,
            )
            .unwrap(),
            path_checker_state: IntGaugeVec::new(
                Opts::new(
                    "checker_state".to_owned(),
                    generate_enum_help::<PathCheckerState>("chk_st"),
                )
                .namespace(Self::NAMESPACE)
                .subsystem("path"),
                &path_labels,
            )
            .unwrap(),
        }
    }
}

impl Collector for MultipathdCollector {
    fn desc(&self) -> Vec<&Desc> {
        let mut desc = Vec::with_capacity(Self::METRICS_NUMBER);
        desc.extend(self.path_group_device_mapper_state.desc());
        desc.extend(self.path_device_mapper_state.desc());
        desc.extend(self.path_device_state.desc());
        desc.extend(self.path_checker_state.desc());

        desc
    }

    fn collect(&self) -> Vec<MetricFamily> {
        let mut mfs = Vec::with_capacity(Self::METRICS_NUMBER);

        let command = std::process::Command::new("multipathd")
            .arg("show")
            .arg("maps")
            .arg("json")
            .output()
            .expect("failed to execute command");

        let multipathd: Multipathd = serde_json::from_slice(&command.stdout).unwrap();

        self.path_group_device_mapper_state.reset();
        self.path_device_mapper_state.reset();
        self.path_device_state.reset();
        self.path_checker_state.reset();

        multipathd.maps.iter().for_each(|map| {
            map.path_groups.iter().for_each(|path_group| {
                path_group.paths.iter().for_each(|path| {
                    let path_group_labels =
                        vec![map.uuid.clone(), path_group.group.clone().to_string()];
                    self.path_group_device_mapper_state
                        .with_label_values(&path_group_labels)
                        .set(path_group.device_mapper_state.clone() as i64);

                    let path_labels = [
                        path_group_labels.clone(),
                        vec![
                            path.target_world_wide_node_name.clone(),
                            path.device_name.clone(),
                        ],
                    ]
                    .concat();
                    self.path_device_mapper_state
                        .with_label_values(&path_labels)
                        .set(path.device_mapper_state.clone() as i64);
                    self.path_device_state
                        .with_label_values(&path_labels)
                        .set(path.device_state.clone() as i64);
                    self.path_checker_state
                        .with_label_values(&path_labels)
                        .set(path.checker_state.clone() as i64);
                });
            });
        });

        mfs.extend(self.path_group_device_mapper_state.collect());
        mfs.extend(self.path_device_mapper_state.collect());
        mfs.extend(self.path_device_state.collect());
        mfs.extend(self.path_checker_state.collect());
        mfs
    }
}
