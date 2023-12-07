FROM rust:1-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

COPY ./ ./
RUN cargo build

CMD trunk serve