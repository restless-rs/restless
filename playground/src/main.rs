use restly::app::App;


fn main() {
    let mut app = App::new();
    app.listen("3000", || {
        println!("Bind at 3000 port")
    });
}

