# syntax=docker/dockerfile:1
FROM rust:1.66 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
RUN adduser --disabled-password --gecos '' birdrust
USER birdrust
COPY --from=builder /usr/local/cargo/bin/birdrust /usr/local/bin/birdrust
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
CMD ["birdrust"]