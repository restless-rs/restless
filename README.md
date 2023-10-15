# Resless

Express-inspired rust web framework.

## Examples

Minimal http server setup

```rust
use restless::{app::App, route_handler::RouteHandler};

fn main() {
    let port = 3000;
    let app = App::new();

    app.get("/", |_, mut res| {
        res.set("Content-Type", "text/plain");
        res.status(200).send("Hello world!")
    });

    app.listen(port, || println!("[info]: Started HTTP server at {port}"));
}
```
