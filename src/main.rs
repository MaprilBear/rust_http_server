use http::{Response, Version};
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    stream.set_nonblocking(false).unwrap();
    let body = "<html><body>hi!</body></html>";
    let response = Response::builder()
        .header("Context-Type", "text/html")
        .header("Content-Length", body.len().to_string())
        .status(200)
        .version(Version::HTTP_11)
        .body("<html><body>hi!</body></html>")
        .unwrap();
    let version_string = match response.version() {
        Version::HTTP_10 => "HTTP/1.0",
        _ => "HTTP/1.1",
    };

    stream
        .write(format!("{} {} {}\r\n", version_string, response.status().as_str(), "OK").as_bytes())
        .unwrap();
    for header in response.headers() {
        stream.write(format!("{}: {}\r\n", header.0.as_str(), header.1.to_str().unwrap()).as_bytes()).unwrap();
    }
    stream.write(b"\r\n").unwrap();
    stream.write(body.as_bytes()).unwrap();
    stream.write(b"\r\n").unwrap();
    stream.flush().unwrap();
}

fn main() {
    // Create TCP socket for HTTP
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
