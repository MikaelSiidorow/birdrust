# Birdrust - birdnest server

A Rust websocket server build for Reaktor's [Summer Assignment](https://assignments.reaktor.com/birdnest)!

## Development

1. `cargo update`
2. `cargo run` (or `cargo watch -x run` for auto-reloading, after `cargo install cargo-watch`)

## Build & run

1. `cargo update`
2. `cargo build --release`
3. `./target/release/birdnest_server`
4. Connect to `ws://localhost:3000/reports` from client

## Documentation

1. `cargo doc --open`

## Tests

1. `cargo test`
