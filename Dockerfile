# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
RUN cargo build --release

# Copy the source code to the container
COPY src/ ./src/

# Build the app
RUN cargo build --release

# Expose the port that the app listens on
EXPOSE 8000

# Start the app
CMD ["./target/release/rust_that_light"]
