use std::io;
fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main(){
    println!("Enter a date in the format DD/MM/YYYY:");
    let date = read_line();

    let parts: Vec<&str> = date.split('/').collect();
    if parts.len() == 3 {
        let day = parts[0];
        let month = parts[1];
        let year = parts[2];

        println!("Day: {}", day);
        println!("Month: {}", month);
        println!("Year: {}", year);
    } else {
        println!("Invalid date format");
    }
}