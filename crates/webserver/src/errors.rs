use thiserror::Error;

#[derive(Debug, Error)]
pub enum HTTPErrorMessage {
    #[error("Unsupported content type")]
    UnsupportedContentType,

    #[error("Invalid request format")]
    InvalidRequestFormat,

    #[error("Missing required header: {0}")]
    MissingHeader(String),
}

impl HTTPErrorMessage {
    pub fn status_code(&self) -> u16 {
        match self {
            HTTPErrorMessage::UnsupportedContentType => 415,
            HTTPErrorMessage::InvalidRequestFormat => 400,
            HTTPErrorMessage::MissingHeader(_) => 400,
        }
    }
}
