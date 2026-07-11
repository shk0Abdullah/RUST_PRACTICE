fn main() {
    let no = greater_of(1, 2);
    let word = greater_of("A", "a");
    println!("{}", no);
    print!("{}", word);
}
fn greater_of<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}
