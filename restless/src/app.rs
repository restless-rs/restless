use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use once_cell::sync::Lazy;

use crate::requrest::Req;
use crate::route::Route;
use crate::route_handler::RouteHandler;

const BASE_ADDR: &str = "127.0.0.1";

pub struct App<'a> {
    pub routes: Vec<Route<'a>>,
}

static mut APP: Lazy<App<'static>> = Lazy::new(|| App { routes: vec![] });

impl App<'static> {
    pub fn new() -> &'static Lazy<App<'static>> {
        unsafe { &APP }
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

        let req = Req::new(&raw_req);

        println!("Handled stream at {}", addr);
        // TODO: Parse stream
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
