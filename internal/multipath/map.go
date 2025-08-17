// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L214-L231
type MapFailback string

const (
	MapFailbackImmediate  MapFailback = "immediate"
	MapFailbackFollowOver MapFailback = "follow_over"
	MapFailbackManual     MapFailback = "manual"
	MapFailbackDeferred   MapFailback = "deferred"
	MapFailbackUndefined  MapFailback = "undef"
)

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L181-L190
type MapWriteProtection string

const (
	MapWriteProtectionReadOnly  MapWriteProtection = "ro"
	MapWriteProtectionReadWrite MapWriteProtection = "rw"
	MapWriteProtectionUndefined MapWriteProtection = "undef"
)

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L233-L253
type MapQueueing string

const (
	MapQueueingOff       MapQueueing = "off"
	MapQueueingOn        MapQueueing = "on"
	MapQueueingTimed     MapQueueing = "timed"
	MapQueueingUndefined MapQueueing = "-"
)

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L181-L190
type MapAction string

const (
	MapActionReject          MapAction = "reject"
	MapActionRename          MapAction = "rename"
	MapActionReload          MapAction = "reload"
	MapActionCreate          MapAction = "create"
	MapActionSwitchPathGroup MapAction = "switchpg"
	MapActionNone            MapAction = ""
)

type Map struct {
	Name              string                `json:"name"`
	UUID              string                `json:"uuid"`
	Sysfs             *string               `json:"sysfs,omitempty"`
	Failback          MapFailback           `json:"failback"`
	Queueing          MapQueueing           `json:"queueing"`
	Paths             uint64                `json:"paths"`
	WriteProtection   MapWriteProtection    `json:"write_prot"`
	DeviceMapperState PathDeviceMapperState `json:"dm_st"`
	Features          string                `json:"features"`
	HardwareHandler   string                `json:"hwhandler"`
	Action            MapAction             `json:"action"`
	PathFaults        uint64                `json:"path_faults"`
	Vendor            string                `json:"vend"`
	Product           string                `json:"prod"`
	Rev               string                `json:"rev"`
	SwitchGroup       uint64                `json:"switch_grp"`
	Loads             uint64                `json:"map_loads"`
	TotalQueueTime    uint64                `json:"total_q_time"`
	QueueTimeouts     uint64                `json:"q_timeouts"`
	PathGroups        []PathGroup           `json:"path_groups"`
}
