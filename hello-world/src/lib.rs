pub fn hello(text: Option<&str>) -> String {
    text.map_or("Hello, World!".into(), |t| format!("Hello, {}!", t))
}
