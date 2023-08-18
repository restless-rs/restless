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

static mut APP: Lazy<App<'static>> = Lazy::new(|| App {
    routes: vec![Route::new(
        "/404",
        |req, mut res| res.status(404).send("Not found"),
        Some("GET"),
    )],
});

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

        let route = self.get_route(&req);

        match route {
            Some(r) => {
                let mut out = (r.callback)(req, res);

                Res::send_outcome(out, write_half).await;
            }
            None => {
                let not_found = self
                    .routes
                    .iter()
                    .find(|r| r.paths[1].value == "404")
                    .unwrap();
                let mut out = (not_found.callback)(req, res);

                Res::send_outcome(out, write_half).await;
            }
        }
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
        let mut res_route = None;
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
        self.routes.push(Route::new(path, handler, Some("GET")));
        self
    }

    fn post(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes.push(Route::new(path, handler, Some("POST")));
        self
    }

    fn put(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes.push(Route::new(path, handler, Some("PUT")));
        self
    }

    fn delete(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes.push(Route::new(path, handler, Some("DELETE")));
        self
    }

    fn patch(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self {
        self.routes.push(Route::new(path, handler, Some("PATCH")));
        self
    }
}

