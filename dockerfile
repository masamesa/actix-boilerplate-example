# Rust as the base image
FROM rust:slim

# New cargo project
RUN USER=root cargo new --bin app

# Our work directory
WORKDIR /app

# Copy the cargo manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# After the dependencies are built, copy the source code
COPY ./src ./src

# Environment variables. 
# You should really have this in a .env file, but this is a quick example.

# The ip is set to 0.0.0.0 for docker
ENV SERVER_IP=0.0.0.0 
# Any port will work, but this is the default. If you change it be sure to change it in the docker run command as well.
ENV SERVER_PORT=8080

# Build for release.
RUN rm ./target/release/deps/app*
RUN cargo install --path .

# Run the application
CMD ["app"]