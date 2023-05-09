use restly::app::App;
use restly::router::RouteHandler;

fn main() {
    let port = 3000;
    let mut app = App::new();
    app.listen(port, || {
        println!("Bind at {port} port")
    });

    app.get("/", || {
        println!("Handled / path")
    });
}
