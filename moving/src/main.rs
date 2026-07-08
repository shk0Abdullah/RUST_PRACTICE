fn main() {
    let s1 = String::from("Heloo");
    let s2 = s1;
    // owner moved from s1 to s2 and s1 gets invalid
    // when you were gonna pass as the arg it becomes invalid too.
    moving(s2); // throws error and says to pass the clone
                // if you execute the line under i have
                // print!("{}", s2);
}
fn moving(s2: String) {
    another_moving(s2.clone());
    // Will ask to send the clone but if we don't have the below print won't complain and simply transfer the ownership
    // to the next variable
    print!("{}", s2);
}

fn another_moving(s3: String) {
    print!("{}", s3);
}
