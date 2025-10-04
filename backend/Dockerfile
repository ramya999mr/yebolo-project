FROM rust:1.76

WORKDIR /app

# Install build dependencies (gcc, g++, make, etc.)
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev

# Copy manifest first for caching
COPY Cargo.toml Cargo.lock ./

# Build dummy to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy actual source
COPY . .

# Build release binary
RUN cargo build --release
CMD ["./target/release/backend"]