mod http_utils;
mod slice_utils;

use crate::http_utils::{generate_http_response, parse_http_request};
use http::{Request, Response};
use router::{route, route_request};
use serde::Serialize;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[route(uri = "/", method = "GET")]
fn foo(request: Request<&[u8]>) -> Response<Vec<u8>> {
    // let content_type = match request.headers().get("Content-Type") {
    //     Some(content_type) => match content_type.to_str().unwrap_or("") {
    //         "text/plain" => ContentType::PlainText,
    //         "application/json" => ContentType::Json,
    //         "application/octet-stream" => ContentType::Binary,
    //         _ => ContentType::Binary,
    //     },
    //     None => ContentType::Binary,
    // };

    let index_html = std::fs::read_to_string("resources/index.html").unwrap();
    let output = index_html.into_bytes();
    Response::builder()
        .header("Context-Type", "text/html")
        .header("Content-Length", output.len())
        .body(output)
        .unwrap()
}

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
    id: u64,
}

#[route(uri = "/user", method = "GET")]
fn get_user(request: Request<&[u8]>) -> Response<Vec<u8>> {
    let user = User { username: "bob".into(), email: "bob@gmail.com".into(), id: 1 };
    let user_json = serde_json::to_string(&user).unwrap_or_else(|e| {
        eprintln!("Serialization failed: {}", e);
        String::new()
    });

    Response::builder()
        .header("Context-Type", "application/json")
        .header("Content-Length", user_json.len())
        .body(user_json.into_bytes()).unwrap()
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
