# Production-ready Dockerfile for wvm-aws-sdk-s3
FROM rust:1.83-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install shuttle CLI via cargo (pin to compatible version)
RUN cargo install cargo-shuttle --version 0.53.0

# Set working directory
WORKDIR /app

# Copy all source code
COPY . .

# Build the application
RUN cargo build --release --bin server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install shuttle CLI for runtime
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/* \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . "$HOME/.cargo/env" \
    && cargo install cargo-shuttle --version 0.53.0
ENV PATH="/root/.cargo/bin:${PATH}"

# Create app user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set working directory
WORKDIR /app

# Copy the entire workspace structure for shuttle
COPY --from=builder /app /app

# Change ownership to app user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8000

# Environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Start with cargo shuttle run
CMD ["cargo", "shuttle", "run"]