enum PathItemType {
    Static,
    Dynamic,
}

struct PathItem<'a> {
    r#type: PathItemType,
    value: &'a str,
}

pub struct Route<'a> {
    paths: Vec<PathItem<'a>>,
    handler: Box<dyn Fn()>,
}
