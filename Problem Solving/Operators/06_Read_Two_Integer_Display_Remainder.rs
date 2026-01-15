use std::io;
fn read_integer() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid integer input");
            std::process::exit(1);
        }
    }
}


fn main(){
    println!("Enter the first integer:");
    let num1 = read_integer();

    println!("Enter the second integer:");
    let num2 = read_integer();
    
    if num2 == 0 {
        println!("Error: Division by zero is not allowed.");
        std::process::exit(1);
    }

    let remainder = num1 % num2;
    println!("The remainder of {} divided by {} is {}", num1, num2, remainder);
}