pub fn hello(text: Option<&str>) -> String {
    let thing = match text {
        None => "World",
        Some(thing) => thing,
    };

    format!("Hello, {}!", thing)
}
