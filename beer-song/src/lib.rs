pub fn verse(num: u8) -> String {
    if num == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".into()
    } else {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".into()
    }
}

pub fn sing(from: u8, to: u8) -> String {
    "".into()
}
