fn main() {
    let number: u8 = 42;

    println!("=== RUST BITS ===");
    println!("Decimal: {}", number);
    println!("Binary: {:08b}", number);
    println!("Octal: {:o}", number);
    println!("Hex: {:X}", number);
}
