use crate::request;
use log::debug;

pub trait Middleware {
    fn handle(&self, req: &request::Request) -> anyhow::Result<()>;
}

pub struct ContentTypeMiddleware;

impl Middleware for ContentTypeMiddleware {
    fn handle(&self, req: &request::Request) -> anyhow::Result<()> {
        debug!("ContentTypeMiddleware is called");
        //unimplemented!("TODO");

        if let Some(content_type) = req.headers.get("Content-Type") {
            if content_type != "application/json" {
                debug!("Invalid Content-Type");
                return Err(anyhow::anyhow!("Missing Content-Type."));
            }
        } else {
            debug!("Missing Content-Type");
            return Err(anyhow::anyhow!("Missing Content-Type."));
        }

        Ok(())
    }
}
