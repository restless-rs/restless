use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Req {
    pub body: Option<String>,
    pub path: String,
    pub method: ReqMethod,
    pub queries: HashMap<String, String>,
    protocol: String,
    headers: HashMap<String, String>,
}

#[derive(Default, Debug)]
pub enum ReqMethod {
    #[default]
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

impl Display for ReqMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReqMethod::Get => write!(f, "GET"),
            ReqMethod::Post => write!(f, "POST"),
            ReqMethod::Patch => write!(f, "PATCH"),
            ReqMethod::Put => write!(f, "PUT"),
            ReqMethod::Delete => write!(f, "DELETE"),
        }
    }
}

impl FromStr for ReqMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(ReqMethod::Get),
            "POST" => Ok(ReqMethod::Post),
            "PATCH" => Ok(ReqMethod::Patch),
            "PUT" => Ok(ReqMethod::Put),
            "DELETE" => Ok(ReqMethod::Delete),
            _ => Err(()),
        }
    }
}

impl Req {
    pub fn new(raw_req: &str) -> Option<Req> {
        let mut lines = raw_req.lines();
        let mut req = Req::default();

        let main_info = match lines.next() {
            Some(v) => v,
            None => {
                return None;
            }
        };

        let (req_method, path, protocol) = Req::parse_first_line(main_info).expect("Wrong format");

        req.method = req_method;
        req.path = path.parse().unwrap();
        req.protocol = protocol.parse().unwrap();

        // Pulling headers
        lines.clone().take_while(|l| !l.is_empty()).for_each(|l| {
            let mut split = l.split(": ");
            let header_name = split.next().unwrap();
            let header_value = split.next().unwrap();

            req.headers
                .insert(header_name.parse().unwrap(), header_value.parse().unwrap());
        });

        req.derive_queries();

        // Pulling body
        let body = lines
            .clone()
            .skip_while(|l| !l.is_empty())
            .fold(String::new(), |mut acc, l| {
                acc += l;
                acc
            });

        if !body.is_empty() {
            req.body = Some(body);
        };

        Some(req)
    }

    pub fn get(&self, header_key: &str) -> Option<&str> {
        let header_value = self.headers.get(header_key)?;

        Some(header_value)
    }

    fn parse_first_line(line: &str) -> Result<(ReqMethod, &str, &str), ()> {
        let mut splitted = line.split_whitespace();

        let method = splitted.next().ok_or(())?;
        let path = splitted.next().ok_or(())?;
        let protocol = splitted.next().ok_or(())?;

        let req_method = ReqMethod::from_str(method)?;

        Ok((req_method, path, protocol))
    }

    fn derive_queries(&mut self) {
        if !self.path.contains('?') {
            return;
        }

        let splitted_path = self.path.split('?').collect::<Vec<_>>();
        let query_string = splitted_path.get(1).unwrap();

        self.queries = Req::parse_query_string(query_string);

        let pure_path = splitted_path.first().unwrap();
        self.path = pure_path.parse().unwrap();
    }

    fn parse_query_string(query_string: &str) -> HashMap<String, String> {
        let mut res = HashMap::new();

        let pairs: Vec<_> = query_string.split('&').collect();
        pairs.into_iter().for_each(|p| {
            let mut splitted = p.split('=');
            let name = splitted.next().unwrap().to_owned();
            let value = splitted.next().unwrap().to_owned();

            res.insert(name, value);
        });

        res
    }
}
