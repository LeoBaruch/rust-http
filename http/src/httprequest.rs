use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Version {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resoure {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resoure,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resoure::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            // println!("line: {:?}", line);
            if line.contains("HTTP") {
                let (method, resource, version) = parse_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = parse_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn parse_req_line(s: &str) -> (Method, Resoure, Version) {
    let mut words = s.split_whitespace();

    let method = words.next().unwrap();
    let resoure = words.next().unwrap();
    let version = words.next().unwrap();
    // println!("method111: {:?}", method);

    (
        method.into(),
        Resoure::Path(resoure.to_string()),
        version.into(),
    )
}

fn parse_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = "".to_string();
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();

        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();

        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn tesst_read_http() {
        let req_origin = "\
GET /greeting HTTP/1.1
Host: localhost:3000
User-Agent: curl/7.71.1
Accept: */*
";

        let s = String::from(req_origin);
        let req: HttpRequest = s.into();
        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        headers_expected.insert("Accept".into(), " */*".into());

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resoure::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
