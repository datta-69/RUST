
use std::io;
fn read_long_integer() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
fn main(){
    println!("Enter a long integer:");
    let long_num = read_long_integer();
    println!("You entered the long integer: {}", long_num);

    println!("Enter another long integer:");
    let another_long_num = read_long_integer();
    println!("You entered the another long integer: {}", another_long_num);
}