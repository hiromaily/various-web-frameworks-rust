use httparse::{Request, EMPTY_HEADER};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use url::Url;

// local
use webserver::parser;

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

    // get method and path
    let method = req.method.unwrap_or("");
    let path = req.path.unwrap_or("");
    let host = parser::extract_host_header(&headers);
    println!(
        "got a request: host: {}, method: {}, path: {}",
        host, method, path
    );

    // debug: Url::parse
    let divided_url = Url::parse(format!("http://{}{}", host, path).as_str()).unwrap();
    println!(
        "parsed path: \n schema:{}, host:{:?}, path:{}, query:{:?}",
        divided_url.scheme(),
        divided_url.host(),
        divided_url.path(),
        divided_url.query()
    );

    // retrieve query from path
    let (path, query) = if let Some((path, query)) = parser::get_query_parameters(path) {
        (path, Some(query))
    } else {
        (path, None)
    };
    println!("parse a request path: path: {}, query: {:?}", path, query);

    // handler
    match (method, path) {
        ("GET", "/") => {
            let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello, GET!</h1>";
            stream.write(response.as_bytes()).unwrap();
        }
        ("POST", "/submit") => {
            // Parse the body
            if let Some(body) = parser::get_request_body(&buffer) {
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

fn main() {
    let addr = "127.0.0.1:8080";
    println!("run web server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
