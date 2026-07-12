// fn longest(a: String, b: String) -> String {
//     if a > b {
//         return a;
//     } else {
//         return b;
//     }
// }
// fn main() {
//     let longer;
//     let a = String::from("A");
//     // variables scope dertermine by the block of the code but the operations performed inside the block
//     // over the global variables remains
//     {
//         let b = String::from("a");
//         longer = longest(a, b);
//     }
//     print!("{}", longer);
// }

// it will compile as long as you were tranfering the ownership but if you try to pass ref

// fn longest(a: &str, b: &str) -> &str {
//     if a > b {
//         return a;
//     } else {
//         return b;
//     }
// }
// fn main() {
//     let longer;
//     let a = String::from("A");
//     {
//         let b = String::from("a");
//         longer = longest(&a, &b);
//     }
//     print!("{}", longer);
// }
// now this above lines will throws the error if b is long and the longest is also returning a reference
// of b then longer will point to an empty memory location and hence it becomes the dangling pointer
// so the solution is to define the Lifetime of there

// SOLUTION:
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b {
        return a;
    } else {
        return b;
    }
}
fn main() {
    let longer;
    let a = String::from("A");
    {
        let b = String::from("a");
        longer = longest(&a, &b);
        // This alos throws an error and say struct user will be valid only if the name is valid 
        // so we have to define the struct user and name b Lifetime relation
        // let user = User{
        //     name: &b
        // }
        let user = User<'a>{
            name &'a b
        }
        // This will throw much better error saying name b is not lived long enough so don't use it 
        // so the user.name will not point to a dangling pointer
    }
    print!("{}", longer);
    print!("{}",user.name )
}
// Now it throws much better error will say a and b will live as long as they both are valid
// means inside the block and so the return value will also be valid for the same life span
// This is what we called Lifetime

// STRUCT WITH LIFETIMES:

struct User {
    naem: &str,
}
