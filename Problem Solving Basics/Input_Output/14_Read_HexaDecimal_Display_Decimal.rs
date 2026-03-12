use std::io;
fn read_hexadecimal() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let hexadecimal = input.trim();

    match u32::from_str_radix(hexadecimal, 16) {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid hexadecimal number (allowed digits: 0-9, A-F, a-f)");
            std::process::exit(1);
        }
    }
}

fn main() {
    println!("Enter a hexadecimal number:");

    let decimal = read_hexadecimal();

    println!("Equivalent decimal value: {}", decimal)
}
