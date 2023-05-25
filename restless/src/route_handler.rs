pub trait RouteHandler {
    fn get<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn();
    fn post<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn();
    fn put<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn();
    fn delete<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn();
    fn patch<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn();
}
