use std::io;
fn read_number() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<u32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid unsigned integer input");
            std::process::exit(1);
        }
    }
}

fn main(){
    println!("Enter the first number:");
    let num1 = read_number();

    println!("Enter the second number:");
    let num2 = read_number();
    let bitwise_xor = num1 ^ num2;
    println!("The bitwise XOR of {} and {} is {}", num1, num2, bitwise_xor);
}