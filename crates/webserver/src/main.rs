use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// local
use webserver::parser;

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    // Call get_method_path and handle each scenario
    // let (method, path, query, body) = match parser::get_req_info(&stream)? {
    //     Some((method, path, query, body)) => (method, path, query, body), // Continue if method and path are present
    //     None => return Ok(()), // Exit early if None is returned
    // };
    let request = match parser::get_req_info(&stream)? {
        Some(req) => req,      // Continue if the request is present
        None => return Ok(()), // Exit early if None is returned
    };
    let method = &request.method;
    let path = &request.path;
    let query = &request.query;
    let body = &request.body;

    println!(
        "get_method_path(): \n method:{:?}, path:{}, query: {:?}, body:{:?}",
        method, path, query, body,
    );

    // handler
    match (method.as_ref(), path.as_ref()) {
        ("GET", "/") => {
            let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello, GET!</h1>";
            stream.write_all(response.as_bytes()).unwrap();
        }
        ("POST", "/submit") => {
            // Parse the body
            //if let Some(body) = parser::get_request_body(&buffer) {
            if body.is_none() {
                let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\n<h1>Bad Request</h1>";
                stream.write_all(response.as_bytes()).unwrap();
            } else {
                println!("Received POST data: {}", body.as_ref().unwrap());
                let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Post Data Received</h1>";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
        _ => {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>404 Not Found</h1>";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }

    stream.flush()?;
    Ok(())
}

fn main() {
    let addr = "127.0.0.1:8080";
    println!("run web server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let _ = handle_connection(stream);
    }
}
