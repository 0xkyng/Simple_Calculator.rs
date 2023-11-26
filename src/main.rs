use std::io;

fn main() {
    println!("Simple Rust Calculator");

    // Input operation symbol
    println!("Enter an operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    // Input two numbers
    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Invalid input. Please enter a number.");

    // Perform the calculation based on the operation
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Division by zero.");
                return;
            }
        }
        _ => {
            println!("Error: Invalid operation symbol.");
            return;
        }
        // The underscore (_) is a wildcard pattern that matches any value not covered by previous patterns.
        //If operation doesn't match any of the previous patterns ("+", "-", "*", "/"), it prints an error message, "Error: Invalid operation symbol," and exits the program using return.
    };

    // Display the result
    println!("Result: {}", result);
}
