FROM rust:latest AS builder

WORKDIR /app

COPY . .

ENV RUST_MIN_STACK=934217728
ENV RUST_BACKTRACE=1
ENV CARGO_BUILD_JOBS=1

RUN cargo build -p token-service --release

FROM debian:bookworm-slim

RUN apt-get update && \
  apt-get install -y curl wget openssl ca-certificates && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/token-service .

EXPOSE 3000

CMD ["./token-service"]
