enum ReqMethod {
    GET,
    POST,
    PATCH,
    PUT,
    DELETE,
}

impl ReqMethod {
    pub fn from_string(raw_method: &str) -> Option<ReqMethod> {
        let raw_method = raw_method.to_uppercase();

        if raw_method == "GET" {
            Some(ReqMethod::GET)
        } else if raw_method == "POST" {
            Some(ReqMethod::POST)
        } else if raw_method == "PATCH" {
            Some(ReqMethod::PATCH)
        } else if raw_method == "DELETE" {
            Some(ReqMethod::DELETE)
        } else if raw_method == "PUT" {
            Some(ReqMethod::PUT)
        } else {
            None
        }
    }
}

pub struct Req<'a> {
    body: Option<&'a str>,
    path: &'a str,
    method: ReqMethod,
}

impl Req<'_> {
    pub fn new(raw_req: String) {
        let mut lines = raw_req.lines();
        let req = Req {
            body: None,
            path: "",
            method: ReqMethod::GET,
        };

        let main_info = lines.next();
    }

    fn parse_main(line: &str) {}
}

