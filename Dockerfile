FROM rust AS base
WORKDIR /app

FROM base AS build
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM debian:stable-slim AS prod
COPY --from=build /app/target/release/rinha02 /usr/bin/rinha02
EXPOSE 3000
CMD ["rinha02"]