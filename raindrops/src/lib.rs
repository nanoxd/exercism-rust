pub fn raindrops(num: i32) -> String {
    let mut string = String::new();

    if num % 3 == 0 {
        string.push_str("Pling");
    }

    if num % 5 == 0 {
        string.push_str("Plang");
    }

    if num % 7 == 0 {
        string.push_str("Plong");
    }

    if string.is_empty() {
        return format!("{}", num);
    }

    string
}
