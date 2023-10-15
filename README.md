# Resless

Express-inspired rust web framework.

## Examples

Minimal http server setup

```rust
use restless::{app::App, route_handler::RouteHandler};

fn main() {
    let port = 8080;
    let app = App::new();

    app.get("/", |_, mut res| {
        res.set("Content-Type", "text/plain");
        res.status(200).send("Hello world!")
    });

    app.listen(port, || println!("[info]: Started HTTP server at {port}"));
}
```

Accessing request fields

```rust
use restless::{app::App, route_handler::RouteHandler};

fn main() {
    let port = 8069;
    let app = App::new();

    app.get("/", |req, mut res| {
        // NOTE: For more details checkout 'src/request.rs'
        println!("req.body={:?}", req.body);

        res.set("Content-Type", "text/plain");
        res.status(200).send("Goodbye world!")
    });

    app.listen(port, || println!("[info]: Started HTTP server at {port}"));
}

```
