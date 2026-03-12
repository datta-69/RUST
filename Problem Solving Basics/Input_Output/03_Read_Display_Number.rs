use std::io;

fn read_integer() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_float() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    println!("Enter first number:");
    let n1 = read_integer();
    println!("Enter second number:");
    let n2 = read_integer();
    
    println!("You entered: {} and {}", n1, n2);

    println!("Enter a floating-point number:");
    let f1 = read_float();
    println!("You entered the floating-point number: {}", f1);
}
