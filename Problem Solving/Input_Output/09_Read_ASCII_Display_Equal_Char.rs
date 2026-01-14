use std::io;
fn read_ascii() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number!")
}
fn main(){
    println!("ASCII to Character Converter");
    println!("You must know  0 to 31 and 127 is not printable characters.");
    println!("--------------------------------------------------------------");
    println!("Enter an ASCII value(0-127):");
    let ascii_value = read_ascii();
    if ascii_value > 127 {
        println!("Please enter a valid ASCII value between 0 and 127.");
        return;
    }
    else {
        let character = ascii_value as char;
        println!("The character for ASCII value {} : '{}'", ascii_value, character);
    }
}