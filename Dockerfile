FROM rust:1.69.0-bullseye as chef
RUN cargo install cargo-chef --locked

WORKDIR /app

RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    libssl-dev \
    pkg-config \
  && \
  rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY Cargo.* ./
COPY src src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.* ./
COPY src src

FROM builder AS builder-hub-permissions-cli
RUN cargo build --release

FROM debian:bullseye-slim as base
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

RUN mkdir -p bin

FROM base AS hub-permissions-cli
COPY --from=builder-hub-permissions-cli /app/target/release/hub-permissions-cli /bin/
ENTRYPOINT ["/bin/hub-permissions-cli"]
