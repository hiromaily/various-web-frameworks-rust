use crate::request;
use std::collections::HashMap;

type Handler = fn(&request::Request) -> anyhow::Result<String>;

pub struct Router {
    route_map: HashMap<(String, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            route_map: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.route_map
            .insert(("GET".to_string(), path.to_string()), handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.route_map
            .insert(("POST".to_string(), path.to_string()), handler);
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.route_map
            .insert(("PUT".to_string(), path.to_string()), handler);
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.route_map
            .insert(("DELETE".to_string(), path.to_string()), handler);
    }

    pub fn route(&self, method: &str, path: &str) -> Option<&Handler> {
        self.route_map.get(&(method.to_string(), path.to_string()))
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
