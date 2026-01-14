use std::io;
fn read_character() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().chars().next().unwrap()
}
fn main(){
    println!("Enter a lowercase character:");
    let character = read_character();
    println!("You entered the uppercase character: {}", character.to_ascii_uppercase());

    println!("Enter another lowercase character:");
    let another_character = read_character();
    println!("You entered the another uppercase character: {}", another_character.to_ascii_uppercase());
}