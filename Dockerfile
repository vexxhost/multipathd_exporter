# Copyright (c) 2024 VEXXHOST, Inc.
# SPDX-License-Identifier: Apache-2.0

FROM golang:1.25.0 AS builder
WORKDIR /src
COPY go.mod go.sum /src/
RUN go mod download
COPY . /src
RUN CGO_ENABLED=0 go build -o /multipath_exporter

FROM scratch
COPY --from=builder /multipath_exporter /bin/multipath_exporter
EXPOSE 9282
ENTRYPOINT ["/bin/multipath_exporter"]
