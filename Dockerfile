
FROM rust AS builder
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential git clang llvm-dev libclang-dev libssl-dev pkg-config libpq-dev musl-tools brotli

RUN USER=root cargo new actix-train-ticket
WORKDIR /usr/src/actix-train-ticket
COPY Cargo.toml Cargo.lock ./
COPY static ./static
COPY src ./src
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo install --target x86_64-unknown-linux-musl --path .

# FROM debian
FROM alpine:3.13
COPY --from=builder /usr/local/cargo/bin/actix-train-ticket .
COPY static ./static
USER 1000
EXPOSE 8000
CMD ["./actix-train-ticket"]