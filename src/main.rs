use std::io::{self, Write};

fn main() {
    let mut number1 = String::new();
    let mut number2 = String::new();

    println!("=== RUST BITS ===");

    print!("Enter first number (0-255): ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut number1)
        .expect("Failed to read input");

    print!("Enter second number (0-255): ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut number2)
        .expect("Failed to read input");

    let a: u8 = match number1.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Please enter a valid number (0-255).");
            return;
        }
    };

    let b: u8 = match number2.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Please enter a valid number (0-255).");
            return;
        }
    };

    let result = a & b;

    println!("A: {:08b} ({})", a, a);
    println!("B: {:08b} ({})", b, b);
    println!("AND: {:08b} ({})", result, result);
}
