fn main() {
    let string: &str = "";
    match string {
        "Hello" => println!("Nice to meet you"),
        _ => print!("can't find any string literal"),
    }
}
