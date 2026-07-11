// DEFAULT IMPLEMENTATION
pub trait Summary {
    fn summarize(&self) -> String {
        return format!("Hello Unknown person");
    }
}

// pub trait Summary {
//     fn summarize(&self) -> String{};
// }

struct User {
    name: String,
    age: u32,
}

struct Fix {}

impl Summary for String {}
// OVERRIDE IF DEFAULT IMPLEMENTATION
impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Abdullah"),
        age: 23,
    };
    let user1 = Fix {};
    // println!("{}", user.summarize());

    // println!("{}", notify(user1));
    // This will throw error as Summary is not implemented for Fix
    println!("For user: {}", notify(user));
    println!("For Strings: {}", notify(String::from("Hello")));
}

fn notify(_x: impl Summary) -> String {
    return format!("Notified");
}
// You can use traits as the Generics as well <T: Summary + Another_Summary>()
