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
            return None;
        }
    };

    Some(number)
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

    let shift_amount: u8 = match read_number("Enter shift amount (0-7): ") {
        Some(value) => value,
        None => {
            println!("Invalid shift amount");
            return;
        }
    };

    if shift_amount >= 8 {
        println!("Invalid shift amount");
        return;
    }

    let and_result = a & b;
    let or_result = a | b;
    let xor_result = a ^ b;
    let not_a = !a;
    let not_b = !b;
    let left_shift = a << shift_amount;
    let right_shift = a >> shift_amount;

    println!();

    println!("A: {:08b} ({})", a, a);
    println!("B: {:08b} ({})", b, b);

    println!();

    println!("AND: {:08b} ({})", and_result, and_result);
    println!("OR: {:08b} ({})", or_result, or_result);
    println!("XOR: {:08b} ({})", xor_result, xor_result);

    println!();

    println!("NOT A: {:08b} ({})", not_a, not_a);
    println!("NOT B: {:08b} ({})", not_b, not_b);

    println!();

    println!("A << {shift_amount}: {:08b} ({})", left_shift, left_shift);
    println!("A >> {shift_amount}: {:08b} ({})", right_shift, right_shift);
}
