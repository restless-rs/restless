use restly::app::App;
use restly::router::RouteHandler;

fn main() {
    let port = 3000;
    let mut app = App::new();

    app.get("/", || {
        println!("Handled / path")
    }).get("/item/:u64", || {
        println!("Give some item")
    }).post("/item/name:str&value:u32", || {
        println!("Got some item")
    });

    app.listen(port, || {
        println!("Bind at {port} port")
    });
}
