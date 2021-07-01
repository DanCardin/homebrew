FROM rust:1-slim-buster AS base

ENV USER=root
ENV SQLX_OFFLINE=true

WORKDIR /usr/src/homebrew-web

RUN cargo install watchexec-cli systemfd

RUN cargo init
COPY homebrew-web/Cargo.toml /usr/src/homebrew-web/Cargo.toml
COPY homebrew /usr/src/homebrew
RUN cargo fetch

COPY homebrew-web/sqlx-data.json /usr/src/homebrew-web/sqlx-data.json
COPY homebrew-web/src /usr/src/homebrew-web/src

RUN cargo build

ENTRYPOINT [ "cargo", "run" ]
