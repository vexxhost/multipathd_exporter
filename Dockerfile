FROM rust:1.86.0@sha256:13e89107e40a1b008fd5f4f23e0737eb655d09f179036d029ee77f0bd465b51a AS builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM rust:1.86.0-slim@sha256:f3b6373bda11771f249d0401eedf5bb2b205ba410773e7559c34a3aa3f623671 AS runtime
RUN \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y multipath-tools
COPY --from=builder /app/target/release/multipathd-exporter /usr/local/bin/multipathd_exporter
EXPOSE 10035
ENTRYPOINT ["/usr/local/bin/multipathd_exporter"]
