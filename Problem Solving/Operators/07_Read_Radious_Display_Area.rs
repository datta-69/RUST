use std::io;
fn read_radious() -> f32 {
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
    println!("Enter the radius of the circle:");
    let radius = read_radious();
    let area = std::f32::consts::PI * radius * radius;
    println!("The area of the circle with radius {} is {}", radius, area);

}