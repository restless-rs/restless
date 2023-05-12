use std::path::Path;

enum PathItemType {
    Static,
    Dynamic,
}

struct PathItem<'a> {
    r#type: PathItemType,
    value: &'a str,
}

impl PathItem<'_> {
    pub fn new(value: &str, r#type: PathItemType) -> PathItem {
        PathItem {
            value,
            r#type,
        }
    }
}

pub struct Route<'a> {
    paths: Vec<PathItem<'a>>,
    handler: fn(),
}

impl Route<'_> {
    pub fn new(path: &str, handler: fn()) -> Route {
        Route {
            paths: Route::parse_path(path),
            handler
        }
    }

    fn parse_path(path: &str) -> Vec<PathItem> {
        if !path.starts_with("/") {
            panic!("Path {} should starts with /", path)
        };

        path.split("/")
            .map(|path_part| {
                let path_type = if path_part.starts_with(":") {
                    PathItemType::Dynamic
                } else {
                    PathItemType::Static
                };

                PathItem::new(path_part, path_type)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::route::{PathItem, PathItemType, Route};

    #[test]
    fn create_route() {
        let route = Route::new("/", || {});
    }
}
