// Importing necessary modules from the Rust standard library.
use std::{
    fs,                            // File system operations, for reading HTML files.
    io::{prelude::*, BufReader}, // Input/output utilities and buffered reader for efficient reading.
    net::{TcpListener, TcpStream}, // Networking types for handling TCP connections.
};

fn main() {
    // Binding the TCP listener to the local IP address and port 9000.
    // This will listen for incoming connections on this address.
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();

    // Iterating over incoming connections.
    for stream in listener.incoming() {
        // Confirm connection in the terminal.
        println!("Connection Success");

        // Unwrap the stream and pass it to the handle_connection function to process the request.
        handle_connection(stream.unwrap());
    }
}

// Function to handle each client connection.
fn handle_connection(mut stream: TcpStream) {
    // Wrap the TCP stream in a buffered reader to handle reading the request lines efficiently.
    let reader = BufReader::new(&stream);

    // Collect the request lines into a vector, stopping when an empty line is encountered.
    // This empty line indicates the end of the HTTP header section.
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap()) // Unwrap each line, assuming it's valid UTF-8.
        .take_while(|line| !line.is_empty()) // Stop reading at the empty line.
        .collect();

    // Print the full request in the terminal for debugging purposes.
    println!("Request: {http_request:#?}");

    // Get the first line of the HTTP request, which contains the HTTP method and requested path.
    let first_line = &http_request[0];
    println!("first line : {}", first_line);

    // Determine the response based on the requested path.
    let (status_line, content) = match first_line.as_str() {
        // Respond with the main page if the root URL is requested.
        "GET / HTTP/1.1" => (
            "HTTP/1.1 200 OK",                             // HTTP success status.
            fs::read_to_string("src/hello.html").unwrap(), // Read the hello.html file to send as the response body.
        ),

        // Respond with the help page if "/help" is requested.
        "GET /help HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            fs::read_to_string("src/help.html").unwrap(),
        ),

        // Respond with a 404 page for any other request.
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("src/error.html").unwrap(),
        ),
    };

    // Calculate the content length to include it in the HTTP response headers.
    let content_length = content.len();

    // Format the HTTP response with headers and body.
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    // Send the response to the client over the TCP stream.
    stream.write_all(response.as_bytes()).unwrap();
}
