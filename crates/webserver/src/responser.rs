use crate::errors::HTTPErrorMessage;

pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");
        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}\r\n",
            self.status_code,
            get_status_message(self.status_code),
            headers,
            self.body
        )
    }

    pub fn html(status_code: u16, body: &str) -> Self {
        Self {
            status_code,
            headers: vec![("Content-Type".to_string(), "text/html".to_string())],
            body: body.to_string(),
        }
    }

    pub fn json(status_code: u16, body: &str) -> Self {
        Self {
            status_code,
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: body.to_string(),
        }
    }

    pub fn error_html(error_message: &HTTPErrorMessage) -> Self {
        Self::html(
            error_message.status_code(),
            &format!("<h1>{} {}</h1>", error_message.status_code(), error_message),
        )
    }

    // WIP: adjustment
    pub fn error_json(status_code: u16, message: &str) -> Self {
        Self::json(status_code, message)
    }
}

fn get_status_message(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        415 => "Unsupported Media Type",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}
