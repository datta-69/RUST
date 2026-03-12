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
    let num = read_number();
    let result = num >> 1;
    println!("The result of dividing {} by 2 using right shift operator is {}", num, result);
}