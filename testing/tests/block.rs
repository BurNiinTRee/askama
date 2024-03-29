use askama::Template;

#[derive(Template)]
#[template(path = "block.html")]
struct Outer<'a> {
    outer: &'a str,
    inner: &'a str,
}

#[derive(Template)]
#[template(path = "block.html", block = "inner")]
struct Inner<'a> {
    inner: &'a str,
}

#[test]
fn block() {
    assert_eq!(&Inner { inner: "world" }.render().unwrap(), "inner world");
}
