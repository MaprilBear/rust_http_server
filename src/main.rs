mod serializable;

use crate::serializable::Serializable;
use http::{Response, Version};
use std::io::Write;
use std::net::{TcpListener, TcpStream};

/// Generate an HTTP response ready to send to the client from a `Response` object
fn generate_http_response<T: Serializable>(response: Response<T>) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(256);

    // Preamble -> HTTP/1.0 200 OK
    let version = match response.version() {
        Version::HTTP_09 => "HTTP/0.9",
        Version::HTTP_10 => "HTTP/1.0",
        Version::HTTP_11 => "HTTP/1.1",
        Version::HTTP_2 => "HTTP/2",
        Version::HTTP_3 => "HTTP/3",
        _ => "HTTP/1.0",
    };
    let status = response.status();
    let status_code = status.as_str();
    let reason = status.canonical_reason().unwrap_or("Unknown");
    write!(buffer, "{version} {status_code} {reason}\r\n").expect("Failed to generate preamble");

    // Headers -> Content-Type: text/plain
    //            Content-Length: 999
    //            etc...
    for (k, v) in response.headers() {
        buffer.extend_from_slice(k.as_str().as_bytes());
        buffer.extend_from_slice(b": ");
        buffer.extend_from_slice(v.as_bytes());
        buffer.extend_from_slice(b"\r\n");
    }
    buffer.extend_from_slice(b"\r\n");

    // Body
    buffer.extend(response.body().serialize());

    buffer
}

fn handle_client(mut stream: TcpStream) {
    stream.set_nonblocking(false).unwrap();
    let body: String = "<html><body>hi!</body></html>".to_string();
    let response = Response::builder()
        .header("Context-Type", "text/html")
        .header("Content-Length", body.as_bytes().len())
        .status(200)
        .version(Version::HTTP_11)
        .body(body)
        .unwrap();
    let response_text = generate_http_response(response);
    stream.write(response_text.as_slice()).unwrap();
    stream.flush().unwrap()
}

fn main() {
    // Create TCP socket for HTTP
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
