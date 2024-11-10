
Simple HTTP Server in Rust

This Rust project implements a basic HTTP server that listens on 127.0.0.1:9000. It serves different HTML pages based on the URL path requested by the client. The server supports three routes:

/ serves hello.html
/help serves help.html
Any other path serves error.html
Requirements

Rust (latest stable version)
Usage

Clone this repository.
Make sure to have the following HTML files in the src folder:
hello.html
help.html
error.html
Run the server with:
cargo run
The server will start and listen on http://127.0.0.1:9000.

