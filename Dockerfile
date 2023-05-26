# Use the official Rust image as the base image
FROM rust:latest

# Install the necessary tools
RUN rustup update
RUN rustup default nightly

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src/ ./src/

# Build the dependencies
RUN cargo build --release

# Expose the port that the app listens on
EXPOSE 8000

# Start the app
CMD ["cargo", "run"] 
