use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::WriteHalf;

#[derive(Debug)]
pub struct Res<'a> {
    stream: WriteHalf<'a>,
    status: usize,
    headers: HashMap<&'a str, &'a str>,
}

static STATUS_TITLES: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
    HashMap::from([
        (100, "Continue"),
        (101, "Switching"),
        (102, "Processing"),
        (103, "Early"),
        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (203, "Non-Authoritative"),
        (200, "OK"),
        (204, "No"),
        (205, "Reset"),
        (206, "Partial"),
        (207, "Multi-Status"),
        (208, "Already"),
        (226, "IM"),
        (300, "Multiple"),
        (301, "Moved"),
        (302, "Found"),
        (303, "See"),
        (304, "Not"),
        (305, "Use"),
        (306, "unused"),
        (307, "Temporary"),
        (302, "Found"),
        (308, "Permanent"),
        (301, "Moved"),
        (400, "Bad"),
        (401, "Unauthorized"),
        (402, "Payment"),
        (403, "Forbidden"),
        (401, "Unauthorized"),
        (404, "Not"),
        (403, "Forbidden"),
        (405, "Method"),
        (406, "Not"),
        (407, "Proxy"),
        (401, "Unauthorized"),
        (408, "Request"),
        (409, "Conflict"),
        (410, "Gone"),
        (411, "Length"),
        (412, "Precondition"),
        (413, "Payload"),
        (414, "URI"),
        (415, "Unsupported"),
        (416, "Range"),
        (417, "Expectation"),
        (418, "I'm"),
        (421, "Misdirected"),
        (422, "Unprocessable"),
        (423, "Locked"),
        (424, "Failed"),
        (425, "Too"),
        (426, "Upgrade"),
        (428, "Precondition"),
        (429, "Too"),
        (431, "Request"),
        (451, "Unavailable"),
        (500, "Internal"),
        (501, "Not"),
        (502, "Bad"),
        (503, "Service"),
        (504, "Gateway"),
        (505, "HTTP"),
        (506, "Variant"),
        (507, "Insufficient"),
        (508, "Loop"),
        (510, "Not"),
        (511, "Network"),
    ])
});

impl<'a> Res<'a> {
    pub fn new(stream: WriteHalf<'a>) -> Res<'a> {
        let res = Res {
            status: 200,
            headers: HashMap::new(),
            stream,
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

    fn status_title(&self) -> Option<&'static str> {
        let title = *STATUS_TITLES.get(&self.status)?;

        Some(title)
    }

    pub async fn send(&mut self, body: &str) {
        let formatted_headers = self.format_headers();
        let title = self.status_title().expect("Wrong status code");

        let raw_res = String::from(format!(
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status, title, formatted_headers, body
        ));

        let _ = self.stream.write_all(raw_res.as_bytes()).await.unwrap();
        self.stream.flush().await.unwrap();
    }

    fn format_headers(&self) -> String {
        let mut formatted_headers = String::new();

        for (key, value) in &self.headers {
            formatted_headers += &format!("{key}: {value}\r\n");
        }

        formatted_headers
    }
}
