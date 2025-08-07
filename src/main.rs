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

fn get_input(unit: &str) -> String {
    print!("Enter temperature in {unit}: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let input = input.trim();

    return input.to_string();
}

fn to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn to_fahrenheit(celcius: f32) -> f32 {
    (celcius * (9.0 / 5.0)) + 32.0 
}