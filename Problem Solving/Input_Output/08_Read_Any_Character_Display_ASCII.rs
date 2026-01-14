use std::io;
fn read_character() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().chars().next().unwrap()
}
fn main(){
    println!("Enter a character:");
    let character = read_character();
    println!("You entered the character: {} \n ASCII value: {}", character, character as u8);

    println!("Enter another character:");
    let another_character = read_character();
    println!("You entered the another character: {} \n ASCII value: {}", another_character, another_character as u8);
}