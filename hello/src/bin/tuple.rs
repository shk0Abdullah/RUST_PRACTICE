fn main() {
    let emp_info: (&str, u8) = ("taiyyar", 50);
    // Destructuring
    let (name, age) = emp_info;
    println!("{} is of age : {}", name, age);
    println!("{}", emp_info.0)
}
