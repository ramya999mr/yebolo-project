FROM rust:1.70

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock* ./

# Create empty src so cargo build works
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release || true

# Copy actual source code
COPY . .

RUN cargo build --release

CMD ["./target/release/backend"]