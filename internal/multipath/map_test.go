// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"k8s.io/utils/ptr"
)

func TestParsingMap(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected Map
	}{
		{
			name: "iSCSI",
			input: `{
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
				"path_groups" : []
			}`,
			expected: Map{
				Name:              "3624a9370e92ace0b12e941bf000cb4d5",
				UUID:              "3624a9370e92ace0b12e941bf000cb4d5",
				Sysfs:             ptr.To("dm-31"),
				Failback:          MapFailbackImmediate,
				Queueing:          MapQueueingOff,
				Paths:             8,
				WriteProtection:   MapWriteProtectionReadWrite,
				DeviceMapperState: PathDeviceMapperStateActive,
				Features:          "0",
				HardwareHandler:   "1 alua",
				Action:            MapActionNone,
				PathFaults:        23,
				Vendor:            "PURE",
				Product:           "FlashArray",
				Rev:               "8888",
				SwitchGroup:       0,
				Loads:             16,
				TotalQueueTime:    0,
				QueueTimeouts:     0,
				PathGroups:        []PathGroup{},
			},
		},
		{
			name: "Fiber Channel",
			input: `{
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
			}`,
			expected: Map{
				Name:              "3624a9370ca38a5c63c724ca8000a5c6d",
				UUID:              "3624a9370ca38a5c63c724ca8000a5c6d",
				Sysfs:             ptr.To("dm-0"),
				Failback:          MapFailbackImmediate,
				Queueing:          MapQueueingOff,
				Paths:             8,
				WriteProtection:   MapWriteProtectionReadWrite,
				DeviceMapperState: PathDeviceMapperStateActive,
				Features:          "0",
				HardwareHandler:   "1 alua",
				Action:            MapActionNone,
				PathFaults:        0,
				Vendor:            "PURE",
				Product:           "FlashArray",
				Rev:               "8888",
				SwitchGroup:       0,
				Loads:             0,
				TotalQueueTime:    0,
				QueueTimeouts:     0,
				PathGroups:        []PathGroup{},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var result Map
			err := json.Unmarshal([]byte(tt.input), &result)

			require.NoError(t, err)
			assert.Equal(t, tt.expected, result)
		})
	}
}
