use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use once_cell::sync::Lazy;

use crate::requrest::Req;
use crate::route::PathItemType;
use crate::route::Route;
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
    #[allow(unused_variables)]
    #[tokio::main]
    pub async fn listen<F>(&'static self, port: u16, on_binded: F)
    where
        F: FnOnce(),
    {
        // TODO: Create `build_addr` function
        let addr = format!("{}:{}", BASE_ADDR.to_owned(), port);

        let listener = TcpListener::bind(addr.clone())
            .await
            .expect(format!("Can't bound at {}", addr).as_str());

        on_binded();

        loop
        /* of pain and suffer */
        {
            let result = listener.accept().await;

            tokio::spawn(async move {
                match result {
                    Ok((stream, addr)) => self.handle_stream(stream, addr).await,
                    Err(err) => println!("Couldn't get client: {:?}", err),
                }
            });
        }
    }

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    async fn handle_stream(&'static self, mut stream: TcpStream, addr: SocketAddr) {
        let (reader, mut writer) = stream.split();

        let mut buf_reader = BufReader::new(reader);
        let mut raw_req = String::new();

        buf_reader.read_to_string(&mut raw_req).await.unwrap();

        println!("{raw_req}");
        let req = Req::new(&raw_req);


        println!("Handled stream at {}", addr);
        // TODO: Parse stream
    }

    fn build_request_path<'a>(&self, req: &'a Req) -> Vec<&Route<'a>> {
        let mut request_map = Vec::new();
        let req_paths = req.path.split_terminator("/").collect::<Vec<_>>();

        for route in &self.routes {
            let mut is_compatible = true;

            if route.paths.len() != req_paths.len() {
                continue;
            }

            for i in 0..route.paths.len() {
                match route.paths[i].r#type {
                    PathItemType::Static => {
                        if route.paths[i].value != req_paths[i] {
                            is_compatible = false;
                            break;
                        }
                    }
                    PathItemType::Dynamic => (),
                }
            }

            if is_compatible {
                request_map.push(route);
            }
        }
        request_map
    }
}

#[allow(unused_variables)]
#[allow(unreachable_code)]
impl RouteHandler for App<'_> {
    fn get<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();
    }

    fn post<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }

    fn put<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }

    fn delete<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }

    fn patch<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_empty() {
        let temp_app = App::new();

        temp_app
            .routes
            .push(Route::new("/home", || println!("home")));
        temp_app
            .routes
            .push(Route::new("/login", || println!("first login")));
        temp_app
            .routes
            .push(Route::new("/login", || println!("second logout")));

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

        let routes = temp_app.build_request_path(&req);

        assert_eq!(routes.len(), 0);
    }

    #[test]
    fn test_build_req_path() {
        let mut temp_app = App::new();

        temp_app
            .routes
            .push(Route::new("/home", || println!("home")));
        temp_app
            .routes
            .push(Route::new("/login", || println!("first login")));
        temp_app
            .routes
            .push(Route::new("/login", || println!("second logout")));

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

        let routes = temp_app.build_request_path(&req);

        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn test_build_with_dynamic() {
        let mut temp_app = App::new();

        temp_app
            .routes
            .push(Route::new("/home", || println!("home")));
        temp_app
            .routes
            .push(Route::new("/:user_id/login", || println!("first login")));
        temp_app
            .routes
            .push(Route::new("/login", || println!("second logout")));

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

        let routes = temp_app.build_request_path(&req);

        assert_eq!(routes.len(), 1);
    }
}
