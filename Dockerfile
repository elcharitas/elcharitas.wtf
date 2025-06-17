FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
RUN apt update -y && apt install nodejs npm
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/aurapay ./app

FROM debian:stable-slim AS runtime
RUN apt update -y \
    && apt install libssl3
WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/app"]