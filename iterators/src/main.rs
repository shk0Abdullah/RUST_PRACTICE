// fn main() {
//     let vec = vec![1, 2, 3, 4, 5];
//     for val in vec.iter() {
//         println!("{:?}", val);
//     }
// }

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let vec = vec.iter_mut();
    for val in vec {
        *val = *val - 1;
        println!("{:?}", val);
    }
}

// iterator borrows the values from the collection that's why we were doing derefrencing
