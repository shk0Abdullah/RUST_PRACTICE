fn main() {
    let string_1: String = String::from("Hello, RUST");
    let reference_1 = &string_1;
    let reference_2 = &string_1;
    println!("{} , {}", reference_1, reference_2);
    error_code();
}

fn error_code() {
    let mut string_1: String = String::from("Hello, RUST");
    let write_1 = &mut string_1;
    write_1.push_str("!");
    let write_2 = &mut string_1;
    write_2.push_str(" Again");
    println!("{}", write_2);
    // println!("{} , {}", write_1, write_2); // Throws an error because of write_1 its the previous state of string_1 and rust uses compiler now think
    // it throws the error for the write write operation and read write operation considering the same scenerio
    // but not for the read read operation
}
