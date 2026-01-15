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
    println!("Enter a Number: ");
    let num = read_integer();
    let result = num << 1;
    println!("The result of multiplying {} by 2 using left shift operator is {}", num, result);
}