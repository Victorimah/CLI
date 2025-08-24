FROM rust:latest AS builder

WORKDIR /usr/src/cli
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/cli/target/release/cli /app/cli

CMD ["./cli"]
