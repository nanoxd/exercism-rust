pub fn reply(msg: &str) -> String {
    let is_shouting = |s: &str| s.to_uppercase() == s && !s.is_empty();
    let is_question = |s: &str| s.ends_with("?");

    match msg {
        s if is_shouting(s) => "Whoa, chill out!".into(),
        s if is_question(s) => "Sure.".into(),
        s if s.is_empty() => "Fine. Be that way!".into(),
        _ => "Whatever.".into()
    }
}
