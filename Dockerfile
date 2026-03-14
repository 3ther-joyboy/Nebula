FROM rust:1.93.1
WORKDIR /app

COPY assets/ .
COPY Cargo.toml .
COPY src/ .

RUN cargo build --release

COPY target/release/* .

EXPOSE 3621
CMD ["./Nebula -a 0.0.0.0:3621"]

