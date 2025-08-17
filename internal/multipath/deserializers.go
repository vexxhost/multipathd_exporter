// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

import "strconv"

// parseHexString parses a hex string to *uint64
func parseHexString(s string) *uint64 {
	if s == "" || s == "[undef]" {
		return nil
	}

	val, err := strconv.ParseUint(s, 0, 64)
	if err != nil {
		return nil
	}

	return &val
}
