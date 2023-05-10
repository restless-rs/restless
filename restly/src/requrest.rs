use tokio::io;

#[derive(Default)]
pub struct Req<'a> {
    body: Option<&'a str>,
    path: &'a str,
    method: &'a str,
}

impl<'a> Req<'a> {
    pub fn new(raw_req: String) -> Result<Req<'a>, io::Error> {
        let mut lines = raw_req.lines();
        let req = Req::default();

        let main_info = lines.next();

        Ok(req)
    }

    fn parse_main(line: &str) {}
}

