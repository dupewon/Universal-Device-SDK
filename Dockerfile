FROM rust:slim-bookworm AS builder

WORKDIR /build
COPY . .
RUN cargo build --release --bin uds

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libudev-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/uds /usr/local/bin/uds
ENTRYPOINT ["uds"]
