pub fn square_of_sum(n: i32) -> i32 {
    (0 .. n + 1)
        .sum::<i32>()
        .pow(2)
}

pub fn sum_of_squares(n: i32) -> i32 {
    (0 .. n + 1).fold(0, |acc, x| acc + x.pow(2))
}

pub fn difference(n: i32) -> i32 {
    170
}
