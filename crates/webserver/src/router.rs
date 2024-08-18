use crate::middleware;
use crate::request;
use crate::responser::Response;
use std::collections::HashMap;

type Handler = fn(&request::Request) -> anyhow::Result<Response>;

pub struct Router {
    route_map: HashMap<(String, String), Handler>,
    middlewares: Vec<Box<dyn middleware::Middleware>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            route_map: HashMap::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.route_map
            .insert(("GET".to_string(), path.to_string()), handler);
    }
    // pub fn get<F>(&mut self, path: &str, handler: F)
    // where
    //     F: Fn(&request::Request, &mut TcpStream) -> anyhow::Result<()> + 'static,
    // {
    //     self.routes.insert(path.to_string(), Box::new(handler));
    // }

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

    // add middleware
    pub fn add_middleware<M: middleware::Middleware + 'static>(&mut self, middleware: M) {
        self.middlewares.push(Box::new(middleware));
    }

    // route finds handler specified by method and path
    pub fn route(&self, method: &str, path: &str) -> Option<&Handler> {
        self.route_map.get(&(method.to_string(), path.to_string()))
    }

    pub fn run_middleware(&self, req: &request::Request) -> anyhow::Result<()> {
        // Apply middleware first
        for middleware in &self.middlewares {
            middleware.handle(req)?;
        }
        Ok(())
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
