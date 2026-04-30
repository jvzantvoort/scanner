# Build stage
FROM rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY benches ./benches

# Build for release
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Create non-root user
RUN addgroup -g 1000 scanner && \
    adduser -D -u 1000 -G scanner scanner

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/scanner /usr/local/bin/scanner

# Switch to non-root user
USER scanner

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/scanner"]
CMD ["--help"]
