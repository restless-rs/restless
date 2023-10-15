use restless::{app::App, route_handler::RouteHandler};

fn main() {
    let port = 3000;
    let app = App::new();

    app.get("/", |req, mut res| {
        // NOTE: For more details checkout 'src/request.rs'
        println!("req.body={:?}", req.body);

        res.set("Content-Type", "text/plain");
        res.status(200).send("Hello world!")
    });

    app.listen(port, || println!("[info]: Started HTTP server at {port}"));
}
