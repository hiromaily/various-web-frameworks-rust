use crate::errors;
use crate::request;

pub trait Middleware {
    fn handle(&self, req: &request::Request) -> anyhow::Result<(), errors::HTTPErrorMessage>;
}

// ContentType validator
pub struct ContentTypeMiddleware;

impl Middleware for ContentTypeMiddleware {
    fn handle(&self, req: &request::Request) -> anyhow::Result<(), errors::HTTPErrorMessage> {
        // check only POST method
        if req.method != "POST" {
            return Ok(());
        }
        if let Some(content_type) = req.headers.get("Content-Type") {
            if content_type != "application/json" {
                //return Err(anyhow::anyhow!("Missing Content-Type."));
                return Err(errors::HTTPErrorMessage::UnsupportedContentType);
            }
        } else {
            //return Err(anyhow::anyhow!("Missing Content-Type."));
            return Err(errors::HTTPErrorMessage::MissingHeader(
                "Content-Type".to_string(),
            ));
        }

        Ok(())
    }
}
