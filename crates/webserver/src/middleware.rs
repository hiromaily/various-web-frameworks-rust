use crate::errors;
use crate::request;
use log::debug;

pub trait Middleware {
    fn handle(&self, req: &request::Request) -> anyhow::Result<(), errors::HTTPErrorMessage>;
}

pub struct ContentTypeMiddleware;

impl Middleware for ContentTypeMiddleware {
    fn handle(&self, req: &request::Request) -> anyhow::Result<(), errors::HTTPErrorMessage> {
        debug!("ContentTypeMiddleware is called");

        if let Some(content_type) = req.headers.get("Content-Type") {
            if content_type != "application/json" {
                debug!("Invalid Content-Type");
                //return Err(anyhow::anyhow!("Missing Content-Type."));
                return Err(errors::HTTPErrorMessage::UnsupportedContentType);
            }
        } else {
            debug!("Missing Content-Type");
            //return Err(anyhow::anyhow!("Missing Content-Type."));
            return Err(errors::HTTPErrorMessage::MissingHeader(
                "Content-Type".to_string(),
            ));
        }

        Ok(())
    }
}
