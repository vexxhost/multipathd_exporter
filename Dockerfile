# Copyright (c) 2024 VEXXHOST, Inc.
# SPDX-License-Identifier: Apache-2.0

FROM golang:1.25.0 AS builder
WORKDIR /src
COPY go.mod go.sum /src/
RUN go mod download
COPY . /src
RUN CGO_ENABLED=0 go build -o /multipathd_exporter

FROM debian:13-slim
RUN \
    --mount=type=cache,target=/var/cache/apt \
    rm -fv /etc/apt/apt.conf.d/docker-clean && \
    apt-get update && \
    apt-get --no-install-recommends install -y multipath-tools
COPY --from=builder /multipathd_exporter /bin/multipathd_exporter
EXPOSE 9282
ENTRYPOINT ["/bin/multipathd_exporter"]
