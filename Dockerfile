FROM rust:1.86.0 AS builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM rust:1.86.0-slim AS runtime
RUN \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && \
    apt-get install -y multipath-tools
COPY --from=builder /app/target/release/multipathd_exporter /usr/local/bin/multipathd_exporter
EXPOSE 10035
ENTRYPOINT ["/usr/local/bin/multipathd_exporter"]
