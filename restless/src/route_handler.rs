pub trait RouteHandler {
    fn get(&mut self, path: &'static str, handler: fn()) -> &mut Self;

    fn post(&mut self, path: &'static str, handler: fn()) -> &mut Self;

    fn put(&mut self, path: &'static str, handler: fn()) -> &mut Self;

    fn delete(&mut self, path: &'static str, handler: fn()) -> &mut Self;

    fn patch(&mut self, path: &'static str, handler: fn()) -> &mut Self;
}
