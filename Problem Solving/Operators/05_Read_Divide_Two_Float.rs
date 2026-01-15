use std::io;
fn read_float() -> f32 {
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
    println!("Enter the first float:");
    let num1 = read_float();

    println!("Enter the second float:");
    let num2 = read_float();
    
    if num2 == 0.0 {
        println!("Error: Division by zero is not allowed.");
        std::process::exit(1);
    }

    let quotient = num1 / num2;
    println!("The quotient of {} divided by {} is {}", num1, num2, quotient);
}