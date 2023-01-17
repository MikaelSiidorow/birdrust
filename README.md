# Birdrust

Live app deployed at [https://birdrust.siidorow.com/](https://birdrust.siidorow.com/)!

### Server

A Rust websocket server build for Reaktor's [Summer Assignment](https://assignments.reaktor.com/birdnest)!

Server deployed at [https://birdrust.fly.dev/](https://birdrust.fly.dev/)!

View client documentation [here](/client/README.md)!

## Development

1. `cargo update`
2. `cargo run` (or `cargo watch -x run` for auto-reloading, after `cargo install cargo-watch`)

## Build & run

1. `docker build -t birdrust .`
2. `docker run -p 8080:8080 birdrust`
3. Connect to `ws://localhost:8080/reports` from client

## Documentation

1. `cargo doc --open`

## Tests

1. `cargo test`
