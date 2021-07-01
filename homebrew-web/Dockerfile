FROM rust:1-slim-buster AS base

ENV USER=root
ENV SQLX_OFFLINE=true

WORKDIR /usr/src/homebrew-web

RUN cargo init
COPY homebrew-web/Cargo.toml /usr/src/homebrew-web/Cargo.toml
COPY homebrew /usr/src/homebrew
RUN cargo fetch

COPY homebrew-web/sqlx-data.json /usr/src/homebrew-web/sqlx-data.json
COPY homebrew-web/src /usr/src/homebrew-web/src

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo install watchexec-cli systemfd
RUN cargo build --release --offline

FROM rust:1-slim-buster

COPY --from=builder /usr/local/cargo/bin/watchexec /usr/bin/watchexec
COPY --from=builder /usr/local/cargo/bin/systemfd /usr/bin/systemfd
COPY --from=builder /usr/src/homebrew-web/target/release/homebrew-web /usr/bin/homebrew-web

EXPOSE 5000

ENTRYPOINT [ "/usr/bin/homebrew-web" ]
