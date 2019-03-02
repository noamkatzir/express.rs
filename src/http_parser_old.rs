use super::types::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::io::BufRead;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Request {
    method: MethodKind,
    uri: String,
    query: Option<QueryString>,
    protocol: HttpVersion,
    headers: Option<HashMap<String,String>>
}

pub struct RequestBuilder {
    method: Option<MethodKind>,
    uri: Option<String>,
    query: Option<QueryString>,
    protocol: Option<HttpVersion>,
    headers: Option<HashMap<String,String>>
}

impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder { 
            method: None, 
            uri: None, 
            query: None, 
            protocol: None, 
            headers: None
        }
    }
    
    pub fn method(&mut self, m: MethodKind) -> &mut Self { self.method = Some(m); self }
    pub fn uri(&mut self, u: String) -> &mut Self { self.uri = Some(u); self }
    pub fn query(&mut self, q: QueryString) -> &mut Self { self.query = Some(q); self }
    pub fn protocol(&mut self, p: HttpVersion) -> &mut Self { self.protocol = Some(p); self }
    pub fn headers(&mut self, h: HashMap<String,String>) -> &mut Self { self.headers = Some(h); self }

    pub fn build(self) -> Request {
        Request {
            method: self.method.unwrap(),
            uri: self.uri.unwrap(),
            query: self.query,
            protocol: self.protocol.unwrap(),
            headers: self.headers
        }
    }
    
}

impl Request {
    fn parse_method(reader: &mut BufReader<&mut TcpStream>) -> Option<(MethodKind,String,HttpVersion)> {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Err(_) | Ok(0) => None,
            Ok(_n) => {
                match parse_method(&buf) {
                    Err(_) => None,
                    Ok(tup) => Some(tup)
                }
            }
        }
    }

    fn parse_headers(reader: &mut BufReader<&mut TcpStream>) -> HashMap<String,String> {
        let mut headers: HashMap<String,String> = HashMap::new();
        loop {
            let mut buf = String::new();
            match reader.read_line(&mut buf) {
                Err(_) | Ok(0) => continue,
                Ok(_n) => {
                    if buf.ends_with("\n") {
                        buf.pop();
                        if buf.ends_with("\r") {
                            buf.pop();
                        }
                    }
                    if buf.is_empty() {
                        break;
                    }
                    
                    let pos = buf.find(":").unwrap();
                    let (name,value) = buf.split_at(pos);
                    let name = name.trim();
                    let value = &value[1..].trim();
                    headers.insert(name.to_string(),value.to_string());
                }
            }
        };

        headers
    }

    pub fn parse(stream: &mut TcpStream) -> Option<Request> {
        let mut builder = RequestBuilder::new();
        let mut reader = BufReader::new(stream);

        let url = match Request::parse_method(&mut reader) {
            Some((method,url,protocol)) => {
                builder.method(method).protocol(protocol);
                url
            },
            _ => return None
        };

        let headers = Request::parse_headers(&mut reader);
        builder.headers(headers);
        

        let split_query_string = url.split("?").collect::<Vec<&str>>();
        match split_query_string.len() {
            2 => {
                match QueryString::parse(split_query_string[1]) {
                    Some(query) => { builder.query(query); },
                    _ => {}
                };
                builder.uri(split_query_string[0].to_string());

            },
            1 => {
                builder.uri(split_query_string[0].to_string());
            },
            _ => {}
        };

        Some(builder.build())
    }

    pub fn parse2(stream: &mut TcpStream) -> Option<Request> {
        let mut builder = RequestBuilder::new();
        let mut buffer = [0u8; 80*1024];
        loop {
            let bytes = match stream.read(&mut buffer) {
                Err(_) | Ok(0) => continue,
                Ok(n) => n
            };

            

        }
        None
    }

    pub fn get_parsed_header(&self, name: &str) -> Option<HttpHeader> {
        match self.get_header(name) {
            Some(value) => {
                match parse_header(name,&value) {
                    Ok(header) => Some(header),
                    _ => None
                }
            },
            _ => None
        }
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        match self.headers {
            Some(ref headers) => {
                match headers.get(name) {
                    Some(value) => Some(value),
                    _ => None
                }
            },
            _ => None
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn query(&self, key: &str) -> Option<&QueryStringParam> {
        match self.query {
            Some(ref query) => query.get(key),
            None => None
        }
    }
}

fn parse_accept(value : &str) -> Vec<(String,f32)> {
    let mime_types: Vec<&str> = value.rsplit(",").collect();
    let mut rank = 1.0;
    let mut res = vec![];
    for mime_type in mime_types {
        let type_and_rank: Vec<&str> = mime_type.split(";q=").collect();

        match type_and_rank.len() {
            1 => res.push((type_and_rank[0].to_string(), rank)),
            2 => {
                rank = type_and_rank[1].parse().unwrap();
                res.push((type_and_rank[0].to_string(), rank))
            },
            _ => continue
        }

    };

    res
}

fn parse_method(buf: &str) -> Result<(MethodKind,String,HttpVersion), String> {
    let method = buf.split_whitespace().take(3).collect::<Vec<&str>>();
    if let [method, uri, proto] = &method[..] {
        let protocol = match &**proto {
            "HTTP/1.1" => HttpVersion::V1,
            "HTTP/1.0" => HttpVersion::V1,
            _ => return Err(format!("unknoun HTTP version: {}",proto))
        };
        let name = match &**method {
            "GET" => MethodKind::Get,
            "POST" => MethodKind::Post,
            "PUT" => MethodKind::Put,
            "DELETE" => MethodKind::Delete,
            _ => return Err("unknoun HTTP Method".to_string())
        };
        Ok((name, uri.to_string(), protocol))
    } else {
        Err(format!("broken method line {}",buf))
    }
}

fn parse_header(name: &str,value: &str) -> Result<HttpHeader, String> {
    let name = name.to_lowercase();
    let name = &name[..];
    Ok(match name {
        "host" => {
            let host: Vec<&str> = value.split(":").collect();
            HttpHeader::Host{name: host[0].to_string(), port: host[1].parse().unwrap() }
        },
        "user-agent" => HttpHeader::UserAgent(value.to_string()),
        "accept" => HttpHeader::Accept(parse_accept(value)),
        "referer" => HttpHeader::Referer(value.to_string()),
        "pragma" => HttpHeader::Pragma(value.to_string()),
        "accept-language" => HttpHeader::AcceptLanguage(parse_accept(value)),
        "accept-encoding" => HttpHeader::AcceptEncoding(value.split(",").map(|x| x.trim().to_string()).collect()),
        "connection" =>  {
            match value {
                "keep-alive" => HttpHeader::Connection(Connection::KeepAlive),
                _ => HttpHeader::Connection(Connection::Close),
            }
        },
        "upgrade-insecure-requests" =>  HttpHeader::UpgradeInsecureRequests,
        "cache-control" => {
            let cache_control: Vec<&str> = value.split("=").collect();
            println!("cache control value: {}", value);
                
            match cache_control.len() {
                2 => HttpHeader::CacheControl(CacheControl::MaxAge(cache_control[1].parse().unwrap())),
                _ => HttpHeader::CacheControl(CacheControl::MaxAge(0))
            }
        },
        _ => return Err(format!("unknoun HTTP header {} => {}",name,value))
    })
}



pub struct QueryString(HashMap<String,QueryStringParam>);
pub enum QueryStringParam {
    SimpleParam(String),
    ArrayParam(Vec<String>)
}

impl QueryString {
    pub fn get(&self, key: &str) -> Option<&QueryStringParam> {
        self.0.get(key)
    }

    fn parse(query_string: &str) -> Option<QueryString> {
        let mut query_map: HashMap<String,QueryStringParam> = HashMap::new();
        for param in query_string.split("&") {
            let segments = param.split("=").collect::<Vec<&str>>();

            if segments.len() != 2 {
                return None;
            }

            let is_array = segments[0].ends_with("[]");
            let key = if is_array { 
                &segments[0][..(segments[0].len()-2)] 
            } else {
                segments[0]
            };
            let value = segments[1];

            match query_map.get_mut(key) {
                Some(QueryStringParam::ArrayParam(ref mut container)) if is_array => {
                    container.push(value.to_string());
                    continue;
                },
                _ => {}
            }
            
            let value = if is_array {
                QueryStringParam::ArrayParam(vec![value.to_string()])
            } else {
                QueryStringParam::SimpleParam(value.to_string())
            };

            query_map.insert(key.to_string(),value);
            
        }

        Some(QueryString(query_map))
    }
}