use std::io::BufReader;
use std::net::TcpStream;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Clone)]
pub enum MethodKind {
    Get,
    Post,
    Put,
    Delete
}
#[derive(Clone)]
pub enum HttpVersion {
    V1,
    V2
}
#[derive(Clone)]
pub enum Connection {
    KeepAlive,
    Close
}
#[derive(Clone)]
pub enum CacheControl {
    MaxAge(i32),
    NoCache
}
#[derive(Clone)]
pub enum HttpHeader {
    Host {name: String, port: i32 },
    UserAgent(String),
    Referer(String),
    Accept(Vec<(String,f32)>),
    AcceptLanguage(Vec<(String,f32)>),
    AcceptEncoding(Vec<String>),
    Connection(Connection),
    UpgradeInsecureRequests,
    CacheControl(CacheControl),
    Pragma(String)
}

pub struct Request {
    method: MethodKind,
    uri: String,
    query: QueryString,
    protocol: HttpVersion,
    headers: HashMap<String,String>
}

impl Request {
    fn parse_method(reader: &mut BufReader<&mut TcpStream>) -> Option<(MethodKind,String,HttpVersion)> {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Err(_) | Ok(0) => None,
            Ok(_n) => {
                match parse_method(&mut buf) {
                    Err(_) => None,
                    Ok(tup) => Some(tup)
                }
            }
        }
    }

    fn parse_headers(reader: &mut BufReader<&mut TcpStream>) -> Option<HashMap<String,String>> {
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

        Some(headers)
    }

    fn get_uri_and_query_string(url: &str) -> Option<(String,QueryString)> {
        let split_query_string = url.split("?").collect::<Vec<&str>>();
        let uri = split_query_string[0];
        let query_string = split_query_string[1];

        match parse_query_string(query_string) {
            Some(query) => Some((uri.to_string(),query)),
            None => None
        }
    }
    pub fn parse(stream: &mut TcpStream) -> Option<Request> {
        let mut reader = BufReader::new(stream);
        let (method,url,protocol) = match Request::parse_method(&mut reader) {
            Some(res) => res,
            _ => return None
        };

        let headers = match Request::parse_headers(&mut reader) {
            Some(res) => res,
            _ => return None
        };

        match Request::get_uri_and_query_string(&url) {
            Some((uri, query)) => Some(Request {method, uri, query, protocol, headers}),
            _ => None
        }
    }

    pub fn get_parsed_header(&self, name: &str) -> Option<HttpHeader> {
        match self.headers.get(name) {
            Some(value) => Some(parse_header(name,&value)),
            _ => None
        }
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        match self.headers.get(name) {
            Some(value) => Some(value),
            _ => None
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn query(&self, key: &str) -> Option<&QueryStringParam> {
        self.query.get(key)
    }
}

// pub struct HttpParser<'a> {
//     underlying: BufReader<&'a TcpStream>
// }
// impl<'a> HttpParser<'a> {
//     pub fn new (reader: BufReader<&'a TcpStream>) -> Self {
//         HttpParser {
//             underlying: reader
//         }
//     }
// }


// impl<'a> Iterator for HttpParser<'a> {
//     type Item = HttpHeader;

//     fn next(&mut self) -> Option<HttpHeader> {
//         let mut buf = String::new();
//         match self.underlying.read_line(&mut buf) {
//             Err(_) | Ok(0) => None,
//             Ok(_n) => {
//                 if buf.ends_with("\n") {
//                     buf.pop();
//                     if buf.ends_with("\r") {
//                         buf.pop();
//                     }
//                 }
//                 Some(parse_header(&buf))
//             }
//         }
//     }
// }

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

fn parse_method(buf: &String) -> Result<(MethodKind,String,HttpVersion),()> {
    let method = buf.split_whitespace().take(3).collect::<Vec<&str>>();
    if let [method, uri, proto] = &method[..] {
        let protocol = match &**proto {
            "HTTP/1.1" => HttpVersion::V1,
            _ => panic!("unknoun HTTP version: {}",proto)
        };
        let name = match &**method {
            "GET" => MethodKind::Get,
            "POST" => MethodKind::Post,
            "PUT" => MethodKind::Put,
            "DELETE" => MethodKind::Delete,
            _ => panic!("unknoun HTTP Method")
        };
        Ok((name, uri.to_string(), protocol))
    } else {
        Err(())
    }
}

fn parse_header(name: &str,value: &String) -> HttpHeader {
    let value = &value[..];
    match name {
        "Host" => {
            let host: Vec<&str> = value.split(":").collect();
            HttpHeader::Host{name: host[0].to_string(), port: host[1].parse().unwrap() }
        },
        "User-Agent" => HttpHeader::UserAgent(value.to_string()),
        "Accept" => HttpHeader::Accept(parse_accept(value)),
        "Referer" => HttpHeader::Referer(value.to_string()),
        "Pragma" => HttpHeader::Pragma(value.to_string()),
        "Accept-Language" => HttpHeader::AcceptLanguage(parse_accept(value)),
        "Accept-Encoding" => HttpHeader::AcceptEncoding(value.split(",").map(|x| x.trim().to_string()).collect()),
        "Connection" =>  {
            match value {
                "keep-alive" => HttpHeader::Connection(Connection::KeepAlive),
                _ => HttpHeader::Connection(Connection::Close),
            }
        },
        "Upgrade-Insecure-Requests" =>  HttpHeader::UpgradeInsecureRequests,
        "Cache-Control" => {
            let cache_control: Vec<&str> = value.split("=").collect();
            println!("cache control value: {}", value);
                
            match cache_control.len() {
                2 => HttpHeader::CacheControl(CacheControl::MaxAge(cache_control[1].parse().unwrap())),
                _ => HttpHeader::CacheControl(CacheControl::MaxAge(0))
            }
        },
        _ => panic!("unknoun HTTP header {} => {}",name,value)
    }
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
}

fn parse_query_string(query_string: &str) -> Option<QueryString> {
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