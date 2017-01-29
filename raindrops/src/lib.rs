pub fn raindrops(num: i32) -> String {
    // - If the number has 3 as a factor, output 'Pling'.
    // - If the number has 5 as a factor, output 'Plang'.
    // - If the number has 7 as a factor, output 'Plong'.
    let factor_of_three = |x: i32| x % 3 == 0;

    let text = match num {
        x if factor_of_three(x) => "Pling".into(),
        _ => format!("{}", num)
    };

    text
}
