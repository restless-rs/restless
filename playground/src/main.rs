use restless::app::App;
use restless::route_handler::RouteHandler;

fn main() {
    let port = 3000;
    let app = App::new();

    app.get("/index", || {});

    app.listen(port, || println!("Bind at {port} port"));
}
