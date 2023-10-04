use restless::app::App;
use restless::route_handler::RouteHandler;

fn main() {
    let port = 3000;
    let app = App::new();

    app.get("/", |_, mut res| {
        res.set("content-type", "application/json");
        res.send("Some")
    });

    app.listen(port, || println!("Bind at {port} port"));
}
