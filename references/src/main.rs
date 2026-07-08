// RULES OF REFERENCES
// 1. You can either have one mutable var ref or any number of immutable var ref
// 2. Ref must always be valid

// To prevent collision and to assure the safety

fn main() {
    let s1: String = String::from("Heloo");
    // let s0 = &mut s1 -> Error
    let s2 = &s1;
    let s3 = &s1;
    print!("{}, {}, {}", s1.to_lowercase(), s2.len(), s3.to_uppercase());
}
