FROM rust:latest as builder

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/apd"]
