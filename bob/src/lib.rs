pub fn reply(msg: &str) -> String {
    let is_shouting = |s: &str| s.to_uppercase() == s;

    match msg {
        s if is_shouting(s) => "Whoa, chill out!".into(),
        _ => "Whatever.".into()
    }
}
