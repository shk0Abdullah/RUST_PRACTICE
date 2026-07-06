fn main() {
    println!("Fibonanci of {}", fib(5)) // 0,1,1,2,3,5
}

// fib(2) = 0,1,1,2
// returns at the index of fibonanci(num)
fn fib(num:u32) -> u32{
    let mut first = 0;
    let mut second = 1;

    if num == 0{
        return first
    }
    if num == 1{
        return second
    }

    for _ in 1..(num){
        let temp = second;
        second = second + first;
        first = temp
    }
    return second
}