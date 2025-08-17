//go:generate go-enum -f=$GOFILE --marshal --names --values

// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

import (
	"encoding/json"
)

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/checkers.h#L84-L91
type PathChecker string

const (
	PathCheckerDirectIO                  PathChecker = "directio"
	PathCheckerTestUnitReady             PathChecker = "tur"
	PathCheckerHpServiceGuard            PathChecker = "hp_sw"
	PathCheckerRDAC                      PathChecker = "rdac"
	PathCheckerEmcClariion               PathChecker = "emc_clariion"
	PathCheckerReadSector0               PathChecker = "readsector0"
	PathCheckerHpSmartArrayTestUnitReady PathChecker = "cciss_tur"
	PathCheckerNone                      PathChecker = "none"
	PathCheckerInvalid                   PathChecker = "invalid"
)

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L523-L547
//
// ENUM(ready, faulty, shaky, ghost, delayed, i/o pending, i/o timeout, undef)
type PathCheckerState string

// Path represents a multipath device path
type Path struct {
	DeviceName              string                `json:"dev"`
	DeviceMajorMinor        string                `json:"dev_t"`
	DeviceMapperState       PathDeviceMapperState `json:"dm_st"`
	DeviceState             DeviceState           `json:"dev_st"`
	Checker                 PathChecker           `json:"checker"`
	CheckerState            PathCheckerState      `json:"chk_st"`
	Priority                uint64                `json:"pri"`
	HostWorldWideNodeName   *string               `json:"host_wwnn"`
	TargetWorldWideNodeName string                `json:"target_wwnn"`
	HostWorldWidePortName   *string               `json:"host_wwpn"`
	TargetWorldWidePortName *string               `json:"target_wwpn"`
	HostAdapter             *string               `json:"host_adapter"`
	LogicalUnitNumber       *uint64               `json:"lun_hex"`
	MarginalPathState       MarginalState         `json:"marginal_st"`
}

// UnmarshalJSON implements custom JSON unmarshaling for Path
func (p *Path) UnmarshalJSON(data []byte) error {
	type Alias Path
	aux := &struct {
		HostWorldWideNodeName   interface{} `json:"host_wwnn"`
		HostWorldWidePortName   interface{} `json:"host_wwpn"`
		TargetWorldWidePortName interface{} `json:"target_wwpn"`
		HostAdapter             interface{} `json:"host_adapter"`
		LogicalUnitNumber       interface{} `json:"lun_hex"`
		*Alias
	}{
		Alias: (*Alias)(p),
	}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	p.HostWorldWideNodeName = parseOptionalString(aux.HostWorldWideNodeName)
	p.HostWorldWidePortName = parseOptionalString(aux.HostWorldWidePortName)
	p.TargetWorldWidePortName = parseOptionalString(aux.TargetWorldWidePortName)
	p.HostAdapter = parseOptionalString(aux.HostAdapter)

	if aux.LogicalUnitNumber != nil {
		if hexStr, ok := aux.LogicalUnitNumber.(string); ok {
			p.LogicalUnitNumber = parseHexString(hexStr)
		}
	}

	return nil
}

// parseOptionalString converts interface{} to *string, treating "[undef]" as nil
func parseOptionalString(v interface{}) *string {
	if v == nil {
		return nil
	}

	str, ok := v.(string)
	if !ok || str == "[undef]" || str == "" {
		return nil
	}

	return &str
}
