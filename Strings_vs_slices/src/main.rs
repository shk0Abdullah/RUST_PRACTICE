fn warmup() {
    let word = String::from("Hello World");
    let word2 = &word[0..5];
    //word.clear(); //Rust won't let you do like this cause clear() takes mutable ref and remember
    // ownership rules can't have mut ref with the immut ref
    print!("{}", word2)
}
fn main() {
    // warmup();
    // slice_vectors()
}
fn slice_vectors() {
    let vec = vec![1, 2, 4, 5, 6];
    print!("{:?}", &vec[0..1])
}
