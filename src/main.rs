use std::io::{self, Write};

fn read_number(prompt: &str) -> Option<u8> {
    let mut input = String::new();

    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u8 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Please enter a valid number (0-255).");
            return None;
        }
    };

    return Some(number);
}

fn main() {
    println!("=== RUST BITS ===");

    let a: u8 = match read_number("Enter first number (0-255): ") {
        Some(value) => value,
        None => {
            println!("Invalid first number");
            return;
        }
    };

    let b: u8 = match read_number("Enter second number (0-255): ") {
        Some(value) => value,
        None => {
            println!("Invalid second number");
            return;
        }
    };

    let result = a & b;

    println!("A: {:08b} ({})", a, a);
    println!("B: {:08b} ({})", b, b);
    println!("AND: {:08b} ({})", result, result);
}
