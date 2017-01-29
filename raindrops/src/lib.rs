pub fn raindrops(num: i32) -> String {
    let factor_of_three = |x: i32| x % 3 == 0;
    let factor_of_five = |x: i32| x % 5 == 0;
    let factor_of_seven = |x: i32| x % 7 == 0;

    match num {
        x if factor_of_three(x) => "Pling".into(),
        x if factor_of_five(x) => "Plang".into(),
        x if factor_of_seven(x) => "Plong".into(),
        _ => format!("{}", num)
    }
}
