# Start with the Rust image, which includes tools for compiling and running Rust applications
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/margot

# Install necessary dependencies (including bash)
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu bash

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies so that they are cached
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy the actual source code
COPY . .

# Compile the project for ARM architecture
RUN rustup target add aarch64-unknown-linux-gnu && cargo build --target aarch64-unknown-linux-gnu --release

# Expose bash as the entrypoint, but you can still run margot commands by overriding it
ENTRYPOINT ["/bin/bash"]

#ENTRYPOINT ["/usr/src/margot/target/aarch64-unknown-linux-gnu/release/margot"]
