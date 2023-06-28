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

// NOTE: Reference:
// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Status>
static STATUS_TITLES: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
    HashMap::from([
        // Information responses
        (100, "Continue"),
        (101, "Switching Protocol"),
        (102, "Processing"),
        (103, "Early Hints"), // WARN: Experimental
        // Successful responses
        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (203, "Non-Authoritative Information"),
        (204, "No Content"),
        (205, "Reset Content"),
        (206, "Partial Content"),
        (207, "Multi-Status Status"), // NOTE: WebDAV
        (208, "Already Reported"),
        (226, "IM Used"),
        // Redirection messages
        (300, "Multiple Choices"),
        (301, "Moved Permanently"),
        (302, "Found"),
        (303, "See Other"),
        (304, "Not Modified"),
        (305, "Use Proxy"), // WARN: Deprecated
        (306, "unused"),
        (307, "Temporary Redirect"),
        (308, "Permanent Redirect"),
        // Client error responses
        (400, "Bad Request"),
        (401, "Unauthorized"),
        (402, "Payment Required"), // NOTE: Experimental
        (403, "Forbidden"),
        (404, "Not Found"),
        (405, "Method Not Allowed"),
        (406, "Not Acceptable"),
        (407, "Proxy Authentication Required"),
        (408, "Request Timeout"),
        (409, "Conflict"),
        (410, "Gone"),
        (411, "Length Required"),
        (412, "Precondition Failed"),
        (413, "Payload Too Large"),
        (414, "URI Too Long"),
        (415, "Unsupported Media Type"),
        (416, "Range Not Satisfiable"),
        (417, "Expectation Failed"),
        (418, "I'm a teapot"),
        (421, "Misdirected Request"),
        (422, "Unprocessable Content"), // NOTE: WebDAV
        (423, "Locked"),                // NOTE: WebDAV
        (424, "Failed Dependency"),     // NOTE: WebDAV
        (425, "Too Early"),             // NOTE: Experimental
        (426, "Upgrade Required"),
        (428, "Precondition Required"),
        (429, "Too Many Requests"),
        (431, "Request Header Fields Too Large"),
        (451, "Unavailable For Legal Reasons"),
        // Server error responses
        (500, "Internal Server Error"),
        (501, "Not Implemented"),
        (502, "Bad Gateway"),
        (503, "Service Unavailable"),
        (504, "Gateway Timeout"),
        (505, "HTTP Version Not Supported"),
        (506, "Variant Also Negotiates"),
        (507, "Insufficient Storage"), // NOTE: WebDAV
        (508, "Loop Detected"),        // NOTE: WebDAV
        (510, "Not Extended"),
        (511, "Network Authentication Required"),
    ])
});

impl<'a> Res<'a> {
    pub fn new(stream: WriteHalf<'a>) -> Res<'a> {
        Res {
            status: 200,
            headers: HashMap::new(),
            stream,
        }
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

        let raw_res = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status, title, formatted_headers, body
        );

        self.stream.write_all(raw_res.as_bytes()).await.unwrap();
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
