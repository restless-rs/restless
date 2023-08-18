use crate::request::Req;
use crate::response::Res;
use derivative::Derivative;
use futures::future::BoxFuture;
use std::borrow::Borrow;
use std::task::{Context, Poll};

#[derive(Debug)]
pub enum PathItemType {
    Static,
    Dynamic,
}

#[derive(Debug)]
pub struct PathItem<'a> {
    pub r#type: PathItemType,
    pub value: &'a str,
}

impl PathItem<'_> {
    pub fn new(value: &str, r#type: PathItemType) -> PathItem {
        PathItem { value, r#type }
    }
}

pub type RouteCallback = fn(Req, Res) -> Res;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Route<'a> {
    pub paths: Vec<PathItem<'a>>,
    pub method: Option<&'a str>,
    #[derivative(Debug = "ignore")]
    pub callback: RouteCallback,
}

impl Route<'_> {
    pub fn new<'a>(path: &'a str, callback: RouteCallback, method: Option<&'a str>) -> Route<'a> {
        Route {
            paths: Route::parse_path(path),
            method,
            callback,
        }
    }

    fn parse_path(path: &str) -> Vec<PathItem> {
        if !path.starts_with('/') {
            panic!("Path {} should starts with /", path)
        };

        path.split('/')
            .map(|path_part| {
                let path_type = if path_part.starts_with(':') {
                    PathItemType::Dynamic
                } else {
                    PathItemType::Static
                };

                PathItem::new(path_part, path_type)
            })
            .collect()
    }
}

