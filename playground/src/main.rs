use restless::app::App;
use restless::request::Req;
use restless::response::Res;
use restless::route_handler::RouteHandler;
use std::future::Future;

fn main() {
    let port = 3000;
    let app = App::new();

    app.get("/index", index);

    app.listen(port, || println!("Bind at {port} port"));
}

async fn index(req: &Req<'_>, res: &mut Res<'_>) {
    res.send("some text").await
}
