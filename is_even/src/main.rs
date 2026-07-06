// WITH REFERENCE
fn main() {
    println!("Hello, world! No is {}", is_even(7));
    // println!("Hello, world! No is {}", is_even(&7));

}

// fn is_even(n: &i32) -> &str{

fn is_even(n: i32) -> &'static str{
    if n%2==0{
        return "Even"
    }
    return "Odd"

}

// Without Reference
// fn main(){
//     println!("No is Even: {}", is_even(2))
// }

// fn is_even(n:i32) -> bool{
//     if n%2==0{
//         return true
//     }
//     return false
// }