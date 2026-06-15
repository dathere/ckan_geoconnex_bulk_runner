FROM rust:1.96 AS builder
WORKDIR /app
RUN rustup set profile minimal
COPY . .
RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /app/target/release/bulk_loader /
ENTRYPOINT ["/bulk_loader"]
