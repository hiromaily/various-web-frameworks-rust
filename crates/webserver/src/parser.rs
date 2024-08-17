use std::str;

// FIXME: expected named lifetime parameter
//fn extract_host_header(headers: &[httparse::Header<'_>; 16]) -> &str {
#[allow(dead_code, unused_variables)]
pub fn extract_host_header<'a>(headers: &'a [httparse::Header<'a>; 16]) -> &'a str {
    headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case("Host"))
        .map(|header| str::from_utf8(header.value).unwrap_or(""))
        .unwrap_or("")
}

// retrieve query from path
pub fn get_query_parameters(path: &str) -> Option<(&str, &str)> {
    // sprit path into path and query by `?`
    if let Some(pos) = path.find('?') {
        Some((&path[..pos], &path[pos + 1..]))
    } else {
        None
    }
}

// get request body for post request
pub fn get_request_body(buffer: &[u8]) -> Option<String> {
    let request_str = str::from_utf8(buffer).ok()?;
    let body_start = request_str.find("\r\n\r\n")? + 4;
    Some(request_str[body_start..].to_string())
}
