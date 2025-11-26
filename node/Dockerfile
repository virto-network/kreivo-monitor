# Builder stage
FROM rust:latest as builder

WORKDIR /usr/src/kreivo-monitor
COPY kreivo-monitor/ .
RUN cargo install --path .

# Runtime stage
FROM debian:stable-slim

# Install systemd for systemctl binary
RUN apt-get update && \
    apt-get install -y systemd && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/kreivo-monitor /usr/local/bin/kreivo-monitor

CMD ["kreivo-monitor"]
