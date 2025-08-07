use std::io::{self, Read, Write};

fn main() {
    println!("Welcome to temperature converter");
    println!(
        "
    1) Celcius to Fahrenheit
    2) Fahrenheit to Celcius
    "
    );
    print!("Choose the option which you want: ");
    let _ = io::stdout().flush();

    let mut operation_mode = String::new();
    let _ = io::stdin().read_line(&mut operation_mode);
    let operation_mode = operation_mode.trim();

    match operation_mode {
        "1" => {
            let celcius = get_input("celcius");
            let celcius: f32 = celcius
                .parse()
                .expect("Error converting user input to number.");
            let fahrenheit = to_fahrenheit(celcius);

            println!("{celcius:.2}°C is {fahrenheit:.2}°F");
        }
        "2" => {
            let fahrenheit = get_input("fahrenheit");
            let fahrenheit: f32 = fahrenheit
                .parse()
                .expect("Error converting user input to number.");
            let celcius = to_celcius(fahrenheit);

            println!("{fahrenheit:.2}°F is {celcius:.2}°C");
        }
        _ => println!("Invalid input '{operation_mode}'!"),
    }

    wait_for_keypress();
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

fn wait_for_keypress() {
    println!("\nPress any key to exit...");
    let _ = io::stdout().flush();

    let _ = io::stdin().read(&mut [0u8]).unwrap();
}