# Birdrust - birdnest server

A Rust websocket server build for Reaktor's [Summer Assignment](https://assignments.reaktor.com/birdnest)!

Live service deployed to [https://birdrust.fly.dev/](https://birdrust.fly.dev/)!

## Development

1. `cargo update`
2. `cargo run` (or `cargo watch -x run` for auto-reloading, after `cargo install cargo-watch`)

## Build & run

1. `docker build -t birdrust .`
2. `cargo run birdrust`
3. Connect to `ws://localhost:8080/reports` from client

## Documentation

1. `cargo doc --open`

## Tests

1. `cargo test`
