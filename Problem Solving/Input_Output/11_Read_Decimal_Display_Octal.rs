use std::io;
fn read_decimal() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<u32>().expect("Please enter a valid decimal number")
}


fn main(){
// note: the only appropriate formatting traits are:
//            - ``, which uses the `Display` trait
//            - `?`, which uses the `Debug` trait
//            - `e`, which uses the `LowerExp` trait
//            - `E`, which uses the `UpperExp` trait
//            - `o`, which uses the `Octal` trait
//            - `p`, which uses the `Pointer` trait
//            - `b`, which uses the `Binary` trait
//            - `x`, which uses the `LowerHex` trait
//            - `X`, which uses the `UpperHex` trait

    println!("Please enter a decimal number:");
    let decimal_number = read_decimal();
    let octal_number = format!("{:o}", decimal_number);
    println!("The octal representation is: {}", octal_number);
}