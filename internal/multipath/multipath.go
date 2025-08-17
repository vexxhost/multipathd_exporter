// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

type Status struct {
	MajorVersion uint64 `json:"major_version"`
	MinorVersion uint64 `json:"minor_version"`
	Maps         []Map  `json:"maps"`
}
