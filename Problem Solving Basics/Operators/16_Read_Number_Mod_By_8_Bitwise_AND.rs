use std::io;

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid unsigned integer input");
            std::process::exit(1);
        }
    }
}

fn main(){
    println!("Enter a Number: ");
    let number = read_number();
    let remainder = number & 7; // Bitwise AND with 7 (binary 111)
    println!("The remainder when {} is divided by 8 is: {}", number, remainder);
}
