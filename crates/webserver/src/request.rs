use std::collections::HashMap;

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

    pub fn print(&self) {
        // let method = &request.method;
        // let path = &request.path;
        // let query = &request.query;
        // let body = &request.body;
        println!(
            "get_method_path(): \n method:{:?}, path:{}, headers: {:?}, query: {:?}, body:{:?}",
            self.method, self.path, self.headers, self.query, self.body,
        );
    }
}
