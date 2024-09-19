FROM rust:1.81-bookworm AS builder
WORKDIR /usr/src/app
COPY ./src ./src
COPY Cargo.toml Cargo.lock .
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM debian:bookworm-slim

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/vss-mapping .
COPY signals signals
COPY vss-core vss-core
CMD ["./vss-mapping"]
