pub trait RouterTrait {
    fn get<F>(path: &str, handler: F) where F: Fn();
    fn post<F>(path: &str, handler: F) where F: Fn();
    fn put<F>(path: &str, handler: F) where F: Fn();
    fn delete<F>(path: &str, handler: F) where F: Fn();
    fn patch<F>(path: &str, handler: F) where F: Fn();
}