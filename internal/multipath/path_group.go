// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

type PathGroup struct {
	Selector          string                     `json:"selector"`
	Priority          uint64                     `json:"pri"`
	DeviceMapperState PathGroupDeviceMapperState `json:"dm_st"`
	MarginalPathState MarginalState              `json:"marginal_st"`
	Group             uint64                     `json:"group"`
	Paths             []Path                     `json:"paths"`
}
