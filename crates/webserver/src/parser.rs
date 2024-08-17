use httparse::EMPTY_HEADER;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use url::Url;

pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub body: Option<String>,
}

impl Request {
    fn new(method: String, path: String, query: Option<String>, body: Option<String>) -> Self {
        Request {
            method,
            path,
            query,
            body,
        }
    }
}

// FIXME: expected named lifetime parameter
//fn extract_host_header(headers: &[httparse::Header<'_>; 16]) -> &str {
#[allow(dead_code, unused_variables)]
fn get_host<'a>(headers: &'a [httparse::Header<'a>; 16]) -> &'a str {
    headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case("Host"))
        .map(|header| str::from_utf8(header.value).unwrap_or(""))
        .unwrap_or("")
}

// get request body for post request
fn get_request_body(buffer: &[u8]) -> Option<String> {
    let request_str = str::from_utf8(buffer).ok()?;
    let body_start = request_str.find("\r\n\r\n")? + 4;
    Some(request_str[body_start..].to_string())
}

// retrieve query from path
pub fn get_query_parameters(path: &str) -> Option<(&str, &str)> {
    // sprit path into path and query by `?`
    // if let Some(pos) = path.find('?') {
    //     Some((&path[..pos], &path[pos + 1..]))
    // } else {
    //     None
    // }
    path.find('?').map(|pos| (&path[..pos], &path[pos + 1..]))
}

pub fn parse_url(url_path: &str) {
    // dummy host is enough to retrieve to parse
    //let divided_url = Url::parse(format!("http://{}{}", host, path).as_str()).unwrap();
    let divided_url = Url::parse(url_path).unwrap();
    println!(
        "parsed path: \n schema:{}, host:{:?}, path:{}, query:{:?}",
        divided_url.scheme(),
        divided_url.host(),
        divided_url.path(),
        divided_url.query()
    )
}

// get_req_info() returns
// # Examples
//
// ```
// parser::parse_url(format!("http://dummy.com{}", path).as_str());
// ```
pub fn get_req_info(mut stream: &TcpStream) -> anyhow::Result<Option<Request>> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        println!("connection closed by client.");
        return Ok(None);
    }

    // handle headers
    let mut headers = [EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buffer)?;

    // get method and path
    let method = req.method.unwrap_or("").to_string();
    let path = req.path.unwrap_or("");

    // get query
    // can be replaced by `Url::parse``
    let (path, query) = if let Some((path, query)) = get_query_parameters(path) {
        (path, Some(query))
    } else {
        (path, None)
    };
    // convert Option<&str> to Option<String>
    let query = query.map(|s| s.to_string());

    // get body
    let body: Option<String> = if method == "POST" {
        get_request_body(&buffer)
    } else {
        None
    };

    // actually, no needs to host from header
    // this is just for study
    // let host = parser::extract_host_header(&headers);
    // println!(
    //     "got a request: host: {}, method: {}, path: {}",
    //     host, method, path
    // );

    Ok(Some(Request::new(method, path.to_string(), query, body)))
}
