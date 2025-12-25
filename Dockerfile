FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev

RUN cargo build -p server --release

FROM debian:bookworm-slim

RUN mkdir /usr/local/bin/config

COPY --from=builder /usr/src/app/config/server.local.toml /usr/local/bin/config/server.local.toml
COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/server

WORKDIR /usr/local/bin/

CMD [ "server" ]