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

    pub fn print(&self) {
        // let method = &request.method;
        // let path = &request.path;
        // let query = &request.query;
        // let body = &request.body;
        println!(
            "get_method_path(): \n method:{:?}, path:{}, query: {:?}, body:{:?}",
            self.method, self.path, self.query, self.body,
        );
    }
}
