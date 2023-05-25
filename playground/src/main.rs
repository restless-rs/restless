use restless::app::App;

fn main() {
    let port = 3000;
    let app = App::new();

    app.listen(port, || println!("Bind at {port} port"));
}
