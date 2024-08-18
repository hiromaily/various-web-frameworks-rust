pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(method: String, path: String, query: Option<String>, body: Option<String>) -> Self {
        Request {
            method,
            path,
            query,
            body,
        }
    }
}
