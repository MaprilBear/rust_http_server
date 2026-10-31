mod http_utils;
mod serializable;
mod slice_utils;

use crate::http_utils::{ContentType, generate_http_response, parse_http_request};
use http::{Request, Response, Version};
use router::{route, route_request};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[route]
fn foo(request: Request<&[u8]>) -> Response<Vec<u8>> {
    let content_type = match request.headers().get("Content-Type") {
        Some(content_type) => match content_type.to_str().unwrap_or("") {
            "text/plain" => ContentType::PlainText,
            "application/json" => ContentType::Json,
            "application/octet-stream" => ContentType::Binary,
            _ => ContentType::Binary,
        },
        None => ContentType::Binary,
    };

    let content = if let ContentType::PlainText = content_type {
        str::from_utf8(request.body()).unwrap().trim_end_matches('\0')
    } else {
        "uh oh!"
    };

    let body: Vec<u8> = format!("<html><body>{}</body></html>", content).into_bytes();
    Response::builder()
        .header("Context-Type", "text/html")
        .header("Content-Length", body.len())
        .status(200)
        .version(Version::HTTP_11)
        .body(body)
        .unwrap()
}

fn handle_client(mut stream: TcpStream) {
    stream.set_read_timeout(Some(Duration::from_millis(100))).expect("Failed to set read timeout");
    // Grab the request data
    let mut request_buffer: [u8; 1024] = [0; 1024];
    let bytes = stream.read(&mut request_buffer);
    if bytes.is_err() {
        return;
    }

    let request = parse_http_request(&request_buffer);
    let response = route_request(request);
    let response_text = generate_http_response(response);
    stream.write_all(response_text.as_slice()).unwrap();
    stream.flush().unwrap()
}

fn main() {
    // Create TCP socket for HTTP
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
