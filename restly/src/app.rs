use crate::router::RouterTrait;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

use crate::route::Route;

pub struct App<'a> {
    handlers: Vec<Route<'a>>,
}

const BASE_ADDR: &str = "127.0.0.1";

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App { handlers: vec![] }
    }

    #[tokio::main]
    pub async fn listen<F>(&mut self, port: &str, on_binded: F)
    where
        F: FnOnce(),
    {
        // TODO: Create `build_addr` function
        let addr = format!("{}:{}", BASE_ADDR.to_owned(), port);

        let listener = TcpListener::bind(addr.clone())
            .await
            .expect(format!("Can't bound at {}", addr).as_str());

        on_binded();

        loop {
            // TODO: Benchmark stream handling
            // NOTE: This can cause high latency because we awaiting result of
            // connection in main loop with `.await`
            let result = listener.accept().await;
            //                            ^^^^^^
            tokio::spawn(async move {
                match result {
                    Ok((stream, addr)) => App::handle_stream(stream, addr).await,
                    Err(err) => {
                        println!("Couldn't get client: {:?}", err);
                    }
                }
            })
            .await
            .unwrap();
        }
    }

    async fn handle_stream(_stream: TcpStream, addr: SocketAddr) {
        println!("Handled stream at {}", addr);
        // TODO: Parse stream
    }
}

impl RouterTrait for App<'_> {
    fn get<F>(path: &str, handler: F)
    where
        F: Fn(),
    {
        todo!()
    }

    fn post<F>(path: &str, handler: F)
    where
        F: Fn(),
    {
        todo!()
    }

    fn put<F>(path: &str, handler: F)
    where
        F: Fn(),
    {
        todo!()
    }

    fn delete<F>(path: &str, handler: F)
    where
        F: Fn(),
    {
        todo!()
    }

    fn patch<F>(path: &str, handler: F)
    where
        F: Fn(),
    {
        todo!()
    }
}
