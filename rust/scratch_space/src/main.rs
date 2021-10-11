fn main() {
    let some_string = "food";
    let i: usize = 3;
    let byte_string = some_string.as_bytes();
    let result = String::from_utf8(byte_string[1..2].to_vec()).ok().unwrap();
    println!("{}", result);
}