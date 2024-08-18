use crate::request;
use std::collections::HashMap;

type Handler = fn(&request::Request) -> anyhow::Result<String>;

pub struct Router {
    routes: HashMap<(String, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.routes
            .insert(("GET".to_string(), path.to_string()), handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.routes
            .insert(("POST".to_string(), path.to_string()), handler);
    }

    pub fn route(&self, method: &str, path: &str) -> Option<&Handler> {
        self.routes.get(&(method.to_string(), path.to_string()))
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
