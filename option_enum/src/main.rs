// Option <i32>  or any T will return T or None its optional to return something
// Option and Result are the built in terms

fn count_first_A_string(string: String) -> Option<i32> {
    for (i, val) in string.chars().enumerate() {
        if val == 'a' {
            return Some(i as i32);
        }
    }
    return None;
}

fn main() {
    let first_count = count_first_A_string(String::from("smile"));
    match first_count {
        Some(i) => println!("First A found at index {}", i),
        None => println!("First A NOT found"),
    }
}
