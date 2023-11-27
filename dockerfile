FROM rust:latest as builder
COPY . .
RUN cargo build --release

FROM debian:latest
COPY --from=builder /target/release/apirust /app/apirust
WORKDIR /app
CMD ["./apirust"]

