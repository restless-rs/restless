use futures::executor::block_on;
use std::net::SocketAddr;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

use once_cell::sync::Lazy;
use tokio::net::tcp::ReadHalf;

use crate::request::Req;
use crate::response::Res;
use crate::route::{PathItemType, Route, RouteCallback};
use crate::route_handler::RouteHandler;

const BASE_ADDR: &str = "127.0.0.1";

pub struct App<'a> {
    pub routes: Vec<Route<'a>>,
}

static mut APP: Lazy<App<'static>> = Lazy::new(|| App { routes: vec![] });

impl App<'static> {
    pub fn new() -> &'static mut Lazy<App<'static>> {
        unsafe { &mut APP }
    }

    // TODO: Client error handle hook on connection
    #[tokio::main]
    pub async fn listen<F>(&'static self, port: u16, on_bound: F)
    where
        F: FnOnce(),
    {
        // TODO: Create `build_addr` function
        let addr = format!("{}:{}", BASE_ADDR.to_owned(), port);

        let listener = TcpListener::bind(addr.clone())
            .await
            .unwrap_or_else(|_| panic!("Can't bound at {}", addr));

        on_bound();

        loop {
            let result = listener.accept().await;

            tokio::spawn(async move {
                match result {
                    Ok((stream, addr)) => self.handle_stream(stream, addr).await,
                    Err(err) => println!("Couldn't get client: {:?}", err),
                }
            });
        }
    }

    async fn handle_stream<'a>(&'static self, mut socket: TcpStream, _addr: SocketAddr) {
        let (mut read_half, write_half) = socket.split();

        let raw_req = self.read_all(&mut read_half).await.unwrap();
        let mut req = Req::new(&raw_req);
        let mut res = Res::new();

        let route = self.get_route(&req).unwrap();

        let mut out = (route.callback)(req, res);

        Res::send_outcome(out, write_half).await;
    }

    async fn read_all<'a>(&self, read_half: &mut ReadHalf<'a>) -> Result<String, std::io::Error> {
        // https://stackoverflow.com/a/71949195
        let mut buf: Vec<u8> = Vec::new();

        // Solve would block problems
        let mut firs_read_buf = [0u8; 2024];
        let bytes_read = read_half.read(&mut firs_read_buf).await.unwrap();
        buf.extend_from_slice(&firs_read_buf[..bytes_read]);

        loop {
            // Creating the buffer **after** the `await` prevents it from
            // being stored in the async task.
            let mut tmp_buf = [0u8; 1024];

            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match read_half.try_read(&mut tmp_buf) {
                Ok(0) => break,
                Ok(bytes_read) => buf.extend_from_slice(&tmp_buf[..bytes_read]),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(std::str::from_utf8(&buf)
            .unwrap()
            .trim_matches(char::from(0))
            .to_owned());
    }

    #[allow(dead_code)]
    fn get_route<'a, 'b>(&'static self, req: &'a Req) -> Option<&Route<'b>> {
        let mut res_route= None;
        let req_paths = req.path.split_terminator('/').collect::<Vec<_>>();

        for route in &self.routes {
            let mut is_compatible = true;

            if route.paths.len() != req_paths.len() {
                continue;
            }

            for (i, path) in route.paths.iter().enumerate().take(route.paths.len()) {
                match path.r#type {
                    PathItemType::Static => {
                        if path.value != req_paths[i] {
                            is_compatible = false;
                            break;
                        }
                    }
                    PathItemType::Dynamic => (),
                }
            }

            if is_compatible {
                res_route = Some(route);
                break;
            }
        }
        res_route
    }
}

impl RouteHandler for App<'_> {
    fn get(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes
            .push(Route::new(path, handler, Some("GET"), false));
        self
    }

    fn post(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes
            .push(Route::new(path, handler, Some("POST"), false));
        self
    }

    fn put(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes
            .push(Route::new(path, handler, Some("PUT"), false));
        self
    }

    fn delete(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes
            .push(Route::new(path, handler, Some("DELETE"), false));
        self
    }

    fn patch(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes
            .push(Route::new(path, handler, Some("PATCH"), false));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_empty() {
        let temp_app = App::new();

        temp_app.routes.push(Route::new(
            "/home",
            |_, _| println!("home"),
            Some("GET"),
            false,
        ));
        temp_app.routes.push(Route::new(
            "/login",
            |_, _| println!("first login"),
            Some("GET"),
            false,
        ));
        temp_app.routes.push(Route::new(
            "/login",
            |_, _| println!("second logout"),
            Some("GET"),
            false,
        ));

        let mock_req = r#"GET / HTTP/1.1
Host: localhost:3000
Connection: keep-alive
sec-ch-ua: "Google Chrome";v="113", "Chromium";v="113", "Not-A.Brand";v="24"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36
Sec-Purpose: prefetch;prerender
Purpose: prefetch
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: en,en-US;q=0.9,ru-RU;q=0.8,ru;q=0.7
Cookie: _ga=GA1.1.132133627.1663565819; a_session_console_legacy=eyJpZCI6IjYzMjgwMDFjNDNiNGEyZDVkODRlIiwic2VjcmV0IjoiMWU4Y2Q4NmIwYmQ5YmE3MzI5ZWY5MjJjOTMwZTBjM2VmZWZiMjM4NzQyYzBlYzE3MmIxODQ4NTQ0ZGY1MGM1ZGE0YjBlYzQzOTIwY2Y3Yzc3Mjg3OWY5MWQ1OTZlNzAwZTdhOWY3NjNkZWI4YjRiYzIwYzJmMDkwZTU3M2EzMzgzZGFlM2M5NjNhYTM1NGU0NmU0ZjgxZjcwYmE2MjI3MWEyMTM5NGYyZmQ0ZDNmNGY3MzJlOWQyMWUyOTI2Yzk3ZWVjZjAwMWJlMDM4NGZhMjA5YTljNGQ4ZDU1YmFkMWMxZTI0MWNiZGQxYTBmMzBlMjkxNDM5NmYzNTQ5YWU4OCJ9; cookie-alert=true; _ym_uid=1674046531634537132; _ym_d=1674046531; csrftoken=icRfgtexnng3ZjsuACqA0zCjdvIEW3t6; Webstorm-349731ad=ca1a0e66-d6d8-446d-a5b3-911737bf1d3e; _ga_EFC8B2CDNB=GS1.1.1679717912.2.0.1679718014.0.0.0; username-localhost-8888="2|1:0|10:1682664751|23:username-localhost-8888|44:ODJmNGUwNTEzOGE1NGYwMDgwOGZiMGFlYmUzZGI5N2Y=|07cf7de31aa2f8fd435b46d9fb76ffffb6f3412857496fafe4c68f426f46ea91"; username-localhost-8889="2|1:0|10:1683263705|23:username-localhost-8889|44:Y2I5MmUzYTA4MWY1NGVlMjg2OWE2ODE5YjZmYzE0NmQ=|b74a99392dc758c4b95264fe1602ceaf0161e6ac5291eeafd830d10cb096b7bb"; Webstorm-3497356c=5cdf5472-3f5b-4526-b4c7-4c705ce4d8e6"#;

        let req = Req::new(mock_req);

        let routes = temp_app.get_route(&req);

        assert_eq!(routes.len(), 0);
    }

    #[test]
    fn test_build_req_path() {
        let temp_app = App::new();

        temp_app.routes.push(Route::new(
            "/home",
            |_, _| println!("home"),
            Some("GET"),
            false,
        ));
        temp_app.routes.push(Route::new(
            "/login",
            |_, _| println!("first login"),
            Some("GET"),
            false,
        ));
        temp_app.routes.push(Route::new(
            "/login",
            |_, _| println!("second logout"),
            Some("GET"),
            false,
        ));

        let mock_req = r#"GET /login HTTP/1.1
Host: localhost:3000
Connection: keep-alive
sec-ch-ua: "Google Chrome";v="113", "Chromium";v="113", "Not-A.Brand";v="24"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36
Sec-Purpose: prefetch;prerender
Purpose: prefetch
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: en,en-US;q=0.9,ru-RU;q=0.8,ru;q=0.7
Cookie: _ga=GA1.1.132133627.1663565819; a_session_console_legacy=eyJpZCI6IjYzMjgwMDFjNDNiNGEyZDVkODRlIiwic2VjcmV0IjoiMWU4Y2Q4NmIwYmQ5YmE3MzI5ZWY5MjJjOTMwZTBjM2VmZWZiMjM4NzQyYzBlYzE3MmIxODQ4NTQ0ZGY1MGM1ZGE0YjBlYzQzOTIwY2Y3Yzc3Mjg3OWY5MWQ1OTZlNzAwZTdhOWY3NjNkZWI4YjRiYzIwYzJmMDkwZTU3M2EzMzgzZGFlM2M5NjNhYTM1NGU0NmU0ZjgxZjcwYmE2MjI3MWEyMTM5NGYyZmQ0ZDNmNGY3MzJlOWQyMWUyOTI2Yzk3ZWVjZjAwMWJlMDM4NGZhMjA5YTljNGQ4ZDU1YmFkMWMxZTI0MWNiZGQxYTBmMzBlMjkxNDM5NmYzNTQ5YWU4OCJ9; cookie-alert=true; _ym_uid=1674046531634537132; _ym_d=1674046531; csrftoken=icRfgtexnng3ZjsuACqA0zCjdvIEW3t6; Webstorm-349731ad=ca1a0e66-d6d8-446d-a5b3-911737bf1d3e; _ga_EFC8B2CDNB=GS1.1.1679717912.2.0.1679718014.0.0.0; username-localhost-8888="2|1:0|10:1682664751|23:username-localhost-8888|44:ODJmNGUwNTEzOGE1NGYwMDgwOGZiMGFlYmUzZGI5N2Y=|07cf7de31aa2f8fd435b46d9fb76ffffb6f3412857496fafe4c68f426f46ea91"; username-localhost-8889="2|1:0|10:1683263705|23:username-localhost-8889|44:Y2I5MmUzYTA4MWY1NGVlMjg2OWE2ODE5YjZmYzE0NmQ=|b74a99392dc758c4b95264fe1602ceaf0161e6ac5291eeafd830d10cb096b7bb"; Webstorm-3497356c=5cdf5472-3f5b-4526-b4c7-4c705ce4d8e6"#;

        let req = Req::new(mock_req);

        let routes = temp_app.get_route(&req);

        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn test_build_with_dynamic() {
        let temp_app = App::new();

        temp_app.routes.push(Route::new(
            "/home",
            |_, _| println!("home"),
            Some("GET"),
            false,
        ));
        temp_app.routes.push(Route::new(
            "/:user_id/login",
            |_, _| println!("first login"),
            Some("GET"),
            false,
        ));
        temp_app.routes.push(Route::new(
            "/login",
            |_, _| println!("second logout"),
            Some("GET"),
            false,
        ));

        let mock_req = r#"GET /234sdf/login HTTP/1.1
Host: localhost:3000
Connection: keep-alive
sec-ch-ua: "Google Chrome";v="113", "Chromium";v="113", "Not-A.Brand";v="24"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36
Sec-Purpose: prefetch;prerender
Purpose: prefetch
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: en,en-US;q=0.9,ru-RU;q=0.8,ru;q=0.7
Cookie: _ga=GA1.1.132133627.1663565819; a_session_console_legacy=eyJpZCI6IjYzMjgwMDFjNDNiNGEyZDVkODRlIiwic2VjcmV0IjoiMWU4Y2Q4NmIwYmQ5YmE3MzI5ZWY5MjJjOTMwZTBjM2VmZWZiMjM4NzQyYzBlYzE3MmIxODQ4NTQ0ZGY1MGM1ZGE0YjBlYzQzOTIwY2Y3Yzc3Mjg3OWY5MWQ1OTZlNzAwZTdhOWY3NjNkZWI4YjRiYzIwYzJmMDkwZTU3M2EzMzgzZGFlM2M5NjNhYTM1NGU0NmU0ZjgxZjcwYmE2MjI3MWEyMTM5NGYyZmQ0ZDNmNGY3MzJlOWQyMWUyOTI2Yzk3ZWVjZjAwMWJlMDM4NGZhMjA5YTljNGQ4ZDU1YmFkMWMxZTI0MWNiZGQxYTBmMzBlMjkxNDM5NmYzNTQ5YWU4OCJ9; cookie-alert=true; _ym_uid=1674046531634537132; _ym_d=1674046531; csrftoken=icRfgtexnng3ZjsuACqA0zCjdvIEW3t6; Webstorm-349731ad=ca1a0e66-d6d8-446d-a5b3-911737bf1d3e; _ga_EFC8B2CDNB=GS1.1.1679717912.2.0.1679718014.0.0.0; username-localhost-8888="2|1:0|10:1682664751|23:username-localhost-8888|44:ODJmNGUwNTEzOGE1NGYwMDgwOGZiMGFlYmUzZGI5N2Y=|07cf7de31aa2f8fd435b46d9fb76ffffb6f3412857496fafe4c68f426f46ea91"; username-localhost-8889="2|1:0|10:1683263705|23:username-localhost-8889|44:Y2I5MmUzYTA4MWY1NGVlMjg2OWE2ODE5YjZmYzE0NmQ=|b74a99392dc758c4b95264fe1602ceaf0161e6ac5291eeafd830d10cb096b7bb"; Webstorm-3497356c=5cdf5472-3f5b-4526-b4c7-4c705ce4d8e6"#;
        let req = Req::new(mock_req);

        let routes = temp_app.get_route(&req);

        assert_eq!(routes.len(), 1);
    }
}
