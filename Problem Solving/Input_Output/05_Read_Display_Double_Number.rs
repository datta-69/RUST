use std::io;
fn read_double_number() ->f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
fn main(){
    println!("Enter a double number: ");
    let double_num = read_double_number();
    println!("You entered the double number: {}", double_num);

    println!("Enter another double number: ");
    let another_double_num = read_double_number();
    println!("You entered the another double number: {}", another_double_num);
}