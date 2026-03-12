use std::io;
fn read_name() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main(){
    println!("Enter your name:");

    let name = read_name();

    println!("Hello, {}!", name);
}