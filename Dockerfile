FROM rust:1.75 AS builder
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY . ./
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/norris ./
CMD ["./norris"]