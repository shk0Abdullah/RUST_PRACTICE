// fn main() {
//     let mut vec = Vec::new();
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     vec.push(5);
//     print!("{:?}", vec)
// }

// Assignment filtering the Vector

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let newVec = filter_even(&vec);
    print!("{:?}", newVec);
}

fn filter_even(vec: &Vec<i32>) -> Vec<i32> {
    let mut newVec: Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        if vec[i] % 2 == 0 {
            newVec.push(vec[i]);
        }
    }
    return newVec;
}
