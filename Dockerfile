FROM rust:1.87 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/constellation /usr/local/bin/constellation
EXPOSE 8080
CMD ["constellation"]
