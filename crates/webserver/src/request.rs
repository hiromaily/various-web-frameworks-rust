use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: Option<String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(
        method: String,
        path: String,
        headers: HashMap<String, String>,
        query: Option<String>,
        body: Option<String>,
    ) -> Self {
        Request {
            method,
            path,
            headers,
            query,
            body,
        }
    }
}
