FROM rust:1.93.1  AS build
# Donwloading external dependency
RUN apt update -y && apt-get install libudev-dev -y

# Dependency cache build
RUN cargo new --bin Nebula
WORKDIR /Nebula

ADD Cargo.toml .
RUN cargo build --release

RUN rm -rf src; rm target/release/deps/Nebula*

# Setting up project
COPY src/ ./src/
RUN cargo build --release

# Smaller image
FROM debian:stable-slim AS final
COPY --from=build /Nebula/target/release/Nebula .

## External dependencyes (again)
RUN apt update -y && apt-get install openssl libc6 -y
COPY assets/ ./assets/
EXPOSE 3621
CMD ./Nebula -a 0.0.0.0:3621

