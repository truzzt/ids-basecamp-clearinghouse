# Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/chapp

# Copy the Cargo.toml and Cargo.lock files to leverage Docker cache
COPY Cargo.toml Cargo.lock config.toml  ./

# Copy the source code into the container
COPY src ./src
COPY init_db ./init_db
COPY keys ./keys
COPY certs ./certs

# Build the Rust application with dependencies (this helps to cache dependencies)
RUN cargo build --release

# Expose any necessary ports (if your Rust app listens on a port)
# EXPOSE 8000

# Run the Rust application when the container starts
CMD ["cargo", "run", "--release"]