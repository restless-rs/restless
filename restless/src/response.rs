use std::collections::HashMap;
use tokio::io::{AsyncWriteExt};
use tokio::net::tcp::WriteHalf;
use tokio::net::TcpStream;
use crate::app::App;

pub struct Res<'a> {
    stream: WriteHalf<'a>,
    status: usize,
    headers: HashMap<&'a str, &'a str>
}

impl<'a> Res<'a> {
    pub fn new(stream: WriteHalf<'a>) -> Res<'a> {
        let mut res = Res {
            status: 200,
            headers: HashMap::new(),
            stream
        };

        res
    }

    pub fn status(&'a mut self, status: usize) -> &mut Res {
        self.status = status;

        self
    }

    pub fn set(&'a mut self, header_key: &'a str, header_value: &'a str) -> &'a mut Res {
        self.headers.insert(header_key, header_value);

        self
    }

    pub fn get(&self, header_key: &str) -> Option<&str> {
        let header_value = *self.headers.get(header_key)?;

        Some(header_value)
    }

    pub async fn send(&mut self, body: &str) {
        let mut raw_res = String::from("HTTP/1.1");
        raw_res += &*format!(" {} ", self.status);
        raw_res += "\r\n\r\n";

        raw_res += body;


        let _ = self.stream.write_all(raw_res.as_bytes()).await.unwrap();
        self.stream.flush().await.unwrap();
    }
}