// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestParsingPathGroup(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected PathGroup
	}{
		{
			name: "Basic",
			input: `{
        "selector" : "service-time 0",
        "pri" : 50,
        "dm_st" : "active",
        "marginal_st" : "normal",
        "group" : 1
     }`,
			expected: PathGroup{
				Selector:          "service-time 0",
				Priority:          50,
				DeviceMapperState: PathGroupDeviceMapperStateActive,
				MarginalPathState: MarginalStateNormal,
				Group:             1,
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var result PathGroup
			err := json.Unmarshal([]byte(tt.input), &result)

			require.NoError(t, err)
			assert.Equal(t, tt.expected, result)
		})
	}
}
