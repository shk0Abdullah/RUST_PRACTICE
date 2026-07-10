fn consuming_adaptor_iterator() {
    let old_vec = vec![1, 2, 3, 4, 5];
    let vec = old_vec.iter();
    print!("{:?}", vec.sum::<i32>());
    //print!("{:?}", vec); //This line throws error iterator got consumed by the Consuming Adaptors
}
fn main() {
    // consuming_adaptor_iterator();
    iterator_adaptor();
}
// they don't consume the iterator they return another iterator
fn iterator_adaptor() {
    let old_vec = vec![1, 2, 3, 4, 5];
    let vec1 = old_vec.iter();
    let vec = old_vec.iter();
    // You have to collect the mapped iterator

    // Vec<_> -> means its a vector figure out their inner elements type
    let mapped_new_vec: Vec<_> = vec.map(|x| x + 1).collect();
    // [2,3,4,5,6]
    let filtered_vec: Vec<_> = vec1.filter(|x| *x % 2 == 0).collect();
    println!("{:?}", filtered_vec);

    print!("{:?}", mapped_new_vec);
}
