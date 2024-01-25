# Use an official Rust runtime as a parent image
FROM rust:latest AS build-env

# Set the working directory inside the container
WORKDIR /usr/src/chapp

# Copy the Cargo.toml and Cargo.lock files to leverage Docker cache
COPY Cargo.toml Cargo.lock config.toml  ./

# Copy the source code into the container
COPY src ./src

# Build the Rust application with dependencies (this helps to cache dependencies)
RUN cargo build --release --features postgres

FROM gcr.io/distroless/cc-debian12

# Expose any necessary ports (if your Rust app listens on a port)
EXPOSE 8000

WORKDIR /app
COPY init_db /app/init_db
COPY config.toml /app/

COPY --from=build-env /usr/src/chapp/target/release/clearing-house-app /app/
CMD ["/app/clearing-house-app"]
