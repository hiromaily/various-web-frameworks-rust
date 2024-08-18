use crate::request;

pub fn handler_a(req: &request::Request) -> anyhow::Result<String> {
    if let Some(query) = &req.query {
        println!("Received query: {}", query);
    }
    let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello, GET!</h1>".to_string();
    Ok(response)
}

pub fn handler_b(req: &request::Request) -> anyhow::Result<String> {
    if let Some(body) = &req.body {
        println!("Received POST data: {}", body);
        let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Post Data Received</h1>".to_string();
        Ok(response)
    } else {
        let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\n<h1>Bad Request</h1>".to_string();
        Ok(response)
    }
}
