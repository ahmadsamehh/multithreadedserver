# Simple HTTP Server in Rust

This Rust project implements a basic HTTP server that listens on `127.0.0.1:9000`. It serves different HTML pages based on the URL path requested by the client. The server supports three routes:

- `/` serves `hello.html`
- `/help` serves `help.html`
- Any other path serves `error.html`

## Requirements

- Rust (latest stable version)

## Usage

1. Clone this repository.
2. Make sure to have the following HTML files in the `src` folder:
    - `hello.html`
    - `help.html`
    - `error.html`
3. Run the server with:

```bash
cargo run
