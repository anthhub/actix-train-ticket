FROM rust:1.46.0-alpine AS builder
ADD . /usr/src/myapp
WORKDIR /usr/src/myapp
RUN cargo build --release

FROM alpine:3.13
EXPOSE 8000
COPY --from=builder /usr/src/myapp/target/release/actix-train-ticket /main
COPY --from=builder /usr/src/myapp/static /static
CMD ["/main"]



