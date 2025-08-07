use std::io::{self, Read, Write};

fn main() {
    println!("Welcome to temperature converter");
    println!("
    1) Celcius to Fahrenheit
    2) Fahrenheit to Celcius
    ");
    print!("Choose the option which you want: ");
    let _ = io::stdout().flush();

    let mut operation_mode = String::new();
    let _ = io::stdin().read_line(&mut operation_mode);
    let operation_mode = operation_mode.trim();
}
