// fn main() {
//     let s1 = String::from("Hello");
//     do_something(&s1);
//     // print!("I am from main: {}", s1)
// }
// fn do_something(s2: &String) {
//     print!("{}", s2);
// }

// Borrowing is simple lend your variable as the function ends it will return the lended memory space
// You were actually passing the reference & of that string
// And this * is used to dereference it but print! automatically does it for you
// i.e: print("{}",*s2)

/* MUTABLE BORROW */

fn main() {
    let mut s1 = String::from("Hello");
    do_something(&mut s1);
    print!("I am from main: {}", s1)
}
fn do_something(s2: &mut String) -> &mut String {
    print!("{}", s2);
    *s2 = String::from("Bye");
    return s2;
}
