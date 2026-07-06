struct User {
    name: String,
    male: bool,
    age: i32,
}
static mut auth: bool = false;
impl User {
    fn authenticate(&self) -> bool {
        unsafe {
            auth = true;
            return auth;
        }
    }
    fn is_authenticated() -> bool {
        unsafe { auth }
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
