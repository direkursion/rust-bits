use std::io::{self, Write};

fn show_menu() {
    println!();
    println!("=== RUST BITS ===");
    println!();
    println!("[1] Number Converter");
    println!("[2] Bitwise Operations");
    println!("[3] Bit Inspector");
    println!("[0] Exit");
    println!();
}

fn run_number_converter() {
    println!();
    println!("=== NUMBER CONVERTER ===");
    println!();

    let a: u8 = match read_number("Enter number (0-255): ") {
        Some(value) => value,
        None => {
            println!("Invalid number");
            wait_for_enter();
            return;
        }
    };

    println!();
    println!("{:13}{}", "Decimal:", a);
    println!("{:13}{:08b}", "Binary:", a);
    println!("{:13}{:o}", "Octal:", a);
    println!("{:13}{:X}", "Hexadecimal:", a);
    println!();

    wait_for_enter();
}

fn run_bitwise_operations() {
    println!();
    println!("=== BITWISE OPERATIONS ===");
    println!();

    let a: u8 = match read_number("Enter first number (0-255): ") {
        Some(value) => value,
        None => {
            println!("Invalid first number");
            wait_for_enter();
            return;
        }
    };

    let b: u8 = match read_number("Enter second number (0-255): ") {
        Some(value) => value,
        None => {
            println!("Invalid second number");
            wait_for_enter();
            return;
        }
    };

    let shift_amount: u8 = match read_number("Enter shift amount (0-7): ") {
        Some(value) => value,
        None => {
            println!("Invalid shift amount");
            wait_for_enter();
            return;
        }
    };

    if shift_amount >= 8 {
        println!("Invalid shift amount");
        wait_for_enter();
        return;
    }

    let and_result = bitwise_and(a, b);
    let or_result = bitwise_or(a, b);
    let xor_result = bitwise_xor(a, b);

    let not_a = bitwise_not(a);
    let not_b = bitwise_not(b);

    let left_shift = shift_left(a, shift_amount);
    let right_shift = shift_right(a, shift_amount);

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
    println!("A << {}: {:08b} ({})", shift_amount, left_shift, left_shift);
    println!(
        "A >> {}: {:08b} ({})",
        shift_amount, right_shift, right_shift
    );
    println!();

    wait_for_enter();
}

fn run_bit_inspector() {
    println!();
    println!("=== BIT INSPECTOR ===");
    println!();

    let a: u8 = match read_number("Enter number (0-255): ") {
        Some(value) => value,
        None => {
            println!("Invalid number");
            wait_for_enter();
            return;
        }
    };

    let bit_position: u8 = match read_number("Enter bit position (0-7): ") {
        Some(value) => value,
        None => {
            println!("Invalid bit position");
            wait_for_enter();
            return;
        }
    };

    if bit_position >= 8 {
        println!("Invalid bit position");
        wait_for_enter();
        return;
    }

    let bit = read_bit(a, bit_position);
    let set_bit_result = set_bit(a, bit_position);
    let clear_bit_result = clear_bit(a, bit_position);
    let toggle_bit_result = toggle_bit(a, bit_position);

    println!();
    println!("Number: {:08b} ({})", a, a);
    println!();
    println!("Read bit {}: {}", bit_position, bit);
    println!("Set bit {}: {}", bit_position, set_bit_result);
    println!("Clear bit {}: {}", bit_position, clear_bit_result);
    println!("Toggle bit {}: {}", bit_position, toggle_bit_result);
    println!();

    wait_for_enter();
}

fn read_number(prompt: &str) -> Option<u8> {
    let mut input = String::new();

    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u8 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => return None,
    };

    Some(number)
}

fn wait_for_enter() {
    let mut input = String::new();

    println!();
    print!("Press Enter to return to menu...");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn bitwise_and(a: u8, b: u8) -> u8 {
    a & b
}

fn bitwise_or(a: u8, b: u8) -> u8 {
    a | b
}

fn bitwise_xor(a: u8, b: u8) -> u8 {
    a ^ b
}

fn bitwise_not(a: u8) -> u8 {
    !a
}

fn shift_left(a: u8, amount: u8) -> u8 {
    a << amount
}

fn shift_right(a: u8, amount: u8) -> u8 {
    a >> amount
}

fn read_bit(a: u8, position: u8) -> u8 {
    (a >> position) & 1
}

fn set_bit(a: u8, position: u8) -> u8 {
    a | (1 << position)
}

fn clear_bit(a: u8, position: u8) -> u8 {
    a & !(1 << position)
}

fn toggle_bit(a: u8, position: u8) -> u8 {
    a ^ (1 << position)
}

fn main() {
    loop {
        clear_screen();
        show_menu();

        let choice: u8 = match read_number("Select an option: ") {
            Some(number) => number,
            None => {
                println!("Invalid option");
                wait_for_enter();
                continue;
            }
        };

        match choice {
            1 => run_number_converter(),
            2 => run_bitwise_operations(),
            3 => run_bit_inspector(),
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option");
                wait_for_enter();
            }
        }
    }
}

#[test]
fn test_bitwise_and() {
    assert_eq!(bitwise_and(12, 10), 8);
    assert_eq!(bitwise_and(255, 0), 0);
    assert_eq!(bitwise_and(255, 255), 255);
}

#[test]
fn test_bitwise_or() {
    assert_eq!(bitwise_or(12, 10), 14);
    assert_eq!(bitwise_or(255, 0), 255);
    assert_eq!(bitwise_or(255, 255), 255);
    assert_eq!(bitwise_or(170, 85), 255);
    assert_eq!(bitwise_or(0, 0), 0);
}

#[test]
fn test_bitwise_xor() {
    assert_eq!(bitwise_xor(12, 10), 6);
    assert_eq!(bitwise_xor(255, 0), 255);
    assert_eq!(bitwise_xor(255, 255), 0);
    assert_eq!(bitwise_xor(170, 85), 255);
    assert_eq!(bitwise_xor(0, 0), 0);
}

#[test]
fn test_bitwise_not() {
    assert_eq!(bitwise_not(0), 255);
    assert_eq!(bitwise_not(12), 243);
    assert_eq!(bitwise_not(255), 0);
    assert_eq!(bitwise_not(170), 85);
}

#[test]
fn test_shift_left() {
    assert_eq!(shift_left(12, 1), 24);
    assert_eq!(shift_left(12, 2), 48);
    assert_eq!(shift_left(128, 1), 0);
    assert_eq!(shift_left(255, 1), 254);
    assert_eq!(shift_left(12, 0), 12);
}

#[test]
fn test_shift_right() {
    assert_eq!(shift_right(12, 1), 6);
    assert_eq!(shift_right(12, 2), 3);
    assert_eq!(shift_right(128, 1), 64);
    assert_eq!(shift_right(255, 1), 127);
    assert_eq!(shift_right(12, 0), 12);
}

#[test]
fn test_read_bit() {
    assert_eq!(read_bit(42, 0), 0);
    assert_eq!(read_bit(42, 1), 1);
    assert_eq!(read_bit(42, 3), 1);
    assert_eq!(read_bit(42, 5), 1);
    assert_eq!(read_bit(255, 7), 1);
    assert_eq!(read_bit(0, 3), 0);
}

#[test]
fn test_set_bit() {
    assert_eq!(set_bit(32, 2), 36);
    assert_eq!(set_bit(42, 0), 43);
    assert_eq!(set_bit(42, 1), 42);
    assert_eq!(set_bit(0, 7), 128);
    assert_eq!(set_bit(255, 3), 255);
}

#[test]
fn test_clear_bit() {
    assert_eq!(clear_bit(42, 3), 34);
    assert_eq!(clear_bit(42, 1), 40);
    assert_eq!(clear_bit(42, 0), 42);
    assert_eq!(clear_bit(255, 7), 127);
    assert_eq!(clear_bit(0, 3), 0);
}

#[test]
fn test_toggle_bit() {
    assert_eq!(toggle_bit(42, 2), 46);
    assert_eq!(toggle_bit(42, 3), 34);
    assert_eq!(toggle_bit(0, 7), 128);
    assert_eq!(toggle_bit(255, 0), 254);
    assert_eq!(toggle_bit(170, 0), 171);
    assert_eq!(toggle_bit(toggle_bit(42, 2), 2), 42);
}
