# ── Builder stage ────────────────────────────────────────────────────────
FROM rust:1.93-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang \
    lld \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install tailwindcss standalone
RUN curl -sL https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    -o /usr/local/bin/tailwindcss && chmod +x /usr/local/bin/tailwindcss

WORKDIR /app

# Copy dependency files first for layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy full source and build
COPY . .
RUN cargo build --release --locked

# ── Runtime stage ────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1000 brewlog && useradd -u 1000 -g brewlog -m brewlog

COPY --from=builder /app/target/release/brewlog /usr/local/bin/brewlog

USER 1000:1000

ENTRYPOINT ["brewlog", "serve", "--database-url", "sqlite:///data/brewlog.db"]
