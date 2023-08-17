use crate::route::RouteCallback;
use std::ptr::addr_of_mut;

pub trait RouteHandler {
    fn get(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self;

    fn post(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self;

    fn put(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self;

    fn delete(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self;

    fn patch(&mut self, path: &'static str, handler: RouteCallback) -> &mut Self;
}
