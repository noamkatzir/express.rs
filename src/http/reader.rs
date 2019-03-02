use std::io::prelude::*;
use std::net::TcpStream;
use bytes::{Bytes, BytesMut, BufMut};

static LOWER_CONTENT_LENGTH: &'static [u8] = b"CONTENT-LENGTH";
static UPPER_CONTENT_LENGTH: &'static [u8] = b"content-length";

pub struct HttpReader<'sock,'buf> {
    socket: &'sock mut TcpStream,
    buffer: &'buf mut [u8],
    pos: usize,
    read_size: usize,
    content_length: usize,
    has_body: bool,
    state: State
}

impl<'sock,'buf> HttpReader<'sock,'buf> {
    pub fn new(socket: &'sock mut TcpStream, buffer: &'buf mut [u8]) -> HttpReader<'sock,'buf> {
        let len = buffer.len();
        HttpReader {
            socket,
            buffer,
            pos: len,
            read_size: 0,
            content_length: 0,
            has_body: false,
            state: State::Method
        }
    }

    fn read_next_page(&mut self) -> Result<(),()> {
        if self.pos == self.buffer.len() {
            match self.socket.read(&mut self.buffer) {
                Ok(s) => {
                    self.read_size = s;
                    self.pos = 0;
                    Ok(())
                },
                _ => Err(())
            }
        } else {
            Ok(())
        }
    }

    fn read_method(&mut self) -> Option<Event> {
        let runner = &self.buffer[self.pos..self.read_size];

        for (index, value) in runner.iter().enumerate() {
            if value == &b' ' {
                let method = &self.buffer[self.pos..index];
                let res = match method {
                    b"GET" | b"POST" | b"PUT" | b"DELETE" => Some(Event::Method(Bytes::from(method))),
                    _ => Some(Event::Err(1))
                };
                self.state = State::Uri;
                self.pos += index+1;
                return res;
            }
        }

        None
    }

    fn read_uri(&mut self) -> Option<Event> {
        let runner = &self.buffer[self.pos..self.read_size];

        for (index, value) in runner.iter().enumerate() {
            if value == &b'?' {
                let res = Some(Event::Uri(Bytes::from(&self.buffer[self.pos..(self.pos+index)])));
                self.state = State::QueryStringParamName;
                self.pos += index+1;
                return res;
            }
            if value == &b' ' {
                let res = Some(Event::Uri(Bytes::from(&self.buffer[self.pos..(self.pos+index)])));
                self.state = State::Version;
                self.pos += index+1;
                return res;
            }
        }

        None
    }

    fn read_query_string(&mut self) -> Option<Event> {
        let runner = &self.buffer[self.pos..self.read_size];
        let mut middle_pos = self.pos;

        for (index, value) in runner.iter().enumerate() {
            match self.state {
                State::QueryStringParamName if value == &b'=' => {
                    middle_pos = self.pos + index;
                    self.state = State::QueryStringParamValue;
                },
                State::QueryStringParamName if value == &b' ' => {
                    let key = Bytes::from(&self.buffer[self.pos..(self.pos+index)]);
                    let value = Bytes::new();
                    let res = Some(Event::QueryStringParam(key,value));
                    self.state = State::Version;
                    self.pos += index+1;
                    return res;
                },
                State::QueryStringParamValue if value == &b'&' => {
                    let key = Bytes::from(&self.buffer[self.pos..middle_pos]);
                    let value = Bytes::from(&self.buffer[(middle_pos+1)..(self.pos+index)]);
                    let res = Some(Event::QueryStringParam(key,value));
                    self.state = State::QueryStringParamName;
                    self.pos += index+1;
                    return res;
                },
                State::QueryStringParamValue if value == &b' ' => {
                    let key = Bytes::from(&self.buffer[self.pos..middle_pos]);
                    let value = Bytes::from(&self.buffer[(middle_pos+1)..(self.pos+index)]);
                    let res = Some(Event::QueryStringParam(key,value));
                    self.state = State::Version;
                    self.pos += index+1;
                    return res;
                },
                _ => ()
            }
        }

        None
    }

    fn read_version(&mut self) -> Option<Event> {
        let runner = &self.buffer[self.pos..self.read_size];

        for (index, value) in runner.iter().enumerate() {
            if value == &b'\n' {
                let end_line_index = if runner[index-1] == b'\r' { index-1 } else { index };
                let res = Some(Event::Version(Bytes::from(&self.buffer[self.pos..(self.pos+end_line_index)])));
                self.state = State::HeaderName;
                self.pos += index+1;
                return res;
            }
        }

        None
    }

    fn read_header(&mut self) -> Option<Event> {
        let runner = &self.buffer[self.pos..self.read_size];
        let mut middle_pos = self.pos;
        let mut has_content_length: Option<bool> = None;

        for (index, value) in runner.iter().enumerate() {
            if has_content_length.is_none() && index >= UPPER_CONTENT_LENGTH.len() {
                has_content_length = Some(false);
            }
            if has_content_length.is_none() && (UPPER_CONTENT_LENGTH[index] != *value &&
                LOWER_CONTENT_LENGTH[index] != *value) {
                has_content_length = Some(false);
            }
            if has_content_length.is_none() && (index + 1) == UPPER_CONTENT_LENGTH.len() {
                has_content_length = Some(true);
            }

            match self.state {
                State::HeaderName if value == &b':' => {
                    middle_pos = self.pos + index;
                    self.state = State::HeaderValue;
                },
                State::HeaderName if value == &b'\n' => {
                    if runner[0] == b'\r' || index == 0 {
                        self.state = State::Body;
                        self.pos += index+1;
                        return Some(Event::EndOfHeaders);
                    } else { //this is fatal parsing error, need to think about it
                        let res = Some(Event::Err(2));
                        self.pos += index+1;
                        return res;
                    }
                },
                State::HeaderValue if value == &b'\n' => {
                    let end_line_index = if runner[index-1] == b'\r' { index-1 } else { index };
                    let res = if let Some(true) = has_content_length {
                        self.has_body = true;
                        self.content_length = byte_array_to_i32(&self.buffer[(middle_pos+1)..(self.pos+end_line_index)]);
                        Some(Event::ContentLength(self.content_length))
                    } else {
                        let key = Bytes::from(&self.buffer[self.pos..middle_pos]);
                        let value = Bytes::from(&self.buffer[(middle_pos+1)..(self.pos+end_line_index)]);
                        Some(Event::Header(key,value))
                    };

                    self.state = State::HeaderName;
                    self.pos += index+1;
                    return res;
                },
                _ => ()
            }
        }

        None
    }

    fn read_body(&mut self) -> Option<Event> {
        if self.content_length > 0 {
            if self.content_length <= (self.read_size - self.pos + 1) {
                self.state = State::End;
                return Some(Event::Body(Bytes::from(&self.buffer[self.pos..self.read_size])));
            } else {
                let mut left_to_read = self.content_length - (self.read_size - self.pos + 1);
                let mut res = BytesMut::with_capacity(self.content_length);
                res.put(&self.buffer[self.pos..self.read_size]);
                loop {
                    if let Err(_) = self.read_next_page() {
                        return Some(Event::Err(3));
                    }
                    res.put(&self.buffer[0..self.read_size]);
                    left_to_read -= self.read_size;
                    if left_to_read <= 0 {
                        self.state = State::End;
                        return Some(Event::Body(res.freeze()));
                    }
                    if self.read_size == 0 {
                        return Some(Event::Err(4));
                    }

                }
                
            }
        }

        None
    }
}

pub enum Event {
    Header(Bytes,Bytes),
    Method(Bytes),
    Uri(Bytes),
    QueryStringParam(Bytes,Bytes),
    Version(Bytes),
    // Chunk(usize,Bytes),
    ContentLength(usize),
    Body(Bytes),
    EndOfHeaders,
    Err(u8)
}

enum State {
    Method,
    Uri,
    QueryStringParamName,
    QueryStringParamValue,
    Version,
    HeaderName,
    HeaderValue,
    Body,
    End
}

impl<'sock,'buf> Iterator for HttpReader<'sock,'buf> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        if self.read_next_page().is_err() {
            return Some(Event::Err(5));
        }

        match self.state {
            State::Method => self.read_method(),
            State::Uri => self.read_uri(),
            State::QueryStringParamName => self.read_query_string(),
            State::Version => self.read_version(),
            State::HeaderName => self.read_header(),
            State::Body => self.read_body(),
            State::End => None,
            _ => Some(Event::Err(6))
        }
    }
}

fn byte_array_to_i32(buffer: &[u8]) -> usize {
    buffer.iter()
        .filter(|x| **x>= b'0' && **x <= b'9').map(|x| (*x - b'0') as usize)
        .fold(0, |acc, x| acc*10 + x)
}