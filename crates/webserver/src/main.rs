use log::info;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// local
use webserver::handler;
use webserver::middleware;
use webserver::parser;
use webserver::router;

fn handle_connection(mut stream: TcpStream, router: &router::Router) -> anyhow::Result<()> {
    // Call get_method_path and handle each scenario
    // let (method, path, query, body) = match parser::get_req_info(&stream)? {
    //     Some((method, path, query, body)) => (method, path, query, body), // Continue if method and path are present
    //     None => return Ok(()), // Exit early if None is returned
    // };
    let request = match parser::get_req_info(&stream)? {
        Some(req) => req,
        None => return Ok(()),
    };
    request.print();

    // middleware
    router.run_middleware(&request)?;

    // handler
    let handler = router.route(&request.method, &request.path);
    match handler {
        Some(h) => {
            let response = h(&request)?;
            stream.write_all(response.as_bytes())?;
        }
        None => {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>404 Not Found</h1>";
            stream.write_all(response.as_bytes())?;
        }
    }
    // match (method.as_ref(), path.as_ref()) {
    //     ("GET", "/") => {
    //         let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello, GET!</h1>";
    //         stream.write_all(response.as_bytes()).unwrap();
    //     }
    //     ("POST", "/submit") => {
    //         println!("Received POST data: {}", body.as_ref().unwrap());
    //         let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Post Data Received</h1>";
    //         stream.write_all(response.as_bytes()).unwrap();
    //     }
    //     _ => {
    //         let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n<h1>404 Not Found</h1>";
    //         stream.write_all(response.as_bytes()).unwrap();
    //     }
    // }

    stream.flush()?;
    Ok(())
}

fn main() {
    env_logger::init();

    // configure router
    let mut router = router::Router::new();
    router.add_middleware(middleware::ContentTypeMiddleware);
    router.get("/", handler::handler_a);
    router.post("/submit", handler::handler_b);

    // initialize server
    let addr = "127.0.0.1:8080";
    info!("run web server on {addr}");
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let _ = handle_connection(stream, &router);
    }
}
