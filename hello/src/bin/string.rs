fn main() {
    let mut string: String = String::from("This is a Dynamic length of string");
    println!("{}", string);
    string.push_str(" I am here again!");
    println!("{}", string);
}
