use crate::errors::HTTPErrorMessage;
use crate::request;
use log::debug;

// TODO: implement responser

pub fn handler_a(req: &request::Request) -> anyhow::Result<String> {
    if let Some(query) = &req.query {
        debug!(" received query: {}", query);
    }
    let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Hello, GET!</h1>\r\n".to_string();
    Ok(response)
}

pub fn handler_b(req: &request::Request) -> anyhow::Result<String> {
    if let Some(body) = &req.body {
        debug!(" Received POST data: {}", body);
        let response = "HTTP/1.1 200 OK\r\n\r\n<h1>Post Data Received</h1>\r\n".to_string();
        Ok(response)
    } else {
        let response = HTTPErrorMessage::BadRequest.response();
        Ok(response)
    }
}
