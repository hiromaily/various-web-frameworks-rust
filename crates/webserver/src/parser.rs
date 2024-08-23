use crate::request;
use httparse::EMPTY_HEADER;
use log::debug;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use url::Url;

#[allow(dead_code)]
fn get_host<'a>(headers: &'a [httparse::Header<'a>; 16]) -> &'a str {
    headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case("Host"))
        .map(|header| str::from_utf8(header.value).unwrap_or(""))
        .unwrap_or("")
}

fn get_request_body(buffer: &[u8]) -> Option<String> {
    // .ok() method converts the Result into an Option
    let request_str = str::from_utf8(buffer).ok()?;
    let body_start = request_str.find("\r\n\r\n")? + 4;
    if request_str[body_start..].is_empty() {
        return None;
    }
    Some(request_str[body_start..].to_string())
}

pub fn get_query_parameters(path: &str) -> Option<(&str, &str)> {
    // sprit path into path and query by `?`
    // if let Some(pos) = path.find('?') {
    //     Some((&path[..pos], &path[pos + 1..]))
    // } else {
    //     None
    // }
    path.find('?').map(|pos| (&path[..pos], &path[pos + 1..]))
}

// # Examples
//
// ```
// parser::parse_url(format!("http://dummy.com{}", path).as_str());
// ```
#[allow(dead_code)]
fn parse_url(url_path: &str) {
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

// get_req_info() returns Request
pub fn get_req_info(mut stream: &TcpStream) -> anyhow::Result<Option<request::Request>> {
    // stream size may be bigger than 1024
    // let mut buffer = [0; 1024];
    // let bytes_read = stream.read(&mut buffer)?;
    // if bytes_read == 0 {
    //     println!("connection closed by client.");
    //     return Ok(None);
    // }

    // `stream.read_to_end()` waits for the connection to be closed before returning
    // let mut buffer = Vec::new();
    // let bytes_read = stream.read_to_end(&mut buffer)?;

    const CHUNK_SIZE: usize = 512;
    let mut buffer = Vec::new();
    let mut chunk = [0; CHUNK_SIZE];
    let mut bytes_read: usize = 0;
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => {
                break;
            }
            Ok(chunk_bytes_read) => {
                debug!("Read {} bytes", chunk_bytes_read);
                bytes_read += chunk_bytes_read;
                buffer.extend_from_slice(&chunk[..chunk_bytes_read]);
                if CHUNK_SIZE > chunk_bytes_read {
                    break;
                }
            }
            Err(e) => {
                debug!("Error reading stream: {}", e);
                break;
            }
        }
    }

    // handle headers
    let mut headers = [EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buffer)?;

    // get method and path
    let method = req.method.unwrap_or("").to_string();
    let path = req.path.unwrap_or("");

    // get headers
    let mut headers_map = HashMap::new();
    for header in req.headers.iter() {
        let name = header.name.to_string();
        let value = std::str::from_utf8(header.value)?.to_string();
        headers_map.insert(name, value);
    }

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
        get_request_body(&buffer[..bytes_read])
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

    Ok(Some(request::Request::new(
        method,
        path.to_string(),
        headers_map,
        query,
        body,
    )))
}
