# Use the official Rust image as the base image
FROM rust:latest

# Set the architecture dynamically (TARGETARCH will be provided via --build-arg or default to x86_64)
ARG TARGETARCH

# Set the working directory inside the container
WORKDIR /usr/src/margot

# Copy the Cargo.toml and Cargo.lock to leverage Docker cache for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src directory and build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Now copy the actual source files
COPY . .

# Conditional: Install the appropriate target based on architecture
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      rustup target add aarch64-unknown-linux-gnu; \
    else \
      rustup target add x86_64-unknown-linux-gnu; \
    fi

# Build the project for the correct target based on architecture
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      cargo build --release --target aarch64-unknown-linux-gnu; \
    else \
      cargo build --release --target x86_64-unknown-linux-gnu; \
    fi

# Set the command to run the appropriate binary
CMD if [ "$TARGETARCH" = "arm64" ]; then \
      target/aarch64-unknown-linux-gnu/release/margot; \
    else \
      target/x86_64-unknown-linux-gnu/release/margot; \
    fi


