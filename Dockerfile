
FROM rust:1.86.0-slim AS build

WORKDIR /app

RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config cmake && \
    rm -rf /var/lib/apt/lists/*

COPY . /app/

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/sonos-rs /usr/local/bin/sonos-rs

RUN chmod +x /usr/local/bin/sonos-rs

ENTRYPOINT ["/usr/local/bin/sonos-rs"]