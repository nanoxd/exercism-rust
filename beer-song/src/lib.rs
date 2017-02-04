pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".into(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".into(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".into(),
        _ => format!("{x} bottles of beer on the wall, {x} bottles of beer.\nTake one down and pass it around, {y} bottles of beer on the wall.\n", x = n, y = n - 1 )
    }
}

pub fn sing(from: u8, to: u8) -> String {
    "".into()
}
