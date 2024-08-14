use httparse::{Request, EMPTY_HEADER};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    if bytes_read == 0 {
        println!("connection closed by client.");
        return;
    }

    // handle headers
    let mut headers = [EMPTY_HEADER; 16];
    let mut req = Request::new(&mut headers);
    req.parse(&buffer).unwrap();

    let method = req.method.unwrap_or("");
    let path = req.path.unwrap_or("");
    println!("got a request: method: {}, path: {}", method, path);

    // handler
    match (method, path) {
        ("GET", "/") => {
            let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello, GET!</h1>";
            stream.write(response.as_bytes()).unwrap();
        }
        ("POST", "/submit") => {
            // Parse the body
            if let Some(body) = get_request_body(&buffer) {
                println!("Received POST data: {}", body);
                let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Post Data Received</h1>";
                stream.write(response.as_bytes()).unwrap();
            } else {
                let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\n<h1>Bad Request</h1>";
                stream.write(response.as_bytes()).unwrap();
            }
        }
        _ => {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>404 Not Found</h1>";
            stream.write(response.as_bytes()).unwrap();
        }
    }

    stream.flush().unwrap();
}

fn get_request_body(buffer: &[u8]) -> Option<String> {
    let request_str = str::from_utf8(buffer).ok()?;
    let body_start = request_str.find("\r\n\r\n")? + 4;
    Some(request_str[body_start..].to_string())
}

fn main() {
    let addr = "127.0.0.1:8080";
    println!("run web server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
