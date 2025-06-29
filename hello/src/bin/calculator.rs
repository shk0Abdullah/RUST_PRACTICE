
fn main(){
add(2,3);
sub(add(2,0),4)
}
fn add(num1:i8,num2:i8 )->i8{
    println!("Sum is {}", num1+num2);
    return num1+num2;
}
fn sub(num1:i8, num2:i8) {
    println!("Subtraction is {}", num1-num2);
    
}