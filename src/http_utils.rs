use crate::serializable::Serializable;
use crate::slice_utils::{split_bytes, split_bytes_once};
use http::{Request, Response, Version};
use std::io::Write;

pub fn parse_http_request(request_data: &[u8]) -> Request<&[u8]> {
    let (preamble_and_headers, body) =
        split_bytes_once(request_data, b"\r\n\r\n").expect("Found more than one double newline");

    let (preamble, headers) =
        split_bytes_once(preamble_and_headers, b"\r\n").expect("Expected a single header");

    let mut request = Request::builder();
    let [method, target, version]: [&str; 3] = str::from_utf8(preamble)
        .unwrap()
        .splitn(3, " ")
        .collect::<Vec<&str>>()
        .try_into()
        .expect("Expected a method, target, and http version in the request line");
    request = request.method(method);
    request = request.uri(target);
    request = request.version(match version {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2.0" => Version::HTTP_2,
        "HTTP/3.0" => Version::HTTP_3,
        _ => Version::HTTP_10,
    });
    for header in split_bytes(headers, b"\r\n") {
        let (key, value) = split_bytes_once(header, b": ").expect("\": \" present in header line");
        request = request.header(key, value);
    }

    request.body(body).expect("Couldn't parse HTTP body")
}

/// Generate an HTTP response ready to send to the client from a `Response` object
pub fn generate_http_response<T: Serializable>(response: Response<T>) -> Vec<u8> {
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

pub enum ContentType {
    PlainText,
    Json,
    Binary,
}
