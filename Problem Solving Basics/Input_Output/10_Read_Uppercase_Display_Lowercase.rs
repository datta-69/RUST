use std::io;
fn read_uppercase() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().next().unwrap()
}

fn main(){
        println!("Please enter an uppercase letter:");
    let uppercase_char = read_uppercase();
    let lowercase_char = uppercase_char.to_ascii_lowercase();
    println!("The lowercase letter is: {}", lowercase_char);
}