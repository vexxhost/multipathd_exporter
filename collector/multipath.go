// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package collector

import (
	"encoding/json"
	"log/slog"
	"os/exec"
	"strconv"

	"github.com/prometheus/client_golang/prometheus"

	"github.com/vexxhost/multipathd_exporter/internal/multipath"
)

type MultipathCollector struct {
	logger *slog.Logger

	pathGroupDeviceMapperState *prometheus.Desc
	pathDeviceMapperState      *prometheus.Desc
	pathDeviceState            *prometheus.Desc
	pathCheckerState           *prometheus.Desc
}

func NewMultipathCollector(logger *slog.Logger) prometheus.Collector {
	return &MultipathCollector{
		logger: logger,

		pathGroupDeviceMapperState: prometheus.NewDesc(
			prometheus.BuildFQName("multipath", "path_group", "device_mapper_state"),
			"Group device mapper state of the multipath path",
			[]string{"map", "group"},
			nil,
		),
		pathDeviceMapperState: prometheus.NewDesc(
			prometheus.BuildFQName("multipath", "path", "device_mapper_state"),
			"Device mapper state of the multipath path",
			[]string{"map", "group", "target_wwnn", "dev"},
			nil,
		),
		pathDeviceState: prometheus.NewDesc(
			prometheus.BuildFQName("multipath", "path", "device_state"),
			"Device state of the multipath path",
			[]string{"map", "group", "target_wwnn", "dev"},
			nil,
		),
		pathCheckerState: prometheus.NewDesc(
			prometheus.BuildFQName("multipath", "path", "checker_state"),
			"Checker state of the multipath path",
			[]string{"map", "group", "target_wwnn", "dev"},
			nil,
		),
	}
}

func (c *MultipathCollector) Describe(ch chan<- *prometheus.Desc) {
	ch <- c.pathGroupDeviceMapperState
	ch <- c.pathDeviceMapperState
	ch <- c.pathDeviceState
	ch <- c.pathCheckerState
}

func (c *MultipathCollector) Collect(ch chan<- prometheus.Metric) {
	cmd := exec.Command("multipathd", "show", "maps", "json")
	output, err := cmd.Output()
	if err != nil {
		c.logger.Error("failed to execute multipathd command", "err", err)
		return
	}

	var result multipath.Status
	if err := json.Unmarshal(output, &result); err != nil {
		c.logger.Error("failed to parse multipathd output", "err", err)
		return
	}

	for _, map_ := range result.Maps {
		for _, pathGroup := range map_.PathGroups {
			emitEnumStateMetric(
				ch,
				c.pathGroupDeviceMapperState,
				pathGroup.DeviceMapperState,
				multipath.PathGroupDeviceMapperStateValues(),
				map_.UUID, strconv.FormatUint(pathGroup.Group, 10),
			)

			for _, path := range pathGroup.Paths {
				emitEnumStateMetric(
					ch,
					c.pathDeviceMapperState,
					path.DeviceMapperState,
					multipath.PathDeviceMapperStateValues(),
					map_.UUID, strconv.FormatUint(pathGroup.Group, 10), path.TargetWorldWideNodeName, path.DeviceName,
				)
				emitEnumStateMetric(
					ch,
					c.pathDeviceState,
					path.DeviceState,
					multipath.DeviceStateValues(),
					map_.UUID, strconv.FormatUint(pathGroup.Group, 10), path.TargetWorldWideNodeName, path.DeviceName,
				)
				emitEnumStateMetric(
					ch,
					c.pathCheckerState,
					path.CheckerState,
					multipath.PathCheckerStateValues(),
					map_.UUID, strconv.FormatUint(pathGroup.Group, 10), path.TargetWorldWideNodeName, path.DeviceName,
				)
			}
		}
	}
}
