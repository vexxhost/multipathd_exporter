//go:generate go-enum -f=$GOFILE --marshal --names --values

// Copyright (c) 2024 VEXXHOST, Inc.
// SPDX-License-Identifier: Apache-2.0

package multipath

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L623-L636
//
// ENUM(enabled, disabled, active, undef)
type PathGroupDeviceMapperState string

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L549-L563
//
// ENUM(active, failed, undef)
type PathDeviceMapperState string

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L512-L521
//
// ENUM(running, offline, unknown)
type DeviceState string

// NOTE(mnaser): This was retrieved from the following, in case we missed any:
//
//	https://github.com/opensvc/multipath-tools/blob/master/libmultipath/print.c#L638-L644
//
// ENUM(marginal, normal)
type MarginalState string
