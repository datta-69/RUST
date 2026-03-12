use std::io;
fn read_octal() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let octal = input.trim();

    match u32::from_str_radix(octal, 8) {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid octal number (allowed digits: 0-7)");
            std::process::exit(1);
        }
    }
}

fn main() {
    println!("Enter an octal number:");

    let decimal = read_octal();

    println!("Equivalent decimal value: {}", decimal)
}
