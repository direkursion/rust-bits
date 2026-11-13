use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    println!("=== RUST BITS ===");
    print!("Enter a number (0-255): ");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u8 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Please enter a valid number (0-255).");
            return;
        }
    };

    println!("Decimal: {}", number);
    println!("Binary: {:08b}", number);
    println!("Octal: {:o}", number);
    println!("Hex: {:X}", number);
}
