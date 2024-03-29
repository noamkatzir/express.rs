use bytes::{Bytes, BufMut, BytesMut};
use super::status::Status;

#[derive(Debug)]
pub struct Response {
    protocol: Bytes,
    status: Status,
    headers: Vec<(Bytes,Bytes)>,
    body: Bytes
}

impl Response {
    pub fn new() -> Self {
        Response {
            protocol: Bytes::new(),
            status: Status::OK,
            headers: vec![],
            body: Bytes::new()
        }
    }

    pub fn add_json_headers(&mut self) -> &mut Self {
        self.add_header_str(b"Server",b"Noam's")
            .add_header_str(b"Content-Type",b"application/json; charset=utf-8")
            .add_header_str(b"Access-Control-Allow-Origin",b"*")
            .add_header_str(b"Cache-Control",b"no-cache")
            .add_header_str(b"Status",b"200 OK");

        self.set_protocol(Bytes::from_static(b"HTTP/1.1"));

        self
    }

    pub fn add_html_headers(&mut self) -> &mut Self {
        self.add_header_str(b"Server",b"Noam's") 
            .add_header_str(b"Content-Type",b"text/html; charset=utf-8")
            .add_header_str(b"Access-Control-Allow-Origin",b"*")
            .add_header_str(b"Cache-Control",b"no-cache")
            .add_header_str(b"Status",b"200 OK");

        self.set_protocol(Bytes::from_static(b"HTTP/1.1"));

        self
    }

    pub fn add_gzip_headers(&mut self) -> &mut Self {
        self.add_header_str(b"Content-Encoding",b"deflate");

        self
    }

    pub fn set_protocol(&mut self, protocol: Bytes) -> &mut Self {
        self.protocol = protocol;
        self
    }

    pub fn set_status(&mut self, status: Status) -> &mut Self {
        self.status = status;
        self
    }


    pub fn add_header(&mut self, name: Bytes, value: Bytes) -> &mut Self {
        self.headers.push((name,value));
        self
    }

    fn add_header_str(&mut self, name: &'static[u8], value: &'static[u8]) -> &mut Self {
        self.add_header(Bytes::from_static(name), Bytes::from_static(value));
        self
    }

    pub fn add_full_body(&mut self, body: Bytes) -> &mut Self {
        self.add_header(Bytes::from_static(b"Content-Length"),Bytes::from(format!("{}",body.len())));
        self.body = body;
        self
    }

    pub fn generate(&self) -> Bytes {
        let space_len = " ".len();
        let end_of_line_len = "\n\r".len();
        let header_seperator_len = ": ".len();
        let mut count = self.protocol.len() 
                        + space_len
                        + self.status.code_bytes_len() 
                        + space_len
                        + self.status.to_message().len() 
                        + end_of_line_len;
        count += self.headers.iter().fold(0, |acc, (key,value)| {
            acc + key.len()+value.len()+ header_seperator_len + end_of_line_len
        });

        count += end_of_line_len + self.body.len();
        let mut result = BytesMut::with_capacity(count);
        result.put(self.protocol.clone());
        result.put(b' ');
        result.put(&format!("{}",self.status.to_code())[..]);
        result.put(b' ');
        result.put(self.status.to_message());
        result.put(&b"\r\n"[..]);
        self.headers.iter().for_each(|(key,value)| {
            result.put(key.clone());
            result.put(&b": "[..]);
            result.put(value.clone());
            result.put(&b"\r\n"[..]);
        });
        result.put(&b"\r\n"[..]);
        result.put(self.body.clone());


        result.freeze()
    }


}
