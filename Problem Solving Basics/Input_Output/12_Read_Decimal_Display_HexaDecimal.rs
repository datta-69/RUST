use std::io;
fn read_decimal () ->u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<u32>().expect("Please enter a valid decimal number")
}

fn main() {
    println!("Please enter a decimal number:");
    let decimal_number = read_decimal();
    println!("Hexadecimal: {:x}", decimal_number);
    println!("Hexadecimal : {:X}", decimal_number);
}