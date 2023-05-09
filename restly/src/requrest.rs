enum ReqMethod {
    GET,
    POST,
    PATCH,
    PUT,
    DELETE,
}

impl ReqMethod {
    pub fn from_string(method_str: &str) -> ReqMethod {
        let method_uppercased = method_str.to_uppercase();
        if method_uppercased == "GET" {
            ReqMethod::GET
        } else if method_uppercased == "POST" {
            ReqMethod::POST
        } else if method_uppercased == "PATCH" {
            ReqMethod::PATCH
        } else if method_uppercased == "DELETE" {
            ReqMethod::DELETE
        } else if method_uppercased == "PUT" {
            ReqMethod::PUT
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
        let req = Req{
            body: None,
            path: "",
            method: ReqMethod::GET,
        };

        let main_info = lines.next();
    }

    fn parse_main(line: &str) {

    }
}