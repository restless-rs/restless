use restly::app::App;

fn main() {
    let port = 3000;
    let mut app = App::new();
    app.listen(port, || {
        println!("Bind at {port} port")
    });
}
