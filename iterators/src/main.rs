// fn main() {
//     let vec = vec![1, 2, 3, 4, 5];
//     for val in vec.iter() {
//         println!("{:?}", val);
//     }
// }

// fn main() {
//     let mut vec = vec![1, 2, 3, 4, 5];
//     let vec = vec.iter_mut();
//     for val in vec {
//         *val = *val - 1;
//         println!("{:?}", val);
//     }
// }

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    for val in vec {
        println!("{}", val);
        // This below line will throws the error as the ownership moves from vec to val
        // print!("{:?}", vec)
    }
    // This would throws the error as well cause the ownership moves to val and as the
    // for block ends val got deallocated as well and vec already become invalidate so we lose both at this line
    // That's why compiler saying to pass ref to sustain vec if you want to use it in future.
    print!("{:?}", vec) //->Error
}

// iterator borrows the values from the collection that's why we were doing derefrencing
