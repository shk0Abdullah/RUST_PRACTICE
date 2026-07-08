fn create_string() {
    let s: String = String::from("Hello, Heap");
    print!("{:?}", s);
}
// As the s goes out of scope Rust will automatically deallocate remove the Hello, Heap from the
// Heap
// Rust does these things on the basis of Ownership Rules
// 1. If var goes out of scope deallocate the memory
// 2. there would be one owner of the allocated memory at a time
// 3. memory should be referenced to an owner var
fn main() {
    create_string()
}
// Here Rule no 1 implies as the s goes out of scope Rust deallocates
// Not Similar to Garbage Collector which runs itself time-to-time to see is there any mem which
// is not being pointing by any var remove it
// Rust sees as the owner goes out of scope deallocate it
