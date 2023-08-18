use once_cell::sync::Lazy;
use std::collections::HashMap;
use futures::Stream;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::WriteHalf;

#[derive(Debug)]
pub struct Res {
    pub outcome: String,
    status: usize,
    headers: HashMap<String, String>,
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

impl<'a> Res {
    pub fn new() -> Res {
        Res {
            outcome: String::new(),
            status: 200,
            headers: HashMap::new(),
        }
    }

    pub fn status(mut self, status: usize) -> Res {
        self.status = status;

        self
    }

    pub fn set(&'a mut self, header_key: &'a str, header_value: &'a str) -> &'a mut Res {
        self.headers.insert(header_key.parse().unwrap(), header_value.parse().unwrap());

        self
    }

    pub fn get(&self, header_key: &str) -> Option<&str> {
        let header_value = self.headers.get(header_key)?;


        Some(header_value)
    }

    fn status_title(&'a self) -> Option<&'static str> {
        let title = *STATUS_TITLES.get(&self.status)?;

        Some(title)
    }

    pub fn send(mut self, outcome: &str) -> Res {
        self.outcome.push_str(outcome);
        self
    }

    pub async fn send_outcome(mut self, mut stream: WriteHalf<'_>) {

        let formatted_headers = self.format_headers();
        let title = self.status_title().expect("Wrong status code");

        let raw_res = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status, title, formatted_headers, self.outcome

        );

        stream.write_all(raw_res.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }

    fn format_headers(&'a self) -> String {
        let mut formatted_headers = String::new();

        for (key, value) in &self.headers {
            formatted_headers += &format!("{key}: {value}\r\n");
        }

        formatted_headers
    }
}
