use std::io;
fn read_temperature() -> f32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid float input");
            std::process::exit(1);
        }
    }
}

fn main(){
    println!("Enter the temperature in Fahrenheit:");
    let fahrenheit = read_temperature();
    let celsius = (fahrenheit - 32.0) * 5.0/9.0;
    println!("{} Fahrenheit is equal to {} Celsius", fahrenheit, celsius);
}