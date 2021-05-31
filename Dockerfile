# # # FROM rust:1.46.0-alpine AS builder
# # FROM rust:1.46.0-alpine 

# # ADD . /usr/src/myapp
# # WORKDIR /usr/src/myapp
# # RUN cargo build --release

# # # FROM alpine:3.13
# # EXPOSE 8000
# # # COPY --from=builder /usr/src/myapp/target/release/actix-train-ticket /main
# # # COPY --from=builder /usr/src/myapp/static /static

# # CMD ["/usr/src/myapp/target/release/actix-train-ticket"]


# FROM rust:1.50 as builder
# ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static \
#     RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup \
#     RUSTFLAGS=-Ctarget-feature=-crt-static
# ADD . /usr/src/myapp
# WORKDIR /usr/src/myapp
# RUN cargo build --release
# RUN ls

# # FROM debian:buster-slim
# FROM alpine:3.13
# EXPOSE 8000
# COPY --from=builder /usr/src/myapp/target/release/actix-train-ticket /actix-train-ticket
# RUN ls
# # COPY --from=builder /usr/src/myapp/static/cities.json /static/cities.json
# CMD ["/actix-train-ticket"]


FROM rust AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential git clang llvm-dev libclang-dev libssl-dev pkg-config libpq-dev musl-tools brotli

RUN USER=root cargo new actix-train-ticket
WORKDIR /usr/src/actix-train-ticket
COPY Cargo.toml Cargo.lock ./
COPY static ./static
# COPY vendor ./vendor
COPY src ./src
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM debian
COPY --from=build /usr/local/cargo/bin/actix-train-ticket .
# COPY .env ./.env
COPY static ./static
# COPY vendor ./vendor
USER 1000
EXPOSE 8000
CMD ["./actix-train-ticket"]