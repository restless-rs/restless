use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::app::App;

#[derive(Default)]
pub struct Response<'a> {
    stream: &'a mut TcpStream,
    status: usize,
    headers: HashMap<&'a str, &'a str>
}

impl Response {
    pub fn new(&mut stream: TcpStream) -> Response {
        let mut res = Response::default();
        res.stream = stream;

        res
    }

    pub fn status(&mut self, status: usize) -> &mut Response {
        self.status = status;

        self
    }

    pub fn set(&mut self, header_key: &str, header_value: &str) -> &mut Response {
        self.headers.insert(header_key, header_value);

        self
    }

    pub fn get(&self, header_key: &str) -> Option<&str> {
        let header_value = *self.headers.get(header_key)?;

        Some(header_value)
    }

    pub fn send(&self, body: &str) {
        let _ = self.stream.write_all(body.as_ref());
    }
}