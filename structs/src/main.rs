struct User {
    name: String,
    male: bool,
    age: i32,
}
static mut AUTH: bool = false;
impl User {
    fn authenticate(&self) -> bool {
        unsafe {
            AUTH = true;
            return AUTH;
        }
    }
    fn is_authenticated() -> bool {
        unsafe { AUTH }
    }
}
fn main() {
    let user = User {
        name: String::from("Abdullah"),
        male: true,
        age: 32,
    };
    println!("{}", user.name);
    print!("{}", user.age);
    println!("{}", user.male);
    // _ = user.authenticate();
    print!("is_authenticated: {}", User::is_authenticated());
}
