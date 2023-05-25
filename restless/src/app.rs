use std::collections::HashMap;
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::requrest::Req;
use crate::route::Route;
use crate::route_handler::RouteHandler;

#[allow(dead_code)]
struct AppContext<'a> {
    pub routes: Vec<Route<'a>>,
}

impl<'a> AppContext<'a> {
    pub fn new() -> AppContext<'a> {
        AppContext { routes: vec![] }
    }
}

type AppContextMap<'a> = Lazy<HashMap<Uuid, AppContext<'a>>>;

// NOTE: Currently it unsafe to read and write in from threads
// TODO: Wrap in `Mutex`
static mut APP_CONTEXT_MAP: AppContextMap = Lazy::new(|| { HashMap::new() });

#[allow(dead_code)]
pub struct App {
    id: Uuid,
}

const BASE_ADDR: &str = "127.0.0.1";

impl App {
    pub fn new() -> App {
        let id = Uuid::new_v4();

        unsafe {
            APP_CONTEXT_MAP.insert(id, AppContext::new());
        }

        App { id }
    }

    // TODO: Client error handle hook on connection
    #[allow(unused_variables)]
    #[tokio::main]
    pub async fn listen<F>(self, port: u16, on_binded: F)
    where
        F: FnOnce(),
    {
        // TODO: Create `build_addr` function
        let addr = format!("{}:{}", BASE_ADDR.to_owned(), port);

        let listener = TcpListener::bind(addr.clone())
            .await
            .expect(format!("Can't bound at {}", addr).as_str());

        on_binded();

        loop /* of pain and suffer */ {
            let result = listener.accept().await;

            let context = (|| -> &mut AppContext {
                unsafe {
                    let result = APP_CONTEXT_MAP.get_mut(&self.id);
                    return result.unwrap();
                }
            })();

            tokio::spawn(async move {
                match result {
                    Ok((stream, addr)) => handle_stream(context, stream, addr).await,
                    Err(err) => println!("Couldn't get client: {:?}", err),
                }
            });

        }
    }
}

#[allow(unused_variables)]
#[allow(unused_mut)]
async fn handle_stream(context: &'static mut AppContext<'_>, mut stream: TcpStream, addr: SocketAddr) {
    let (reader, mut writer) = stream.split();

    let mut buf_reader = BufReader::new(reader);
    let mut raw_req = String::new();

    buf_reader.read_to_string(&mut raw_req).await.unwrap();

    context.routes.push(Route::new("/foo", || {}));

    let req = Req::new(&raw_req);

    println!("Handled stream at {}", addr);
    // TODO: Parse stream
}

#[allow(unused_variables)]
#[allow(unreachable_code)]
impl RouteHandler for App {
    fn get<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(),
    {
        todo!();

        self
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
