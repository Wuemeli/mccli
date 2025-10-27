FROM rust:alpine as builder

WORKDIR /app

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/mcvcli /usr/local/bin/mcvcli

ENTRYPOINT ["mcvcli"]
CMD ["--help"]
