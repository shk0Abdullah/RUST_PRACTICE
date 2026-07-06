fn main() {
    let string: String = String::from("ok");
    println!("Len is: {}", get_str_len(&string));
}

fn get_str_len(string: &String) -> usize {
    return string.len();
}
