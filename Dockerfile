# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.95.0
ARG DEBIAN_VERSION=trixie

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION} AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:${DEBIAN_VERSION}-slim AS runtime

WORKDIR /app

RUN useradd --create-home --shell /usr/sbin/nologin appuser

COPY --from=builder /app/target/release/identity /usr/local/bin/identity

USER appuser

ENTRYPOINT ["identity"]
