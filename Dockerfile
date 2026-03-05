# Copyright (c) 2024 VEXXHOST, Inc.
# SPDX-License-Identifier: Apache-2.0

FROM golang:1.25.6@sha256:06d1251c59a75761ce4ebc8b299030576233d7437c886a68b43464bad62d4bb1 AS builder
WORKDIR /src
COPY go.mod go.sum /src/
RUN go mod download
COPY . /src
RUN CGO_ENABLED=0 go build -o /multipathd_exporter

FROM debian:13-slim@sha256:1d3c811171a08a5adaa4a163fbafd96b61b87aa871bbc7aa15431ac275d3d430
RUN \
    --mount=type=cache,target=/var/cache/apt \
    rm -fv /etc/apt/apt.conf.d/docker-clean && \
    apt-get update && \
    apt-get --no-install-recommends install -y multipath-tools
COPY --from=builder /multipathd_exporter /bin/multipathd_exporter
EXPOSE 9282
ENTRYPOINT ["/bin/multipathd_exporter"]
