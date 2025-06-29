FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
RUN apt update -y && apt install nodejs npm -y
RUN npm install -g tailwindcss@3
RUN npm install @tailwindcss/typography@0.5.9

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN npx tailwindcss -i ./global.css -o ./public/styles.css --minify
RUN cargo build --release
RUN mv ./target/release/elcharitas .

FROM debian:stable-slim AS runtime
RUN apt update -y \
    && apt install -y libssl3 ca-certificates

WORKDIR /app
COPY --from=builder /app/elcharitas /usr/local/bin
COPY --from=builder /app/public ./public

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/elcharitas"]