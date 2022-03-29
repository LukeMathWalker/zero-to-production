FROM docker.io/library/rust:1.59-slim-bullseye as builder

RUN USER=root cargo new --bin zero-to-production

WORKDIR /zero-to-production

RUN touch ./src/lib.rs

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --release

RUN rm ./src/*.rs ./target/release/deps/zero2prod* ./target/release/deps/libzero2prod* ./target/release/libzero2prod* ./target/release/zero2prod*

COPY ./src ./src

ENV SQLX_OFFLINE true

RUN cargo build --release --bin zero2prod

FROM docker.io/debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
