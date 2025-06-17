FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
RUN apt update -y && apt install nodejs npm -y
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/elcharitas .

FROM debian:stable-slim AS runtime
RUN apt update -y \
    && apt install libssl3
WORKDIR /app
COPY --from=builder /app/elcharitas /usr/local/bin
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/elcharitas"]