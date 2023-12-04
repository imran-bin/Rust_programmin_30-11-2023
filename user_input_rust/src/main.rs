use std::io;

fn main() {
    println!("Enter the first number:");

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    // Parse the first number
    let number1: f64 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    println!("Enter the second number:");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // Parse the second number
    let number2: f64 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // Perform addition
    let sum = number1 + number2;

    println!("The sum of {} and {} is: {}", number1, number2, sum);
}
