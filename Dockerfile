FROM rust:1.88

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml first (required)
COPY Cargo.toml ./

# Copy Cargo.lock if it exists (optional - uses glob pattern)
COPY Cargo.loc[k] ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached unless Cargo files change)
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Expose port (adjust as needed for your application)
EXPOSE 8080

# Run the application
CMD ["./target/release/rust_web_crawler"]
