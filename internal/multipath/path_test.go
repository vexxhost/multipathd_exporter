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

func TestParsingPath(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected Path
	}{
		{
			name: "iSCSI",
			input: `{
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
     }`,
			expected: Path{
				DeviceName:              "sdag",
				DeviceMajorMinor:        "66:0",
				DeviceMapperState:       PathDeviceMapperStateActive,
				DeviceState:             DeviceStateRunning,
				CheckerState:            PathCheckerStateReady,
				Checker:                 PathCheckerTestUnitReady,
				Priority:                50,
				HostWorldWideNodeName:   nil,
				TargetWorldWideNodeName: "iqn.2010-06.com.purestorage:flasharray.1007a309fb52d942",
				HostWorldWidePortName:   nil,
				TargetWorldWidePortName: nil,
				HostAdapter:             ptr.To("192.168.131.31"),
				LogicalUnitNumber:       ptr.To[uint64](0x000e000000000000),
				MarginalPathState:       MarginalStateNormal,
			},
		},
		{
			name: "Fiber Channel",
			input: `{
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
     }`,
			expected: Path{
				DeviceName:              "sdcx",
				DeviceMajorMinor:        "70:80",
				DeviceMapperState:       PathDeviceMapperStateActive,
				DeviceState:             DeviceStateRunning,
				CheckerState:            PathCheckerStateReady,
				Checker:                 PathCheckerTestUnitReady,
				Priority:                50,
				HostWorldWideNodeName:   ptr.To("0x20000090fa678144"),
				TargetWorldWideNodeName: "0x524a937247e74300",
				HostWorldWidePortName:   ptr.To("0x10000090fa678144"),
				TargetWorldWidePortName: ptr.To("0x524a937247e74300"),
				HostAdapter:             ptr.To("0000:00:03.0"),
				LogicalUnitNumber:       nil, // Note: not in the JSON, so should be nil
				MarginalPathState:       MarginalStateNormal,
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var result Path
			err := json.Unmarshal([]byte(tt.input), &result)

			require.NoError(t, err)
			assert.Equal(t, tt.expected, result)
		})
	}
}
