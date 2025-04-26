FROM rust:1.86.0@sha256:7b65306dd21304f48c22be08d6a3e41001eef738b3bd3a5da51119c802321883 AS builder
ADD . /app
WORKDIR /app
RUN cargo build --release

FROM rust:1.86.0-slim@sha256:3f391b0678a6e0c88fd26f13e399c9c515ac47354e3cadfee7daee3b21651a4f AS runtime
RUN \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y multipath-tools
COPY --from=builder /app/target/release/multipathd-exporter /usr/local/bin/multipathd_exporter
EXPOSE 10035
ENTRYPOINT ["/usr/local/bin/multipathd_exporter"]
