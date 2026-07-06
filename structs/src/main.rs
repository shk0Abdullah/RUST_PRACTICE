struct User {
    name: String,
    male: bool,
    age: i32,
}
fn main() {
    let user = User {
        name: String::from("Abdullah"),
        male: true,
        age: 32,
    };
    println!("{}", user.name);
    print!("{}", user.age);
    print!("{}", user.male);
}
