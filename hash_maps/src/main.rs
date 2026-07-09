use std::collections::HashMap;

// fn main() {
//     let mut hm = HashMap::new();
//     hm.insert(String::from("Abdullah"), 20);
//     print!("{:?}", hm.get("Abdullah"));
//     // hm.get(k)-> Options<i32>
//     // You can use match pattern
//     // hm.remove("Abdullah");
//     hm.clear();
//     hm.insert(String::from("Ali"), 21);
//     match hm.get("Abdullah") {
//         Some(val) => print!("{}", val),
//         None => print!("Abdullah is not there take Ali: {:?}", hm.get("Ali")),
//     }
// }

// Assignment

fn main() {
    let vec: Vec<(String, i32)> = vec![(String::from("Abdullah"), 20), (String::from("Ali"), 30)];
    let hm = vec_to_hash_maps(vec);
    print!("{:?}", hm.get("Abdullah"));
}

fn vec_to_hash_maps(x: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut new_hm = HashMap::new();
    for (name, age) in x {
        new_hm.insert(name, age);
    }
    return new_hm;
}
