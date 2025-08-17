// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"k8s.io/utils/ptr"
)

func TestParseHexString(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected *uint64
	}{
		// Nil cases
		{
			name:     "empty string returns nil",
			input:    "",
			expected: nil,
		},
		{
			name:     "undef string returns nil",
			input:    "[undef]",
			expected: nil,
		},

		// Hex with 0x prefix
		{
			name:     "hex with 0x prefix",
			input:    "0x1234",
			expected: ptr.To[uint64](0x1234),
		},
		{
			name:     "hex with 0X prefix (uppercase)",
			input:    "0X1234",
			expected: ptr.To[uint64](0x1234),
		},
		{
			name:     "hex with letters",
			input:    "0xdeadbeef",
			expected: ptr.To[uint64](0xdeadbeef),
		},
		{
			name:     "hex with uppercase letters",
			input:    "0xDEADBEEF",
			expected: ptr.To[uint64](0xDEADBEEF),
		},
		{
			name:     "zero hex value",
			input:    "0x0",
			expected: ptr.To[uint64](0),
		},
		{
			name:     "large hex value",
			input:    "0x000e000000000000",
			expected: ptr.To[uint64](0x000e000000000000),
		},
		{
			name:     "max uint64 hex",
			input:    "0xffffffffffffffff",
			expected: ptr.To[uint64](0xffffffffffffffff),
		},

		// Decimal numbers (base 0 interprets as decimal without prefix)
		{
			name:     "decimal number",
			input:    "1234",
			expected: ptr.To[uint64](1234),
		},
		{
			name:     "zero decimal",
			input:    "0",
			expected: ptr.To[uint64](0),
		},

		// Octal numbers (base 0 interprets leading 0 as octal)
		{
			name:     "octal number",
			input:    "0755",
			expected: ptr.To[uint64](0755), // 493 in decimal
		},

		// Invalid cases
		{
			name:     "invalid hex characters",
			input:    "0xZZZ",
			expected: nil,
		},
		{
			name:     "hex without 0x not valid in base 0",
			input:    "deadbeef",
			expected: nil,
		},
		{
			name:     "overflow uint64",
			input:    "0x10000000000000000", // 2^64
			expected: nil,
		},
		{
			name:     "negative number",
			input:    "-1",
			expected: nil,
		},
		{
			name:     "invalid format",
			input:    "0x",
			expected: nil,
		},
		{
			name:     "text string",
			input:    "not a number",
			expected: nil,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := parseHexString(tt.input)
			assert.Equal(t, tt.expected, result)
		})
	}
}
