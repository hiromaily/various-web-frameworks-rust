use thiserror::Error;

#[derive(Debug, Error)]
pub enum HTTPErrorMessage {
    #[error("Bad Request")]
    BadRequest,

    #[error("Invalid request format")]
    InvalidRequestFormat,

    #[error("Missing required header: {0}")]
    MissingHeader(String),

    #[error("Not Found")]
    NotFound,

    #[error("Unsupported content type")]
    UnsupportedContentType,
}

impl HTTPErrorMessage {
    pub fn status_code(&self) -> u16 {
        match self {
            HTTPErrorMessage::BadRequest => 400,
            HTTPErrorMessage::InvalidRequestFormat => 400,
            HTTPErrorMessage::MissingHeader(_) => 400,
            HTTPErrorMessage::NotFound => 404,
            HTTPErrorMessage::UnsupportedContentType => 415,
        }
    }

    pub fn response(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n\r\n<h1>{} {}</h1>\r\n",
            self.status_code(),
            self,
            self.status_code(),
            self
        )
    }
}
